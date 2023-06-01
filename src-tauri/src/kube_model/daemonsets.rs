use k8s_openapi::{api::apps::v1::DaemonSet, chrono::Utc};
use serde::{Deserialize, Serialize};

use crate::common::utils;

use super::base::KubeResource;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeDaemonSet {
    pub name: String,
    pub namespace: String,
    pub desired: i32,
    pub current: i32,
    pub ready: i32,
    pub up_to_date: i32,
    pub available: i32,
    pub age: String,
    kube_object: DaemonSet,
}
impl From<DaemonSet> for KubeDaemonSet {
    fn from(daemon_set: DaemonSet) -> Self {
        let (desired, current, ready, up_to_date, available) = match daemon_set.status.as_ref() {
            Some(status) => (
                status.desired_number_scheduled,
                status.current_number_scheduled,
                status.number_ready,
                status.updated_number_scheduled.unwrap_or_default(),
                status.number_available.unwrap_or_default(),
            ),
            _ => (0, 0, 0, 0, 0),
        };

        KubeDaemonSet {
            name: daemon_set.metadata.name.clone().unwrap_or_default(),
            namespace: daemon_set.metadata.namespace.clone().unwrap_or_default(),
            age: utils::to_age(daemon_set.metadata.creation_timestamp.as_ref(), Utc::now()),
            desired,
            current,
            ready,
            up_to_date,
            available,
            kube_object: utils::remove_managed_fields(daemon_set),
        }
    }
}
impl KubeResource<DaemonSet> for KubeDaemonSet {
    fn get_kube_object(&self) -> &DaemonSet {
        &self.kube_object
    }
}
