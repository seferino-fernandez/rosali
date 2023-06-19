use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

use crate::cluster_connections::ClusterConnections;
use crate::common::response::Response;
use crate::kube_model::clusterrolebindings::KubeClusterRoleBinding;
use crate::kube_model::clusterroles::KubeClusterRole;
use crate::kube_model::rolebindings::KubeRoleBinding;
use crate::kube_model::roles::KubeRole;
use crate::services::kube_access_control_service;

#[tauri::command]
pub async fn get_clusterroles_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
) -> Result<Response<Vec<KubeClusterRole>>, ()> {
    kube_access_control_service::get_cluster_roles(connections, id).await
}

#[tauri::command]
pub async fn get_clusterrole_bindings_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
) -> Result<Response<Vec<KubeClusterRoleBinding>>, ()> {
    kube_access_control_service::get_cluster_role_bindings(connections, id).await
}

#[tauri::command]
pub async fn get_roles_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeRole>>, ()> {
    kube_access_control_service::get_roles(connections, id, namespace).await
}

#[tauri::command]
pub async fn get_role_bindings_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeRoleBinding>>, ()> {
    kube_access_control_service::get_role_bindings(connections, id, namespace).await
}
