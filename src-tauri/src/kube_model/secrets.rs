use k8s_openapi::{api::core::v1::Secret, chrono::Utc};
use serde::{Deserialize, Serialize};

use crate::common::utils;

use super::base::KubeResource;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeSecret {
    pub name: String,
    pub namespace: String,
    pub labels: std::collections::BTreeMap<String, String>,
    pub keys: Vec<String>,
    pub secret_type: String,
    pub age: String,
    kube_object: Secret,
}

impl From<Secret> for KubeSecret {
    fn from(secret: Secret) -> Self {
        let keys: Vec<String> = match &secret.data {
            Some(data) => data.keys().cloned().collect(),
            None => vec![],
        };

        KubeSecret {
            name: secret.metadata.name.clone().unwrap_or_default(),
            namespace: secret.metadata.namespace.clone().unwrap_or_default(),
            labels: secret.metadata.labels.clone().unwrap_or_default(),
            keys,
            secret_type: secret.type_.clone().unwrap_or_default(),
            age: utils::to_age(secret.metadata.creation_timestamp.as_ref(), Utc::now()),
            kube_object: utils::remove_managed_fields(secret),
        }
    }
}

impl KubeResource<Secret> for KubeSecret {
    fn get_kube_object(&self) -> &Secret {
        &self.kube_object
    }
}
