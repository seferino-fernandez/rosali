use k8s_openapi::{api::rbac::v1::Role, chrono::Utc};
use serde::{Deserialize, Serialize};

use crate::common::utils;

use super::base::KubeResource;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeRole {
    pub name: String,
    pub namespace: String,
    pub labels: Option<std::collections::BTreeMap<String, String>>,
    pub annotations: Option<std::collections::BTreeMap<String, String>>,
    pub age: String,
    kube_object: Role,
}

impl From<Role> for KubeRole {
    fn from(role: Role) -> Self {
        KubeRole {
            name: role.metadata.name.clone().unwrap_or_default(),
            namespace: role.metadata.namespace.clone().unwrap_or_default(),
            labels: role.metadata.labels.clone(),
            annotations: role.metadata.annotations.clone(),
            age: utils::to_age(role.metadata.creation_timestamp.as_ref(), Utc::now()),
            kube_object: utils::remove_managed_fields(role),
        }
    }
}

impl KubeResource<Role> for KubeRole {
    fn get_kube_object(&self) -> &Role {
        &self.kube_object
    }
}
