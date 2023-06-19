use super::base::KubeResource;
use crate::common::utils;
use k8s_openapi::{api::networking::v1::IngressClass, chrono::Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeIngressClass {
    pub name: String,
    pub namespace: String,
    pub controller: String,
    pub api_group: String,
    pub scope: String,
    pub kind: String,
    pub age: String,
    k8s_obj: IngressClass,
}

impl From<IngressClass> for KubeIngressClass {
    fn from(ingress_class: IngressClass) -> Self {
        KubeIngressClass {
            name: ingress_class
                .metadata
                .name
                .clone()
                .unwrap_or_default()
                ,
            namespace: ingress_class.metadata.namespace.clone().unwrap_or_default(),
            controller: ingress_class
                .spec
                .clone()
                .unwrap_or_default()
                .controller
                .unwrap_or_default()
                ,
            api_group: ingress_class
                .metadata
                .annotations
                .clone()
                .unwrap_or_default()
                .get("kubernetes.io/ingress.class")
                .unwrap_or(&"".to_string())
                .to_string(),
            scope: ingress_class
                .spec
                .clone()
                .unwrap_or_default()
                .parameters
                .unwrap_or_default()
                .scope
                .unwrap_or_default()
                ,
            kind: ingress_class
                .spec
                .clone()
                .unwrap_or_default()
                .parameters
                .as_ref()
                .map(|parameters| parameters.kind.clone())
                .unwrap_or_default(),
            age: utils::to_age(
                ingress_class.metadata.creation_timestamp.as_ref(),
                Utc::now(),
            ),
            k8s_obj: ingress_class,
        }
    }
}

impl KubeResource<IngressClass> for KubeIngressClass {
    fn get_kube_object(&self) -> &IngressClass {
        &self.k8s_obj
    }
}
