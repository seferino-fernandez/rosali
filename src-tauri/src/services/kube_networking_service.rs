use std::sync::Arc;

use tauri::State;
use tokio::sync::Mutex;

use crate::cluster_connections::ClusterConnections;
use crate::common::common::Response;
use crate::kube_clients::kube_networking_client;
use crate::kube_model::endpoints::KubeEndpoints;
use crate::kube_model::ingress::KubeIngress;
use crate::kube_model::ingress_class::KubeIngressClass;
use crate::kube_model::network_policy::KubeNetworkPolicy;
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

    match kube_networking_client::get_services(&connection.client(), namespace).await {
        Ok(jobs) => Ok(Response {
            success: true,
            data: Some(jobs),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeService>>::error(e.to_string())),
    }
}

pub async fn get_ingresses(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeIngress>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeIngress>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match kube_networking_client::get_ingresses(&connection.client(), namespace).await {
        Ok(ingresses) => Ok(Response {
            success: true,
            data: Some(ingresses),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeIngress>>::error(e.to_string())),
    }
}

pub async fn get_ingress_classes(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
) -> Result<Response<Vec<KubeIngressClass>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeIngressClass>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match kube_networking_client::get_ingress_classes(&connection.client()).await {
        Ok(ingress_classes) => Ok(Response {
            success: true,
            data: Some(ingress_classes),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeIngressClass>>::error(e.to_string())),
    }
}

pub async fn get_network_policies(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeNetworkPolicy>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeNetworkPolicy>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match kube_networking_client::get_network_policies(&connection.client(), namespace).await {
        Ok(network_policies) => Ok(Response {
            success: true,
            data: Some(network_policies),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeNetworkPolicy>>::error(e.to_string())),
    }
}

pub async fn get_endpoints(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeEndpoints>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeEndpoints>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match kube_networking_client::get_endpoints(&connection.client(), namespace).await {
        Ok(endpoints) => Ok(Response {
            success: true,
            data: Some(endpoints),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeEndpoints>>::error(e.to_string())),
    }
}
