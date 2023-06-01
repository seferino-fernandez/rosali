use std::sync::Arc;

use tauri::State;
use tokio::sync::Mutex;

use crate::cluster_connections::ClusterConnections;
use crate::common::common::Response;

use crate::kube_model::configmap::KubeConfigMap;
use crate::kube_model::persistent_volumn_claims::KubePersistentVolumeClaim;
use crate::kube_model::secrets::KubeSecret;
use crate::kube_model::storage_classes::KubeStorageClass;

pub async fn get_config_maps(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeConfigMap>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeConfigMap>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match crate::kube_clients::kube_config_storage_client::get_config_maps(
        &connection.client(),
        namespace,
    )
    .await
    {
        Ok(configmaps) => Ok(Response {
            success: true,
            data: Some(configmaps),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeConfigMap>>::error(e.to_string())),
    }
}

pub async fn get_secrets(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubeSecret>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeSecret>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match crate::kube_clients::kube_config_storage_client::get_secrets(
        &connection.client(),
        namespace,
    )
    .await
    {
        Ok(secrets) => Ok(Response {
            success: true,
            data: Some(secrets),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeSecret>>::error(e.to_string())),
    }
}

pub async fn get_persistent_volumn_claims(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
    namespace: Option<String>,
) -> Result<Response<Vec<KubePersistentVolumeClaim>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubePersistentVolumeClaim>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match crate::kube_clients::kube_config_storage_client::get_persistent_volumn_claims(
        &connection.client(),
        namespace,
    )
    .await
    {
        Ok(pvcs) => Ok(Response {
            success: true,
            data: Some(pvcs),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubePersistentVolumeClaim>>::error(
            e.to_string(),
        )),
    }
}

pub async fn get_storage_classes(
    connections: State<'_, Arc<Mutex<ClusterConnections>>>,
    id: String,
) -> Result<Response<Vec<KubeStorageClass>>, ()> {
    let connections_locked = connections.lock().await;
    let connection = match connections_locked.get_connection(&id) {
        Some(conn) => conn,
        None => {
            return Ok(Response::<Vec<KubeStorageClass>>::error(
                "Cluster not found".to_string(),
            ));
        }
    };

    match crate::kube_clients::kube_config_storage_client::get_storage_classes(&connection.client())
        .await
    {
        Ok(storage_classes) => Ok(Response {
            success: true,
            data: Some(storage_classes),
            error: None,
        }),
        Err(e) => Ok(Response::<Vec<KubeStorageClass>>::error(e.to_string())),
    }
}
