use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{State};
use crate::{cluster_connections::{ClusterConnections}, services::kube_networking_service};
use crate::common::common::Response;

use crate::kube_model::services::KubeService;

#[tauri::command]
pub async fn get_services_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeService>>, ()> {
    kube_networking_service::get_services(connections, id, namespace).await
}