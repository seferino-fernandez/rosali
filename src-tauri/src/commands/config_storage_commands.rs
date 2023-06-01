use crate::cluster_connections::ClusterConnections;
use crate::common::common::Response;
use crate::kube_model::configmap::KubeConfigMap;
use crate::kube_model::persistent_volumn_claims::KubePersistentVolumeClaim;
use crate::kube_model::secrets::KubeSecret;
use crate::kube_model::storage_classes::KubeStorageClass;
use crate::services::kube_config_storage_service;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn get_configmaps_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeConfigMap>>, ()> {
    kube_config_storage_service::get_config_maps(connections, id, namespace).await
}

#[tauri::command]
pub async fn get_secrets_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeSecret>>, ()> {
    kube_config_storage_service::get_secrets(connections, id, namespace).await
}

#[tauri::command]
pub async fn get_persistent_volumn_claims_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubePersistentVolumeClaim>>, ()> {
    kube_config_storage_service::get_persistent_volumn_claims(connections, id, namespace).await
}

#[tauri::command]
pub async fn get_storage_classes_command(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
) -> Result<Response<Vec<KubeStorageClass>>, ()> {
    kube_config_storage_service::get_storage_classes(connections, id).await
}
