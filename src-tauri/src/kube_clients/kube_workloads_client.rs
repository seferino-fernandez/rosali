use std::ops::Deref;
use std::str;

use crate::kube_model::cronjobs::KubeCronJob;
use crate::kube_model::daemonsets::KubeDaemonSet;
use crate::kube_model::deployments::KubeDeployment;
use crate::kube_model::event::KubeEvent;
use crate::kube_model::jobs::KubeJob;
use crate::kube_model::pods::KubePod;
use crate::kube_model::replicasets::KubeReplicaSet;
use crate::kube_model::replication_controllers::KubeReplicationController;
use crate::kube_model::statefulsets::KubeStatefulSet;
use crate::kube_model::workload_status::{
    DeploymentStatus, PodStatus, ReplicaStatus, WorkloadStatus,
};
use futures::{AsyncBufReadExt, TryStreamExt};
use k8s_openapi::api::apps::v1::{DaemonSet, Deployment, ReplicaSet, StatefulSet};
use k8s_openapi::api::batch::v1::{CronJob, Job};
use k8s_openapi::api::core::v1::{Event, Pod, ReplicationController};
use kube::api::{ListParams, LogParams};
use kube::{Api, Client};
use std::sync::{Arc, Mutex};
use tauri::Window;
use tokio::task;

pub async fn get_pods(
    client: &Client,
    namespace: Option<String>,
) -> Result<Vec<KubePod>, kube::Error> {
    let pods_api: Api<Pod> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let pods = pods_api.list(&ListParams::default()).await?;
    let kube_pods = pods.into_iter().map(KubePod::from).collect();
    Ok(kube_pods)
}

pub async fn get_workload_status(
    client: &kube::Client,
    namespace: Option<String>,
) -> Result<WorkloadStatus, Box<dyn std::error::Error>> {
    let namespace_ref = namespace.as_deref();

    let pods_api: Api<Pod> = match namespace_ref {
        Some(namespace) => Api::namespaced(client.clone(), namespace),
        None => Api::all(client.clone()),
    };
    let deployments_api: Api<Deployment> = match namespace_ref {
        Some(namespace) => Api::namespaced(client.clone(), namespace),
        None => Api::all(client.clone()),
    };
    let replica_sets_api: Api<ReplicaSet> = match namespace_ref {
        Some(namespace) => Api::namespaced(client.clone(), namespace),
        None => Api::all(client.clone()),
    };

    let lp: ListParams = ListParams::default();

    let pod_list: kube::core::ObjectList<Pod> = pods_api.list(&lp).await?;
    let deployment_list: kube::core::ObjectList<Deployment> = deployments_api.list(&lp).await?;
    let replica_set_list: kube::core::ObjectList<ReplicaSet> = replica_sets_api.list(&lp).await?;

    let mut pod_running: i32 = 0;
    let mut pod_failed: i32 = 0;
    let mut deployment_running: i32 = 0;
    let mut deployment_failed: i32 = 0;
    let mut replica_running: i32 = 0;
    let mut replica_failed: i32 = 0;

    for pod in pod_list {
        let status: &k8s_openapi::api::core::v1::PodStatus = pod.status.as_ref().unwrap();
        let phase: &str = status.phase.as_ref().unwrap().as_str();
        if phase == "Running" {
            pod_running += 1;
        } else {
            pod_failed += 1;
        }
    }

    for deployment in deployment_list {
        let spec = deployment.spec.as_ref().unwrap();
        let status = deployment.status.as_ref().unwrap();
        let replicas: i32 = spec.replicas.unwrap_or(1);
        let available_replicas: i32 = status.available_replicas.unwrap_or(0);
        let failed_replicas: i32 = replicas - available_replicas;

        deployment_running += available_replicas;
        deployment_failed += failed_replicas;
    }

    for replica_set in replica_set_list {
        let spec = replica_set.spec.as_ref().unwrap();
        let status = replica_set.status.as_ref().unwrap();
        let desired = spec.replicas.unwrap_or(0);
        let current = status.replicas;
        let ready = status.ready_replicas.unwrap_or(0);

        if desired == current && current == ready {
            replica_running += 1;
        } else if desired > 0 && (current < desired || ready < desired) {
            replica_failed += 1;
        }
    }

    Ok(WorkloadStatus {
        deployments: DeploymentStatus {
            running: deployment_running,
            failed: deployment_failed,
        },
        pods: PodStatus {
            running: pod_running,
            failed: pod_failed,
        },
        replicas: ReplicaStatus {
            running: replica_running,
            failed: replica_failed,
        },
    })
}

pub async fn get_recent_events(
    client: &kube::Client,
    namespace: Option<String>,
) -> Result<Vec<KubeEvent>, Box<dyn std::error::Error>> {
    let events_api: Api<Event> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), &namespace),
        None => Api::all(client.clone()),
    };
    let lp = ListParams::default().timeout(30);
    let events = events_api.list(&lp).await?;

    let mut recent_events: Vec<KubeEvent> = Vec::new();

    for event in events {
        let formatted_source = if let Some(source) = event.source {
            format!(
                "{} {}",
                source.host.unwrap_or_default(),
                source.component.unwrap_or_default()
            )
        } else {
            String::new()
        };

        let kube_event = KubeEvent {
            name: event.metadata.name.unwrap_or_default(),
            namespace: event.metadata.namespace,
            reason: event.reason,
            message: event.message,
            warning_or_regular: event.type_.unwrap_or_else(|| "Normal".to_string()),
            source: Some(formatted_source),
            reporting_component: event.reporting_component,
            count: event.count.unwrap_or(0),
            first_time: event
                .first_timestamp
                .map(|timestamp| timestamp.0.to_rfc3339()),
            last_time: event
                .last_timestamp
                .map(|timestamp| timestamp.0.to_rfc3339()),
        };
        recent_events.push(kube_event);
    }

    Ok(recent_events)
}

pub async fn stream_pod_logs(
    client: &Client,
    namespace: Option<String>,
    pod_name: &str,
    window: Window,
) -> Result<(), kube::Error> {
    let pods_api: Api<Pod> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), &namespace),
        None => Api::all(client.clone()),
    };

    let log_params = LogParams {
        follow: true,
        container: None,
        tail_lines: None,
        timestamps: true,
        since_seconds: None,
        limit_bytes: None,
        pretty: true,
        previous: true,
    };

    let mut log_stream = pods_api.log_stream(pod_name, &log_params).await?.lines();

    let window_arc = Arc::new(Mutex::new(window));

    task::spawn(async move {
        while let Some(line) = log_stream.try_next().await.expect("Error reading log line") {
            let window = window_arc.clone();
            window
                .lock()
                .unwrap()
                .emit("log_line", Some(&line))
                .expect("Failed to emit log line");
        }
    });

    Ok(())
}

pub async fn get_deployments(
    client: &Client,
    namespace: Option<String>,
) -> Result<Vec<KubeDeployment>, kube::Error> {
    let deployments_api: Api<Deployment> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let deployments = deployments_api.list(&ListParams::default()).await?;
    Ok(deployments.into_iter().map(KubeDeployment::from).collect())
}

pub async fn restart_deployment(
    client: &Client,
    namespace: Option<String>,
    name: &str,
) -> Result<KubeDeployment, kube::Error> {
    let namespace = namespace.unwrap_or_else(|| String::from("default"));
    let deployments_api: Api<Deployment> = Api::namespaced(client.clone(), &namespace);
    let deployment = deployments_api.restart(name).await?;
    Ok(KubeDeployment::from(deployment))
}

pub async fn get_statefulsets(
    client: &Client,
    namespace: Option<String>,
) -> Result<Vec<KubeStatefulSet>, kube::Error> {
    let statefulsets_api: Api<StatefulSet> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let statefulsets = statefulsets_api.list(&ListParams::default()).await?;
    Ok(statefulsets
        .into_iter()
        .map(KubeStatefulSet::from)
        .collect())
}

pub async fn get_daemonsets(
    client: &Client,
    namespace: Option<String>,
) -> Result<Vec<KubeDaemonSet>, kube::Error> {
    let daemonsets_api: Api<DaemonSet> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let daemonsets = daemonsets_api.list(&ListParams::default()).await?;
    Ok(daemonsets.into_iter().map(KubeDaemonSet::from).collect())
}

pub async fn get_replicasets(
    client: &Client,
    namespace: Option<String>,
) -> Result<Vec<KubeReplicaSet>, kube::Error> {
    let replicasets_api: Api<ReplicaSet> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let replicasets = replicasets_api.list(&ListParams::default()).await?;
    Ok(replicasets.into_iter().map(KubeReplicaSet::from).collect())
}

pub async fn get_jobs(
    client: &Client,
    namespace: Option<String>,
) -> Result<Vec<KubeJob>, kube::Error> {
    let jobs_api: Api<Job> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let jobs = jobs_api.list(&ListParams::default()).await?;
    Ok(jobs.into_iter().map(KubeJob::from).collect())
}

pub async fn get_cronjobs(
    client: &Client,
    namespace: Option<String>,
) -> Result<Vec<KubeCronJob>, kube::Error> {
    let cronjobs_api: Api<CronJob> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let cronjobs = cronjobs_api.list(&ListParams::default()).await?;
    Ok(cronjobs.into_iter().map(KubeCronJob::from).collect())
}

pub async fn get_replication_controllers(
    client: &Client,
    namespace: Option<String>,
) -> Result<Vec<KubeReplicationController>, kube::Error> {
    let replication_controllers_api: Api<ReplicationController> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let replication_controllers = replication_controllers_api
        .list(&ListParams::default())
        .await?;
    Ok(replication_controllers
        .into_iter()
        .map(KubeReplicationController::from)
        .collect())
}
