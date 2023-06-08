use crate::common::common::Response;
use crate::kube_model::endpoints::KubeEndpoints;
use crate::kube_model::network_policy::KubeNetworkPolicy;
use crate::{
    cluster_connections::ClusterConnections,
    kube_model::{ingress::KubeIngress, ingress_class::KubeIngressClass},
    services::kube_networking_service,
};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

use crate::kube_model::services::KubeService;

#[tauri::command]
pub async fn get_services_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeService>>, ()> {
    kube_networking_service::get_services(connections, id, namespace).await
}

#[tauri::command]
pub async fn get_ingresses_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeIngress>>, ()> {
    kube_networking_service::get_ingresses(connections, id, namespace).await
}

#[tauri::command]
pub async fn get_ingress_classes_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
) -> Result<Response<Vec<KubeIngressClass>>, ()> {
    kube_networking_service::get_ingress_classes(connections, id).await
}

#[tauri::command]
pub async fn get_network_policies_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeNetworkPolicy>>, ()> {
    kube_networking_service::get_network_policies(connections, id, namespace).await
}

#[tauri::command]
pub async fn get_endpoints_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeEndpoints>>, ()> {
    kube_networking_service::get_endpoints(connections, id, namespace).await
}
