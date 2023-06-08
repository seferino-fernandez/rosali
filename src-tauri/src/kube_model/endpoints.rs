use super::base::KubeResource;
use crate::common::utils;
use k8s_openapi::api::core::v1::Endpoints;
use k8s_openapi::chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeEndpoints {
    pub name: String,
    pub namespace: String,
    pub endpoints: Vec<String>,
    pub age: String,
    k8s_obj: Endpoints,
}

impl From<Endpoints> for KubeEndpoints {
    fn from(endpoints: Endpoints) -> Self {
        KubeEndpoints {
            name: endpoints.metadata.name.clone().unwrap_or_default(),
            namespace: endpoints.metadata.namespace.clone().unwrap_or_default(),
            endpoints: Self::extract_endpoints(&endpoints),
            age: utils::to_age(endpoints.metadata.creation_timestamp.as_ref(), Utc::now()),
            k8s_obj: endpoints,
        }
    }
}

impl KubeEndpoints {
    fn extract_endpoints(endpoints: &Endpoints) -> Vec<String> {
        let mut result = Vec::new();
        if let Some(subsets) = &endpoints.subsets {
            for subset in subsets {
                if let Some(addresses) = &subset.addresses {
                    for address in addresses {
                        if let Some(ports) = &subset.ports {
                            for port in ports {
                                let endpoint = format!("{}:{}", address.ip, port.port);
                                result.push(endpoint);
                            }
                        }
                    }
                }
            }
        }
        result
    }
}

impl KubeResource<Endpoints> for KubeEndpoints {
    fn get_kube_object(&self) -> &Endpoints {
        &self.k8s_obj
    }
}
