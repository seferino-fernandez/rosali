use std::ops::Deref;

use k8s_openapi::api::core::v1::{Namespace, Node, ServiceAccount};
use kube::{api::ListParams, Api, Client};

use crate::kube_model::{
    namespaces::KubeNamespace, nodes::KubeNode, serviceaccounts::KubeServiceAccount,
};

pub async fn get_service_accounts(
    client: &Client,
    namespace: Option<String>,
) -> Result<Vec<KubeServiceAccount>, kube::Error> {
    let service_accounts_api: Api<ServiceAccount> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let service_accounts = service_accounts_api.list(&ListParams::default()).await?;
    Ok(service_accounts
        .into_iter()
        .map(KubeServiceAccount::from)
        .collect())
}

pub async fn get_namespaces(client: &Client) -> Result<Vec<KubeNamespace>, kube::Error> {
    let namespaces_api: Api<Namespace> = Api::all(client.clone());
    let namespaces = namespaces_api.list(&ListParams::default()).await?;
    Ok(namespaces.into_iter().map(KubeNamespace::from).collect())
}

pub async fn get_nodes(client: &Client) -> Result<Vec<KubeNode>, kube::Error> {
    let nodes_api: Api<Node> = Api::all(client.clone());
    let nodes = nodes_api.list(&ListParams::default()).await?;
    Ok(nodes.into_iter().map(KubeNode::from).collect())
}
