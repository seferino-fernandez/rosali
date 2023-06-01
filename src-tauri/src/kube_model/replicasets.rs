use crate::common::utils;
use k8s_openapi::{api::apps::v1::ReplicaSet, chrono::Utc};
use serde::{Deserialize, Serialize};

use super::base::KubeResource;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeReplicaSet {
    name: String,
    namespace: String,
    desired: i32,
    current: i32,
    ready: i32,
    age: String,
    kube_object: ReplicaSet,
}

impl From<ReplicaSet> for KubeReplicaSet {
    fn from(replica_set: ReplicaSet) -> Self {
        let replica_set_clone: ReplicaSet = replica_set.clone();

        let (desired, current, ready) = match replica_set.status {
            Some(status) => (
                status.replicas,
                status.available_replicas.unwrap_or_default(),
                status.ready_replicas.unwrap_or_default(),
            ),
            _ => (0, 0, 0),
        };

        KubeReplicaSet {
            name: replica_set.metadata.name.unwrap_or_default(),
            namespace: replica_set.metadata.namespace.unwrap_or_default(),
            age: utils::to_age(replica_set.metadata.creation_timestamp.as_ref(), Utc::now()),
            desired,
            current,
            ready,
            kube_object: replica_set_clone,
        }
    }
}

impl KubeResource<ReplicaSet> for KubeReplicaSet {
    fn get_kube_object(&self) -> &ReplicaSet {
        &self.kube_object
    }
}
