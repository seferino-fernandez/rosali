use k8s_openapi::api::apps::v1::{DeploymentSpec, Deployment};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KubeDeployment {
    pub name: String,
    pub namespace: String,
    pub spec: Option<DeploymentSpec>,
    pub status: Option<DeploymentStatus>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeploymentStatus {
    pub replicas: Option<i32>,
    pub updated_replicas: Option<i32>,
    pub ready_replicas: Option<i32>,
    pub available_replicas: Option<i32>,
}

impl From<Deployment> for KubeDeployment {
    fn from(deployment: Deployment) -> Self {
        Self {
            name: deployment.metadata.name.unwrap_or_else(|| String::from("")),
            namespace: deployment.metadata.namespace.unwrap_or_else(|| String::from("")),
            spec: deployment.spec,
            status: deployment.status.map(|status| DeploymentStatus {
                replicas: status.replicas,
                updated_replicas: status.updated_replicas,
                ready_replicas: status.ready_replicas,
                available_replicas: status.available_replicas,
            }),
        }
    }
}