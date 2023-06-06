use super::base::KubeResource;
use crate::common::utils;
use k8s_openapi::api::core::v1::Namespace;
use k8s_openapi::chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeNamespace {
    pub name: String,
    pub labels: Option<std::collections::BTreeMap<String, String>>,
    pub age: String,
    pub status: String,
    kube_object: Namespace,
}

impl From<Namespace> for KubeNamespace {
    fn from(namespace: Namespace) -> Self {
        KubeNamespace {
            name: namespace.metadata.name.clone().unwrap_or_default(),
            labels: namespace.metadata.labels.clone(),
            age: utils::to_age(namespace.metadata.creation_timestamp.as_ref(), Utc::now()),
            status: namespace.status.clone().unwrap().phase.unwrap_or_default(),
            kube_object: utils::remove_managed_fields(namespace),
        }
    }
}

impl KubeResource<Namespace> for KubeNamespace {
    fn get_kube_object(&self) -> &Namespace {
        &self.kube_object
    }
}
