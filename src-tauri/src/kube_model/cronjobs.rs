use k8s_openapi::{api::batch::v1::CronJob, chrono::Utc};
use serde::{Deserialize, Serialize};

use crate::common::utils;

use super::base::KubeResource;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeCronJob {
    pub name: String,
    pub namespace: String,
    pub schedule: String,
    pub last_schedule: String,
    pub suspend: bool,
    pub active: usize,
    pub age: String,
    kube_object: CronJob,
}

impl From<CronJob> for KubeCronJob {
    fn from(cronjob: CronJob) -> Self {
        let (last_schedule, active) = match &cronjob.status {
            Some(cronjob_status) => (
                utils::to_age_secs(cronjob_status.last_schedule_time.as_ref(), Utc::now()),
                cronjob_status.active.clone().unwrap_or_default().len(),
            ),
            None => (utils::NOT_AVAILABLE.into(), 0),
        };

        let (schedule, suspend) = match &cronjob.spec {
            Some(cronjob_spec) => (
                cronjob_spec.schedule.clone(),
                cronjob_spec.suspend.unwrap_or_default(),
            ),
            None => ("".to_string(), false),
        };

        KubeCronJob {
            name: cronjob.metadata.name.clone().unwrap_or_default(),
            namespace: cronjob.metadata.namespace.clone().unwrap_or_default(),
            schedule,
            suspend,
            last_schedule,
            active,
            age: utils::to_age(cronjob.metadata.creation_timestamp.as_ref(), Utc::now()),
            kube_object: utils::remove_managed_fields(cronjob),
        }
    }
}
impl KubeResource<CronJob> for KubeCronJob {
    fn get_kube_object(&self) -> &CronJob {
        &self.kube_object
    }
}
