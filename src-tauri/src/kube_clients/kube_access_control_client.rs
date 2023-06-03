use k8s_openapi::api::rbac::v1::{ClusterRole, ClusterRoleBinding, Role, RoleBinding};
use std::ops::Deref;

use kube::{api::ListParams, Api, Client};

use crate::kube_model::{
    clusterrolebindings::KubeClusterRoleBinding, clusterroles::KubeClusterRole,
    rolebindings::KubeRoleBinding, roles::KubeRole,
};

pub async fn get_cluster_roles(client: &Client) -> Result<Vec<KubeClusterRole>, kube::Error> {
    let cluster_roles_api: Api<ClusterRole> = Api::all(client.clone());
    let cluster_roles = cluster_roles_api.list(&ListParams::default()).await?;
    Ok(cluster_roles
        .into_iter()
        .map(KubeClusterRole::from)
        .collect())
}

pub async fn get_cluster_role_bindings(
    client: &Client,
) -> Result<Vec<KubeClusterRoleBinding>, kube::Error> {
    let cluster_role_bindings_api: Api<ClusterRoleBinding> = Api::all(client.clone());
    let cluster_role_bindings = cluster_role_bindings_api
        .list(&ListParams::default())
        .await?;
    Ok(cluster_role_bindings
        .into_iter()
        .map(KubeClusterRoleBinding::from)
        .collect())
}

pub async fn get_roles(
    client: &Client,
    namespace: Option<String>,
) -> Result<Vec<KubeRole>, kube::Error> {
    let roles_api: Api<Role> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let roles = roles_api.list(&ListParams::default()).await?;
    Ok(roles.into_iter().map(KubeRole::from).collect())
}

pub async fn get_roles_bindings(
    client: &Client,
    namespace: Option<String>,
) -> Result<Vec<KubeRoleBinding>, kube::Error> {
    let role_bindings_api: Api<RoleBinding> = match namespace {
        Some(namespace) => Api::namespaced(client.clone(), namespace.deref()),
        None => Api::all(client.clone()),
    };
    let role_bindings = role_bindings_api.list(&ListParams::default()).await?;
    Ok(role_bindings
        .into_iter()
        .map(KubeRoleBinding::from)
        .collect())
}
