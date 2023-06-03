use k8s_openapi::{api::rbac::v1::ClusterRoleBinding, chrono::Utc};
use serde::{Deserialize, Serialize};

use crate::common::utils;

use super::base::KubeResource;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeClusterRoleBinding {
    pub name: String,
    pub bindings: Vec<String>,
    pub age: String,
    kube_object: ClusterRoleBinding,
}

impl From<ClusterRoleBinding> for KubeClusterRoleBinding {
    fn from(cluster_role_binding: ClusterRoleBinding) -> Self {
        KubeClusterRoleBinding {
            name: cluster_role_binding
                .metadata
                .name
                .clone()
                .unwrap_or_default(),
            bindings: cluster_role_binding
                .role_ref
                .name
                .split(',')
                .map(|s| s.to_string())
                .collect(),
            age: utils::to_age(
                cluster_role_binding.metadata.creation_timestamp.as_ref(),
                Utc::now(),
            ),
            kube_object: utils::remove_managed_fields(cluster_role_binding),
        }
    }
}

impl KubeResource<ClusterRoleBinding> for KubeClusterRoleBinding {
    fn get_kube_object(&self) -> &ClusterRoleBinding {
        &self.kube_object
    }
}
