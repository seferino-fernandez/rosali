use k8s_openapi::{api::rbac::v1::RoleBinding, chrono::Utc};
use serde::{Deserialize, Serialize};

use crate::common::utils;

use super::base::KubeResource;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeRoleBinding {
    pub name: String,
    pub namespace: String,
    pub bindings: Vec<String>,
    pub age: String,
    kube_object: RoleBinding,
}

impl From<RoleBinding> for KubeRoleBinding {
    fn from(role_binding: RoleBinding) -> Self {
        KubeRoleBinding {
            name: role_binding.metadata.name.clone().unwrap_or_default(),
            namespace: role_binding.metadata.namespace.clone().unwrap_or_default(),
            bindings: role_binding
                .role_ref
                .name
                .split(',')
                .map(|s| s.to_string())
                .collect(),
            age: utils::to_age(
                role_binding.metadata.creation_timestamp.as_ref(),
                Utc::now(),
            ),
            kube_object: utils::remove_managed_fields(role_binding),
        }
    }
}

impl KubeResource<RoleBinding> for KubeRoleBinding {
    fn get_kube_object(&self) -> &RoleBinding {
        &self.kube_object
    }
}
