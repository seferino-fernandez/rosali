use k8s_openapi::{api::rbac::v1::ClusterRole, chrono::Utc};
use serde::{Deserialize, Serialize};

use crate::common::utils;

use super::base::KubeResource;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeClusterRole {
    pub name: String,
    pub labels: Option<std::collections::BTreeMap<String, String>>,
    pub annotations: Option<std::collections::BTreeMap<String, String>>,
    pub age: String,
    kube_object: ClusterRole,
}

impl From<ClusterRole> for KubeClusterRole {
    fn from(cluster_role: ClusterRole) -> Self {
        KubeClusterRole {
            name: cluster_role.metadata.name.clone().unwrap_or_default(),
            labels: cluster_role.metadata.labels.clone(),
            annotations: cluster_role.metadata.annotations.clone(),
            age: utils::to_age(
                cluster_role.metadata.creation_timestamp.as_ref(),
                Utc::now(),
            ),
            kube_object: utils::remove_managed_fields(cluster_role),
        }
    }
}

impl KubeResource<ClusterRole> for KubeClusterRole {
    fn get_kube_object(&self) -> &ClusterRole {
        &self.kube_object
    }
}
