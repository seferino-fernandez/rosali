use super::base::KubeResource;
use crate::common::utils;
use k8s_openapi::api::core::v1::Node;
use k8s_openapi::chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeCondition {
    pub type_: String,
    pub status: String,
    pub message: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeNode {
    pub name: String,
    pub age: String,
    pub taints: Vec<String>,
    pub version: String,
    pub conditions: Vec<NodeCondition>,
    kube_object: Node,
}

impl From<Node> for KubeNode {
    fn from(node: Node) -> Self {
        let conditions: Vec<NodeCondition> = node
            .status
            .as_ref()
            .and_then(|status| status.conditions.as_ref())
            .map_or_else(Vec::new, |conditions| {
                conditions
                    .iter()
                    .map(|condition| NodeCondition {
                        type_: condition.type_.clone(),
                        status: condition.status.clone(),
                        message: condition.message.clone(),
                    })
                    .collect()
            });

        KubeNode {
            name: node.metadata.name.clone().unwrap_or_default(),
            age: utils::to_age(node.metadata.creation_timestamp.as_ref(), Utc::now()),
            taints: node
                .spec
                .as_ref()
                .and_then(|spec| spec.taints.as_ref())
                .map_or_else(Vec::new, |taints| {
                    taints.iter().map(|taint| taint.key.clone()).collect()
                }),
            version: node
                .status
                .as_ref()
                .and_then(|status| status.node_info.as_ref())
                .map(|node_info| node_info.kubelet_version.clone())
                .unwrap_or_default(),
            conditions,
            kube_object: utils::remove_managed_fields(node),
        }
    }
}

impl KubeResource<Node> for KubeNode {
    fn get_kube_object(&self) -> &Node {
        &self.kube_object
    }
}