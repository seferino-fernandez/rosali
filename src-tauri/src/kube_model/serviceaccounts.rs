use super::base::KubeResource;
use crate::common::utils;
use k8s_openapi::api::core::v1::ServiceAccount;
use k8s_openapi::chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeServiceAccount {
    pub name: String,
    pub namespace: String,
    pub age: String,
    kube_object: ServiceAccount,
}

impl From<ServiceAccount> for KubeServiceAccount {
    fn from(sa: ServiceAccount) -> Self {
        KubeServiceAccount {
            name: sa.metadata.name.clone().unwrap_or_default(),
            namespace: sa.metadata.namespace.clone().unwrap_or_default(),
            age: utils::to_age(sa.metadata.creation_timestamp.as_ref(), Utc::now()),
            kube_object: utils::remove_managed_fields(sa),
        }
    }
}

impl KubeResource<ServiceAccount> for KubeServiceAccount {
    fn get_kube_object(&self) -> &ServiceAccount {
        &self.kube_object
    }
}
