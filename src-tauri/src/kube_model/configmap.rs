use k8s_openapi::{api::core::v1::ConfigMap, chrono::Utc};
use serde::{Deserialize, Serialize};

use crate::common::utils;

use super::base::KubeResource;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeConfigMap {
    pub name: String,
    pub namespace: String,
    pub keys: Vec<String>,
    pub age: String,
    kube_object: ConfigMap,
}

impl From<ConfigMap> for KubeConfigMap {
    fn from(config_map: ConfigMap) -> Self {
        let keys: Vec<String> = match &config_map.data {
            Some(data) => data.keys().cloned().collect(),
            None => vec![],
        };

        KubeConfigMap {
            name: config_map.metadata.name.clone().unwrap_or_default(),
            namespace: config_map.metadata.namespace.clone().unwrap_or_default(),
            keys,
            age: utils::to_age(config_map.metadata.creation_timestamp.as_ref(), Utc::now()),
            kube_object: utils::remove_managed_fields(config_map),
        }
    }
}

impl KubeResource<ConfigMap> for KubeConfigMap {
    fn get_kube_object(&self) -> &ConfigMap {
        &self.kube_object
    }
}
