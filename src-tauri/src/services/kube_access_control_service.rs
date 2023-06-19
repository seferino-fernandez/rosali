use std::sync::Arc;

use tauri::State;
use tokio::sync::Mutex;

use crate::cluster_connections::ClusterConnections;
use crate::common::response::Response;
use crate::kube_clients::kube_access_control_client;
use crate::kube_model::clusterrolebindings::KubeClusterRoleBinding;
use crate::kube_model::clusterroles::KubeClusterRole;
use crate::kube_model::rolebindings::KubeRoleBinding;
use crate::kube_model::roles::KubeRole;

pub async fn get_cluster_roles(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
) -> Result<Response<Vec<KubeClusterRole>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeClusterRole>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match kube_access_control_client::get_cluster_roles(connection.client()).await {
        Ok(cluster_roles) => Ok(Response {
            success: true,
            data: Some(cluster_roles),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeClusterRole>>::error(e.to_string())),
    }
}

pub async fn get_cluster_role_bindings(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
) -> Result<Response<Vec<KubeClusterRoleBinding>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeClusterRoleBinding>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match kube_access_control_client::get_cluster_role_bindings(connection.client()).await {
        Ok(cluster_role_bindings) => Ok(Response {
            success: true,
            data: Some(cluster_role_bindings),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeClusterRoleBinding>>::error(
            e.to_string(),
        )),
    }
}

pub async fn get_roles(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeRole>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeRole>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match kube_access_control_client::get_roles(connection.client(), namespace).await {
        Ok(roles) => Ok(Response {
            success: true,
            data: Some(roles),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeRole>>::error(e.to_string())),
    }
}

pub async fn get_role_bindings(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeRoleBinding>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeRoleBinding>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match kube_access_control_client::get_roles_bindings(connection.client(), namespace).await {
        Ok(role_bindings) => Ok(Response {
            success: true,
            data: Some(role_bindings),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeRoleBinding>>::error(e.to_string())),
    }
}
