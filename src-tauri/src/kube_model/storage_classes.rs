use super::base::KubeResource;
use crate::common::utils;
use k8s_openapi::{api::storage::v1::StorageClass, chrono::Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeStorageClass {
    pub name: String,
    pub provisioner: String,
    pub reclaim_policy: String,
    pub default: bool,
    pub age: String,
    kube_object: StorageClass,
}

impl From<StorageClass> for KubeStorageClass {
    fn from(sc: StorageClass) -> Self {
        let default = sc
            .metadata
            .annotations
            .clone()
            .and_then(|annotations| {
                annotations
                    .get("storageclass.kubernetes.io/is-default-class")
                    .map(|s| s.to_lowercase() == "true")
            })
            .unwrap_or(false);

        KubeStorageClass {
            name: sc.metadata.name.clone().unwrap_or_default(),
            provisioner: sc.provisioner.clone(),
            reclaim_policy: sc.reclaim_policy.clone().unwrap_or_default(),
            default,
            age: utils::to_age(sc.metadata.creation_timestamp.as_ref(), Utc::now()),
            kube_object: utils::remove_managed_fields(sc),
        }
    }
}

impl KubeResource<StorageClass> for KubeStorageClass {
    fn get_kube_object(&self) -> &StorageClass {
        &self.kube_object
    }
}
