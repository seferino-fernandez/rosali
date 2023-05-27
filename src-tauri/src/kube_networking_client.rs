use std::ops::Deref;

use k8s_openapi::api::{core::v1::Service, networking::v1::{Ingress, IngressClass}};
use kube::{Api, Client, api::ListParams};
use crate::kube_model::{services::KubeService, ingress::KubeIngress, ingress_class::KubeIngressClass};

pub async fn get_services(client: &Client, namespace: Option<String>,) -> Result<Vec<KubeService>, kube::Error> {
    let services_api: Api<Service> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let services = services_api.list(&ListParams::default()).await?;
    Ok(services.into_iter().map(KubeService::from).collect())
}

pub async fn get_ingresses(client: &Client, namespace: Option<String>,) -> Result<Vec<KubeIngress>, kube::Error> {
    let ingresses_api: Api<Ingress> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let ingresses = ingresses_api.list(&ListParams::default()).await?;
    Ok(ingresses.into_iter().map(KubeIngress::from).collect())
}

pub async fn get_ingress_classes(client: &Client) -> Result<Vec<KubeIngressClass>, kube::Error> {
    let ingress_classes_api: Api<IngressClass> = Api::all(client.clone());
    let ingress_classes = ingress_classes_api.list(&ListParams::default()).await?;
    Ok(ingress_classes.into_iter().map(KubeIngressClass::from).collect())
}
