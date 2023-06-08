use super::base::KubeResource;
use crate::common::utils;
use k8s_openapi::api::networking::v1::NetworkPolicy;
use k8s_openapi::chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeNetworkPolicy {
    pub name: String,
    pub namespace: String,
    pub policy_types: Option<Vec<String>>,
    pub age: String,
    k8s_obj: NetworkPolicy,
}

impl From<NetworkPolicy> for KubeNetworkPolicy {
    fn from(network_policy: NetworkPolicy) -> Self {
        KubeNetworkPolicy {
            name: network_policy.metadata.name.clone().unwrap_or_default(),
            namespace: network_policy
                .metadata
                .namespace
                .clone()
                .unwrap_or_default(),
            policy_types: network_policy
                .spec
                .clone()
                .and_then(|spec| spec.policy_types),
            age: utils::to_age(
                network_policy.metadata.creation_timestamp.as_ref(),
                Utc::now(),
            ),
            k8s_obj: network_policy,
        }
    }
}

impl KubeResource<NetworkPolicy> for KubeNetworkPolicy {
    fn get_kube_object(&self) -> &NetworkPolicy {
        &self.k8s_obj
    }
}
