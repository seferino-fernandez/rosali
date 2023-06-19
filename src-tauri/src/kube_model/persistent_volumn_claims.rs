use k8s_openapi::{api::core::v1::PersistentVolumeClaim, chrono::Utc};
use serde::{Deserialize, Serialize};

use super::base::KubeResource;
use crate::common::utils;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubePersistentVolumeClaim {
    pub name: String,
    pub namespace: String,
    pub storage_class: String,
    pub size: String,
    pub pods: Vec<String>,
    pub age: String,
    pub status: String,
    kube_object: PersistentVolumeClaim,
}

impl From<PersistentVolumeClaim> for KubePersistentVolumeClaim {
    fn from(pvc: PersistentVolumeClaim) -> Self {
        let size = match pvc
            .spec
            .clone()
            .and_then(|spec| spec.resources)
            .and_then(|resources| resources.requests)
            .and_then(|requests| requests.get("storage").cloned())
        {
            Some(quantity) => quantity.0,
            None => "".to_string(),
        };

        KubePersistentVolumeClaim {
            name: pvc.metadata.name.clone().unwrap_or_default(),
            namespace: pvc.metadata.namespace.clone().unwrap_or_default(),
            storage_class: pvc
                .spec
                .clone()
                .unwrap()
                .storage_class_name
                .unwrap_or_default(),
            size,
            pods: vec![],
            age: utils::to_age(pvc.metadata.creation_timestamp.as_ref(), Utc::now()),
            status: pvc.status.clone().unwrap().phase.unwrap_or_default(),
            kube_object: utils::remove_managed_fields(pvc),
        }
    }
}

impl KubeResource<PersistentVolumeClaim> for KubePersistentVolumeClaim {
    fn get_kube_object(&self) -> &PersistentVolumeClaim {
        &self.kube_object
    }
}
