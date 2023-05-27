use std::sync::Arc;

use tauri::{State};
use tokio::sync::Mutex;

use crate::common::common::Response;
use crate::cluster_connections::{ClusterConnections};
use crate::kube_model::services::KubeService;

pub async fn get_services(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeService>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeService>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match crate::kube_networking_client::get_services(&connection.client(), namespace).await {
        Ok(jobs) => Ok(Response {
            success: true,
            data: Some(jobs),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeService>>::error(e.to_string())),
    }
}