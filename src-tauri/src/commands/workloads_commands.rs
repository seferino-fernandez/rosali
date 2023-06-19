use std::sync::Arc;
use tauri::{State, Window};
use tokio::sync::Mutex;

use crate::common::response::Response;
use crate::{
    cluster_connections::ClusterConnections,
    kube_model::{
        cronjobs::KubeCronJob, daemonsets::KubeDaemonSet, deployments::KubeDeployment,
        event::KubeEvent, jobs::KubeJob, pods::KubePod, replicasets::KubeReplicaSet,
        replication_controllers::KubeReplicationController, statefulsets::KubeStatefulSet,
        workload_status::WorkloadStatus,
    },
    services::{kube_workloads_service, kubeconfig},
};

#[tauri::command]
pub fn find_default_config_command() -> Result<Response<Vec<kubeconfig::Config>>, String> {
    kube_workloads_service::find_default_config()
}

#[tauri::command]
pub fn parse_kubeconfig(path: &str) -> String {
    kube_workloads_service::parse_kubeconfig(path)
}

#[tauri::command]
pub async fn add_cluster_connection(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    context_name: String,
    context_path: Option<String>,
) -> Result<Response<String>, ()> {
    kube_workloads_service::add_cluster_connection(connections, context_name, context_path).await
}

#[tauri::command]
pub async fn remove_cluster_connection(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: &str,
) -> Result<String, ()> {
    kube_workloads_service::remove_cluster_connection(connections, id).await
}

#[tauri::command]
pub async fn get_context_overview(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<WorkloadStatus>, ()> {
    kube_workloads_service::get_context_overview(connections, id, namespace).await
}

#[tauri::command]
pub async fn get_recent_events_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeEvent>>, ()> {
    kube_workloads_service::get_recent_events(connections, id, namespace).await
}

#[tauri::command]
pub async fn get_pods_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubePod>>, ()> {
    kube_workloads_service::get_pods(connections, id, namespace).await
}

#[tauri::command]
pub async fn stream_pod_logs_command(
    window: Window,
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
    pod_name: String,
) -> Result<Response<String>, ()> {
    kube_workloads_service::stream_pod_logs(window, connections, id, namespace, pod_name).await
}

#[tauri::command]
pub async fn get_deployments_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeDeployment>>, ()> {
    kube_workloads_service::get_deployments(connections, id, namespace).await
}

#[tauri::command]
pub async fn restart_deployment_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
    name: String,
) -> Result<Response<KubeDeployment>, ()> {
    kube_workloads_service::restart_deployment(connections, id, namespace, name).await
}

#[tauri::command]
pub async fn get_daemonsets_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeDaemonSet>>, ()> {
    kube_workloads_service::get_daemonsets(connections, id, namespace).await
}

#[tauri::command]
pub async fn get_replicasets_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeReplicaSet>>, ()> {
    kube_workloads_service::get_replicasets(connections, id, namespace).await
}

#[tauri::command]
pub async fn get_statefulsets_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeStatefulSet>>, ()> {
    kube_workloads_service::get_statefulsets(connections, id, namespace).await
}

#[tauri::command]
pub async fn get_jobs_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeJob>>, ()> {
    kube_workloads_service::get_jobs(connections, id, namespace).await
}

#[tauri::command]
pub async fn get_cronjobs_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeCronJob>>, ()> {
    kube_workloads_service::get_cronjobs(connections, id, namespace).await
}

#[tauri::command]
pub async fn get_replication_controllers_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeReplicationController>>, ()> {
    kube_workloads_service::get_replication_controllers(connections, id, namespace).await
}
