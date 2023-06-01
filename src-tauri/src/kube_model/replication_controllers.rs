use std::collections::BTreeMap;

use k8s_openapi::{api::core::v1::ReplicationController, chrono::Utc};
use serde::{Deserialize, Serialize};

use crate::common::utils;

use super::base::KubeResource;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeReplicationController {
    pub name: String,
    pub namespace: String,
    pub desired: i32,
    pub current: i32,
    pub ready: i32,
    pub containers: String,
    pub images: String,
    pub selector: String,
    pub age: String,
    kube_object: ReplicationController,
}

impl From<ReplicationController> for KubeReplicationController {
    fn from(replication_controller: ReplicationController) -> Self {
        let (current, ready) = match replication_controller.status.as_ref() {
            Some(status) => (status.replicas, status.ready_replicas.unwrap_or_default()),
            _ => (0, 0),
        };

        let (desired, selector, (containers, images)) = match replication_controller.spec.as_ref() {
            Some(spec) => (
                spec.replicas.unwrap_or_default(),
                spec.selector
                    .as_ref()
                    .unwrap_or(&BTreeMap::new())
                    .iter()
                    .map(|(key, val)| format!("{}={}", key, val))
                    .collect::<Vec<String>>()
                    .join(","),
                match spec.template.as_ref() {
                    Some(tmpl) => match tmpl.spec.as_ref() {
                        Some(pspec) => (
                            pspec
                                .containers
                                .iter()
                                .map(|container| container.name.to_owned())
                                .collect::<Vec<String>>()
                                .join(","),
                            pspec
                                .containers
                                .iter()
                                .filter_map(|container| container.image.to_owned())
                                .collect::<Vec<String>>()
                                .join(","),
                        ),
                        None => ("".into(), "".into()),
                    },
                    None => ("".into(), "".into()),
                },
            ),
            None => (0, "".into(), ("".into(), "".into())),
        };

        KubeReplicationController {
            name: replication_controller
                .metadata
                .name
                .clone()
                .unwrap_or_default(),
            namespace: replication_controller
                .metadata
                .namespace
                .clone()
                .unwrap_or_default(),
            age: utils::to_age(
                replication_controller.metadata.creation_timestamp.as_ref(),
                Utc::now(),
            ),
            desired,
            current,
            ready,
            containers,
            images,
            selector,
            kube_object: utils::remove_managed_fields(replication_controller),
        }
    }
}

impl KubeResource<ReplicationController> for KubeReplicationController {
    fn get_kube_object(&self) -> &ReplicationController {
        &self.kube_object
    }
}
