use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

use crate::cluster_connections::ClusterConnections;
use crate::common::response::Response;
use crate::kube_model::namespaces::KubeNamespace;
use crate::kube_model::nodes::KubeNode;
use crate::kube_model::serviceaccounts::KubeServiceAccount;
use crate::services::kube_cluster_service;

#[tauri::command]
pub async fn get_service_accounts_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeServiceAccount>>, ()> {
    kube_cluster_service::get_service_accounts(connections, id, namespace).await
}

#[tauri::command]
pub async fn get_namespaces_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
) -> Result<Response<Vec<KubeNamespace>>, ()> {
    kube_cluster_service::get_namespaces(connections, id).await
}

#[tauri::command]
pub async fn get_nodes_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
) -> Result<Response<Vec<KubeNode>>, ()> {
    kube_cluster_service::get_nodes(connections, id).await
}
