use std::sync::Arc;

use kube::config::Kubeconfig;
use tauri::{State, Window};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::cluster_connections::{ClusterConnections, ClusterConnection};
use crate::kube_model::event::KubeEvent;
use crate::kube_model::workload_status::WorkloadStatus;
use crate::kube_model::cronjobs::KubeCronJob;
use crate::kube_model::daemonsets::KubeDaemonSet;
use crate::kube_model::deployments::KubeDeployment;
use crate::kube_model::jobs::KubeJob;
use crate::kube_model::replicasets::KubeReplicaSet;
use crate::kube_model::replication_controllers::KubeReplicationController;
use crate::kube_model::statefulsets::KubeStatefulSet;
use crate::kube_model::pods::{KubePod};
use crate::common::common::Response;

use super::kubeconfig::{self, find_kubeconfig_for_context};

pub fn find_default_config() -> Result<Response<Vec<kubeconfig::Config>>, String> {
    match kubeconfig::get_default_kubeconfig() {
        Ok(kubeconfig) => {
            let contexts: Vec<kubeconfig::Config> = kubeconfig
                .contexts
                .into_iter()
                .map(|context| kubeconfig::Config {
                    name: context.name,
                    path: kubeconfig::find_default_kubeconfig(),
                    cluster: Some(context.context.clone().unwrap_or_default().cluster),
                    user: Some(context.context.clone().unwrap_or_default().user),
                    namespace: context.context.clone().unwrap_or_default().namespace,
                })
                .collect();

            Ok(Response {
                success: true,
                data: Some(contexts),
                error: None,
            })
        }
        Err(e) => Ok(Response::<Vec<kubeconfig::Config>> {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

pub fn parse_kubeconfig(path: &str) -> String {
    match kubeconfig::parse_kubeconfig(path) {
        Ok(kubeconfig) => serde_json::to_string(&Response {
            success: true,
            data: Some(kubeconfig),
            error: None,
        })
        .unwrap(),
        Err(e) => serde_json::to_string(&Response::<kube::config::Kubeconfig> {
            success: false,
            data: None,
            error: Some(e.to_string()),
        })
        .unwrap(),
    }
}

pub async fn add_cluster_connection(
    connections: State<'_, std::sync::Arc<Mutex<ClusterConnections>>>,
    context_name: String,
    context_path: Option<String>,
) -> Result<Response<String>, ()> {
    let kubeconfig_path = context_path.unwrap_or_else(|| {
        match find_kubeconfig_for_context(&context_name) {
            Ok(path) => path,
            Err(e) => {
                return format!(
                    "Could not find kubeconfig for context name {}. {}",
                    context_name, e
                )
            }
        }
    });

    match add_cluster_to_connections(&connections, &context_name, &kubeconfig_path).await {
        Ok(connection_id) => Ok(Response {
            success: true,
            data: Some(connection_id),
            error: None,
        }),
        Err(e) => Ok(Response {
            success: false,
            data: None,
            error: Some(e),
        }),
    }
}

// This is a private helper function that adds a cluster to connections.
async fn add_cluster_to_connections(
    connections: &Arc<Mutex<ClusterConnections>>,
    context_name: &str,
    kubeconfig_path: &str,
) -> Result<String, String> {
    let kubeconfig: Kubeconfig = kubeconfig::parse_kubeconfig(&kubeconfig_path).map_err(|e| {
        format!(
            "Failed to parse kubeconfig with context {}: {}",
            context_name, e
        )
    })?;

    let options = kube::config::KubeConfigOptions {
        context: Some(context_name.to_string()),
        ..Default::default()
    };
    let config = kube::Config::from_custom_kubeconfig(kubeconfig, &options)
        .await
        .map_err(|_| format!("Failed to create kube::Config for context {}", context_name))?;
    let client = kube::Client::try_from(config)
        .map_err(|_| format!("Failed to create kube::Client for context {}", context_name))?;

    let connection_id = Uuid::new_v4().to_string();
    println!("Generated connection_id: {}", connection_id);
    let connection = ClusterConnection::new(
        connection_id.clone(),
        client,
    );

    connections.lock().await.add_connection(connection);
    Ok(connection_id)
}

pub async fn remove_cluster_connection(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: &str,
) -> Result<String, ()> {
    connections.lock().await.remove_connection(id);
    let good_response = "Cluster connection successfully closed.";
    Ok(serde_json::to_string(&Response {
        success: true,
        data: Some(good_response),
        error: None,
    })
    .unwrap())
}

pub async fn get_context_overview(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
) -> Result<Response<WorkloadStatus>, ()> {
    let connections_locked = connections.lock().await;

    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<WorkloadStatus>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    let workload_status = crate::kube_workloads_client::get_workload_status(
        &connection.client().clone(),
        Some(connection.client().default_namespace().to_string()),
    )
    .await
    .unwrap_or_else(|_| WorkloadStatus::new());

    Ok(Response {
        success: true,
        data: Some(workload_status),
        error: None,
    })
}

pub async fn get_recent_events(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeEvent>>, ()> {
    let connections_locked = connections.lock().await;
    println!("Looking for connection with id: {}", id);
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeEvent>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match crate::kube_workloads_client::get_recent_events(&connection.client(), namespace).await {
        Ok(recent_events) => Ok(Response {
            success: true,
            data: Some(recent_events),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeEvent>>::error(e.to_string())),
    }
}

pub async fn get_pods(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubePod>>, ()> { // Replace CustomResource with KubePod
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubePod>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match crate::kube_workloads_client::get_pods(&connection.client(), namespace).await {
        Ok(pods) => Ok(Response {
            success: true,
            data: Some(pods),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubePod>>::error(e.to_string())),
    }
}

pub async fn stream_pod_logs(
    window: Window,
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
    pod_name: String,
) -> Result<Response<String>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<String>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match crate::kube_workloads_client::stream_pod_logs(&connection.client(), namespace, &pod_name, window).await {
        Ok(()) => Ok(Response {
            success: true,
            data: None,
            error: None,
        }),
        Err(e) => Ok(Response::<String>::error(e.to_string())),
    }
}

pub async fn get_deployments(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeDeployment>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeDeployment>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match crate::kube_workloads_client::get_deployments(&connection.client(), namespace).await {
        Ok(deployments) => Ok(Response {
            success: true,
            data: Some(deployments),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeDeployment>>::error(e.to_string())),
    }
}

pub async fn restart_deployment(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
    name: String,
) -> Result<Response<KubeDeployment>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<KubeDeployment>::error(
                "Cluster not found".to_string(),
            ));
        }
    };
    match crate::kube_workloads_client::restart_deployment(&connection.client(), namespace, &name).await {
        Ok(kube_deployment) => Ok(Response::<KubeDeployment>::success(kube_deployment)),
        Err(e) => Ok(Response::<KubeDeployment>::error(e.to_string())),
    }
}

pub async fn get_statefulsets(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeStatefulSet>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeStatefulSet>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match crate::kube_workloads_client::get_statefulsets(&connection.client(), namespace).await {
        Ok(statefulsets) => Ok(Response {
            success: true,
            data: Some(statefulsets),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeStatefulSet>>::error(e.to_string())),
    }
}

pub async fn get_daemonsets(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeDaemonSet>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeDaemonSet>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match crate::kube_workloads_client::get_daemonsets(&connection.client(), namespace).await {
        Ok(statefulsets) => Ok(Response {
            success: true,
            data: Some(statefulsets),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeDaemonSet>>::error(e.to_string())),
    }
}

pub async fn get_replicasets(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeReplicaSet>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeReplicaSet>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match crate::kube_workloads_client::get_replicasets(&connection.client(), namespace).await {
        Ok(statefulsets) => Ok(Response {
            success: true,
            data: Some(statefulsets),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeReplicaSet>>::error(e.to_string())),
    }
}

pub async fn get_jobs(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeJob>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeJob>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match crate::kube_workloads_client::get_jobs(&connection.client(), namespace).await {
        Ok(jobs) => Ok(Response {
            success: true,
            data: Some(jobs),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeJob>>::error(e.to_string())),
    }
}

pub async fn get_cronjobs(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeCronJob>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeCronJob>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match crate::kube_workloads_client::get_cronjobs(&connection.client(), namespace).await {
        Ok(jobs) => Ok(Response {
            success: true,
            data: Some(jobs),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeCronJob>>::error(e.to_string())),
    }
}

pub async fn get_replication_controllers(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeReplicationController>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeReplicationController>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match crate::kube_workloads_client::get_replication_controllers(&connection.client(), namespace).await {
        Ok(jobs) => Ok(Response {
            success: true,
            data: Some(jobs),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeReplicationController>>::error(e.to_string())),
    }
}