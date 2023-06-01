use k8s_openapi::{
    api::core::v1::{
        Container, ContainerPort, ContainerState, ContainerStateWaiting, ContainerStatus, Pod,
        PodSpec, PodStatus,
    },
    chrono::Utc,
};
use serde::Serialize;

use crate::common::utils;

use super::base::KubeResource;

#[derive(Clone, Default, Debug, PartialEq, Serialize)]
pub struct KubePod {
    pub namespace: String,
    pub name: String,
    pub ready: (i32, i32),
    pub status: String,
    pub restarts: i32,
    pub cpu: String,
    pub mem: String,
    pub age: String,
    pub containers: Vec<KubeContainer>,
    kube_object: Pod,
}

#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize)]
pub struct KubeContainer {
    pub name: String,
    pub image: String,
    pub ready: String,
    pub status: String,
    pub restarts: i32,
    pub liveliness_probe: bool,
    pub readiness_probe: bool,
    pub ports: String,
    pub age: String,
    pub pod_name: String,
    pub init: bool,
}

impl From<Pod> for KubePod {
    fn from(pod: Pod) -> Self {
        let age = utils::to_age(pod.metadata.creation_timestamp.as_ref(), Utc::now());
        let pod_name = pod.metadata.name.clone().unwrap_or_default();
        let (status, containers_ready, restarts, containers_count, containers) = match &pod.status {
            Some(status) => {
                let (mut containers_ready_counter, mut restart_counter) = (0, 0);
                let containers_count = match status.container_statuses.as_ref() {
                    Some(container_statuses) => {
                        container_statuses.iter().for_each(|container_status| {
                            if container_status.ready {
                                containers_ready_counter += 1;
                            }
                            restart_counter += container_status.restart_count;
                        });
                        container_statuses.len()
                    }
                    None => 0,
                };

                let mut containers: Vec<KubeContainer> = pod
                    .spec
                    .as_ref()
                    .unwrap_or(&PodSpec::default())
                    .containers
                    .iter()
                    .map(|container| {
                        KubeContainer::from_api(
                            container,
                            pod_name.to_owned(),
                            age.to_owned(),
                            &status.container_statuses,
                            false,
                        )
                    })
                    .collect();

                let mut init_containers: Vec<KubeContainer> = pod
                    .spec
                    .as_ref()
                    .unwrap_or(&PodSpec::default())
                    .init_containers
                    .as_ref()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|container| {
                        KubeContainer::from_api(
                            container,
                            pod_name.to_owned(),
                            age.to_owned(),
                            &status.init_container_statuses,
                            true,
                        )
                    })
                    .collect();

                // merge containers and init-containers into single array
                containers.append(&mut init_containers);

                (
                    get_status(status, &pod),
                    containers_ready_counter,
                    restart_counter,
                    containers_count,
                    containers,
                )
            }
            _ => (utils::UNKNOWN.into(), 0, 0, 0, vec![]),
        };

        KubePod {
            name: pod_name,
            namespace: pod.metadata.namespace.clone().unwrap_or_default(),
            ready: (containers_ready, containers_count as i32),
            restarts,
            // TODO implement pod metrics
            cpu: String::default(),
            mem: String::default(),
            status,
            age,
            containers,
            kube_object: utils::remove_managed_fields(pod),
        }
    }
}

impl KubeResource<Pod> for KubePod {
    fn get_kube_object(&self) -> &Pod {
        &self.kube_object
    }
}

impl KubeContainer {
    pub fn from_api(
        container: &Container,
        pod_name: String,
        age: String,
        container_statuses_ref: &Option<Vec<ContainerStatus>>,
        init: bool,
    ) -> Self {
        let (mut ready, mut status, mut restarts) = ("false".to_string(), "<none>".to_string(), 0);
        if let Some(container_statuses) = container_statuses_ref {
            if let Some(container_status) = container_statuses
                .iter()
                .find(|cs| cs.name == container.name)
            {
                ready = container_status.ready.to_string();
                status = get_container_state(container_status.state.clone());
                restarts = container_status.restart_count;
            }
        }

        KubeContainer {
            name: container.name.clone(),
            pod_name,
            image: container.image.clone().unwrap_or_default(),
            ready,
            status,
            restarts,
            liveliness_probe: container.liveness_probe.is_some(),
            readiness_probe: container.readiness_probe.is_some(),
            ports: get_container_ports(&container.ports).unwrap_or_default(),
            age,
            init,
        }
    }
}

fn get_container_state(container_state: Option<ContainerState>) -> String {
    match container_state {
        Some(state) => {
            if let Some(container_state_waiting) = state.waiting {
                container_state_waiting
                    .reason
                    .unwrap_or_else(|| "Waiting".into())
            } else if let Some(container_state_terminated) = state.terminated {
                container_state_terminated
                    .reason
                    .unwrap_or_else(|| "Terminating".into())
            } else if state.running.is_some() {
                "Running".into()
            } else {
                utils::NONE.into()
            }
        }
        None => utils::NONE.into(),
    }
}

fn get_status(stat: &PodStatus, pod: &Pod) -> String {
    let status = match &stat.phase {
        Some(phase) => phase.to_owned(),
        _ => utils::UNKNOWN.into(),
    };
    let status = match &stat.reason {
        Some(reason) => {
            if reason == "NodeLost" && pod.metadata.deletion_timestamp.is_some() {
                utils::UNKNOWN.into()
            } else {
                reason.to_owned()
            }
        }
        None => status,
    };

    // get int container status
    let status = match &stat.init_container_statuses {
        Some(ics) => {
            for (i, container_status) in ics.iter().enumerate() {
                let c_status = match &container_status.state {
                    Some(container_state) => {
                        if let Some(container_state_terminated) = &container_state.terminated {
                            if container_state_terminated.exit_code == 0 {
                                "".into()
                            } else if container_state_terminated
                                .reason
                                .as_ref()
                                .unwrap_or(&String::default())
                                .is_empty()
                            {
                                format!(
                                    "Init:{}",
                                    container_state_terminated.reason.as_ref().unwrap()
                                )
                            } else if container_state_terminated.signal.unwrap_or_default() != 0 {
                                format!(
                                    "Init:Signal:{}",
                                    container_state_terminated.signal.unwrap()
                                )
                            } else {
                                format!("Init:ExitCode:{}", container_state_terminated.exit_code)
                            }
                        } else if is_pod_init(container_state.waiting.clone()) {
                            format!(
                                "Init:{}",
                                container_state
                                    .waiting
                                    .as_ref()
                                    .unwrap()
                                    .reason
                                    .as_ref()
                                    .unwrap_or(&String::default())
                            )
                        } else {
                            format!(
                                "Init:{}/{}",
                                i,
                                pod.spec
                                    .as_ref()
                                    .and_then(|ps| ps.init_containers.as_ref().map(|pic| pic.len()))
                                    .unwrap_or(0)
                            )
                        }
                    }
                    None => "".into(),
                };
                if !c_status.is_empty() {
                    return c_status;
                }
            }
            status
        }
        None => status,
    };

    let (mut status, running) = match &stat.container_statuses {
        Some(container_statuses_ref) => {
            let mut running = false;
            let status = container_statuses_ref
                .iter()
                .rev()
                .find_map(|container_state| {
                    container_state.state.as_ref().and_then(|s| {
                        if container_state.ready && s.running.is_some() {
                            running = true;
                        }
                        if s.waiting
                            .as_ref()
                            .and_then(|w| w.reason.as_ref().map(|v| !v.is_empty()))
                            .unwrap_or_default()
                        {
                            s.waiting.as_ref().and_then(|w| w.reason.clone())
                        } else if s
                            .terminated
                            .as_ref()
                            .and_then(|w| w.reason.as_ref().map(|v| !v.is_empty()))
                            .unwrap_or_default()
                        {
                            s.terminated.as_ref().and_then(|w| w.reason.clone())
                        } else if let Some(st) = &s.terminated {
                            if st.signal.unwrap_or_default() != 0 {
                                Some(format!("Signal:{}", st.signal.unwrap_or_default()))
                            } else {
                                Some(format!("ExitCode:{}", st.exit_code))
                            }
                        } else {
                            Some(status.clone())
                        }
                    })
                })
                .unwrap_or_default();
            (status, running)
        }
        None => (status, false),
    };

    if running && status == "Completed" {
        status = "Running".into();
    }

    if pod.metadata.deletion_timestamp.is_none() {
        return status;
    }

    "Terminating".into()
}

fn is_pod_init(container_state_waiting: Option<ContainerStateWaiting>) -> bool {
    container_state_waiting
        .map(|state_waiting| state_waiting.reason.unwrap_or_default() != "PodInitializing")
        .unwrap_or_default()
}

fn get_container_ports(ports_ref: &Option<Vec<ContainerPort>>) -> Option<String> {
    ports_ref.as_ref().map(|ports| {
        ports
            .iter()
            .map(|c_port| {
                let mut port = String::new();
                if let Some(name) = c_port.name.clone() {
                    port = format!("{}:", name);
                }
                port = format!("{}{}", port, c_port.container_port);
                if let Some(protocol) = c_port.protocol.clone() {
                    if protocol != "TCP" {
                        port = format!("{}/{}", port, c_port.protocol.clone().unwrap());
                    }
                }
                port
            })
            .collect::<Vec<_>>()
            .join(", ")
    })
}
