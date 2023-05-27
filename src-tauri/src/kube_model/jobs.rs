use k8s_openapi::{api::batch::v1::Job, chrono::Utc};
use serde::{Deserialize, Serialize};

use crate::common::utils;

use super::{base::KubeResource};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeJob {
  pub name: String,
  pub namespace: String,
  pub completions: i32,
  pub parallelism: i32,
  pub active: i32,
  pub succeeded: i32,
  pub failed: i32,
  pub age: String,
  kube_object: Job,
}
impl From<Job> for KubeJob {
  fn from(job: Job) -> Self {
    let (completions, parallelism, active, succeeded, failed) = match job.spec.as_ref() {
      Some(job_spec) => (
        job_spec.completions.unwrap_or_default(),
        job_spec.parallelism.unwrap_or_default(),
        job.status.as_ref().map_or(0, |status| status.active.unwrap_or_default()),
        job.status.as_ref().map_or(0, |status| status.succeeded.unwrap_or_default()),
        job.status.as_ref().map_or(0, |status: &k8s_openapi::api::batch::v1::JobStatus| status.failed.unwrap_or_default()),
      ),
      _ => (0, 0, 0, 0, 0),
    };

    KubeJob {
      name: job.metadata.name.clone().unwrap_or_default(),
      namespace: job.metadata.namespace.clone().unwrap_or_default(),
      age: utils::to_age(job.metadata.creation_timestamp.as_ref(), Utc::now()),
      completions,
      parallelism,
      active,
      succeeded,
      failed,
      kube_object: utils::remove_managed_fields(job),
    }
  }
}
impl KubeResource<Job> for KubeJob {
  fn get_kube_object(&self) -> &Job {
    &self.kube_object
  }
}