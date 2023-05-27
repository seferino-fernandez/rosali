use std::collections::BTreeMap;

use k8s_openapi::{api::core::v1::Service, chrono::Utc};
use serde::{Serialize, Deserialize};

use crate::common::utils;

use super::{base::KubeResource};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeService {
    pub name: String,
    pub namespace: String,
    pub type_: String,
    pub cluster_ip: String,
    pub ports: String,
    pub external_ip: String,
    pub selector: String,
    pub age: String,
    pub status: String,
    kube_object: Service,
}

impl From<Service> for KubeService {
  fn from(service: Service) -> Self {
      KubeService {
          name: service.metadata.name.clone().unwrap_or_default(),
          namespace: service.metadata.namespace.clone().unwrap_or_default(),
          type_: service.spec.as_ref().unwrap().type_.as_ref().unwrap_or(&"".to_string()).clone(),
          cluster_ip: service.spec.as_ref().unwrap().cluster_ip.clone().unwrap_or_default(),
          ports: service.spec.as_ref().unwrap().ports.as_ref().unwrap_or(&Vec::new()).iter().map(|port| format!("{} ({})", port.port, port.protocol.as_ref().unwrap_or(&"".to_string()))).collect::<Vec<String>>().join(", "),
          external_ip: service.spec.as_ref().unwrap().external_ips.as_ref().unwrap_or(&Vec::new()).join(", "),
          selector: service.spec.as_ref().unwrap().selector.as_ref().unwrap_or(&BTreeMap::new()).iter().map(|(key, val)| format!("{}={}", key, val)).collect::<Vec<String>>().join(","),
          age: utils::to_age(service.metadata.creation_timestamp.as_ref(), Utc::now()),
          status: if service.spec.as_ref().unwrap().external_ips.as_ref().unwrap_or(&Vec::new()).is_empty() && service.spec.as_ref().unwrap().cluster_ips.as_ref().unwrap_or(&Vec::new()).is_empty() { "Inactive".to_string() } else { "Active".to_string() },
          kube_object: utils::remove_managed_fields(service),
      }
  }
}

impl KubeResource<Service> for KubeService {
    fn get_kube_object(&self) -> &Service {
        &self.kube_object
    }
}