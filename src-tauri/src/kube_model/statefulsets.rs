use k8s_openapi::{api::apps::v1::StatefulSet, chrono::Utc};
use serde::{Deserialize, Serialize};

use crate::common::utils;

use super::{base::KubeResource};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeStatefulSet {
  pub name: String,
  pub namespace: String,
  pub ready: String,
  pub service: String,
  pub age: String,
  kube_object: StatefulSet,
}

impl From<StatefulSet> for KubeStatefulSet {
  fn from(stateful_set: StatefulSet) -> Self {
    let ready = match &stateful_set.status {
      Some(status) => format!("{}/{}", status.ready_replicas.unwrap_or_default(), status.replicas),
      _ => "".into(),
    };

    KubeStatefulSet {
      name: stateful_set.metadata.name.clone().unwrap_or_default(),
      namespace: stateful_set.metadata.namespace.clone().unwrap_or_default(),
      age: utils::to_age(stateful_set.metadata.creation_timestamp.as_ref(), Utc::now()),
      service: stateful_set
        .spec
        .as_ref()
        .map_or(utils::NOT_AVAILABLE.into(), |spec| spec.service_name.to_owned()),
      ready,
      kube_object: utils::remove_managed_fields(stateful_set),
    }
  }
}

impl KubeResource<StatefulSet> for KubeStatefulSet {
  fn get_kube_object(&self) -> &StatefulSet {
    &self.kube_object
  }
}
