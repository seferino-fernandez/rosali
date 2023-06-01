use std::ops::Deref;

use k8s_openapi::api::{
    core::v1::{ConfigMap, PersistentVolumeClaim, Secret},
    storage::v1::StorageClass,
};

use crate::kube_model::{
    configmap::KubeConfigMap, persistent_volumn_claims::KubePersistentVolumeClaim,
    secrets::KubeSecret, storage_classes::KubeStorageClass,
};
use kube::{api::ListParams, Api, Client};

pub async fn get_config_maps(
    client: &Client,
    namespace: Option<String>,
) -> Result<Vec<KubeConfigMap>, kube::Error> {
    let config_maps_api: Api<ConfigMap> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let config_maps = config_maps_api.list(&ListParams::default()).await?;
    Ok(config_maps.into_iter().map(KubeConfigMap::from).collect())
}

pub async fn get_secrets(
    client: &Client,
    namespace: Option<String>,
) -> Result<Vec<KubeSecret>, kube::Error> {
    let secrets_api: Api<Secret> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let secrets = secrets_api.list(&ListParams::default()).await?;
    Ok(secrets.into_iter().map(KubeSecret::from).collect())
}

pub async fn get_persistent_volumn_claims(
    client: &Client,
    namespace: Option<String>,
) -> Result<Vec<KubePersistentVolumeClaim>, kube::Error> {
    let pvcs_api: Api<PersistentVolumeClaim> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let pvcs = pvcs_api.list(&ListParams::default()).await?;
    Ok(pvcs
        .into_iter()
        .map(KubePersistentVolumeClaim::from)
        .collect())
}

pub async fn get_storage_classes(client: &Client) -> Result<Vec<KubeStorageClass>, kube::Error> {
    let storage_classes_api: Api<StorageClass> = Api::all(client.clone());
    let storage_classes = storage_classes_api.list(&ListParams::default()).await?;
    Ok(storage_classes
        .into_iter()
        .map(KubeStorageClass::from)
        .collect())
}
