use std::sync::Arc;

use tauri::State;
use tokio::sync::Mutex;

use crate::cluster_connections::ClusterConnections;
use crate::common::common::Response;
use crate::kube_clients::kube_cluster_client;
use crate::kube_model::namespaces::KubeNamespace;
use crate::kube_model::nodes::KubeNode;
use crate::kube_model::serviceaccounts::KubeServiceAccount;

pub async fn get_service_accounts(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeServiceAccount>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeServiceAccount>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match kube_cluster_client::get_service_accounts(&connection.client(), namespace).await {
        Ok(service_accounts) => Ok(Response {
            success: true,
            data: Some(service_accounts),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeServiceAccount>>::error(e.to_string())),
    }
}

pub async fn get_namespaces(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
) -> Result<Response<Vec<KubeNamespace>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeNamespace>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match kube_cluster_client::get_namespaces(&connection.client()).await {
        Ok(namespaces) => Ok(Response {
            success: true,
            data: Some(namespaces),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeNamespace>>::error(e.to_string())),
    }
}

pub async fn get_nodes(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
) -> Result<Response<Vec<KubeNode>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeNode>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match kube_cluster_client::get_nodes(&connection.client()).await {
        Ok(nodes) => Ok(Response {
            success: true,
            data: Some(nodes),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeNode>>::error(e.to_string())),
    }
}
