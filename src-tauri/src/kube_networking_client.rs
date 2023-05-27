use std::ops::Deref;

use k8s_openapi::api::core::v1::Service;
use kube::{Api, Client, api::ListParams};
use crate::kube_model::services::KubeService;

pub async fn get_services(client: &Client, namespace: Option<String>,) -> Result<Vec<KubeService>, kube::Error> {
    let services_api: Api<Service> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let services = services_api.list(&ListParams::default()).await?;
    Ok(services.into_iter().map(KubeService::from).collect())
}
