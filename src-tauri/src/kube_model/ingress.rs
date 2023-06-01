use k8s_openapi::{api::networking::v1::Ingress, chrono::Utc};
use serde::{Deserialize, Serialize};

use crate::common::utils;

use super::base::KubeResource;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeIngress {
    pub name: String,
    pub namespace: String,
    pub rules: String,
    pub load_balancer: String,
    pub age: String,
    k8s_obj: Ingress,
}

impl From<Ingress> for KubeIngress {
    fn from(ingress: Ingress) -> Self {
        let rules = ingress
            .spec
            .as_ref()
            .and_then(|spec| {
                spec.rules.as_ref().map(|rules| {
                    rules
                        .iter()
                        .map(|rule| {
                            let paths = rule
                                .http
                                .as_ref()
                                .and_then(|http| {
                                    Some(
                                        http.paths
                                            .iter()
                                            .map(|path| path.path.clone().unwrap_or_default())
                                            .collect::<Vec<_>>()
                                            .join(", "),
                                    )
                                })
                                .unwrap_or_default();
                            format!("{}: {}", rule.host.clone().unwrap_or_default(), paths)
                        })
                        .collect::<Vec<_>>()
                        .join("; ")
                })
            })
            .unwrap_or_default();

        let load_balancer = ingress
            .status
            .as_ref()
            .and_then(|status| {
                status.load_balancer.as_ref().and_then(|load_balancer| {
                    Some(
                        load_balancer
                            .ingress
                            .as_ref()
                            .map(|ingresses| {
                                ingresses
                                    .iter()
                                    .map(|ingress| {
                                        let ip = ingress.ip.as_ref().unwrap().clone();
                                        let hostname = ingress.hostname.as_ref().unwrap().clone();
                                        if !ip.is_empty() {
                                            ip
                                        } else {
                                            hostname
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            })
                            .unwrap_or_default(),
                    )
                })
            })
            .unwrap_or_default();

        KubeIngress {
            name: ingress.metadata.name.clone().unwrap_or_default(),
            namespace: ingress.metadata.namespace.clone().unwrap_or_default(),
            rules,
            load_balancer,
            age: utils::to_age(ingress.metadata.creation_timestamp.as_ref(), Utc::now()),
            k8s_obj: ingress,
        }
    }
}

impl KubeResource<Ingress> for KubeIngress {
    fn get_kube_object(&self) -> &Ingress {
        &self.k8s_obj
    }
}
