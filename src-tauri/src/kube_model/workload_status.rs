use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WorkloadStatus {
    pub deployments: DeploymentStatus,
    pub pods: PodStatus,
    pub replicas: ReplicaStatus,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DeploymentStatus {
    pub running: i32,
    pub failed: i32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PodStatus {
    pub running: i32,
    pub failed: i32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ReplicaStatus {
    pub running: i32,
    pub failed: i32,
}

impl WorkloadStatus {
    pub fn new() -> Self {
        WorkloadStatus {
            deployments: Default::default(),
            pods: Default::default(),
            replicas: Default::default(),
        }
    }
}
