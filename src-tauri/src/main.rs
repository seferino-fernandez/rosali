#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::sync::Arc;
use tokio::sync::Mutex;

mod cluster_connections;
mod commands;
mod common;
mod kube_clients;
mod kube_model;
mod kube_networking_client;
mod kube_workloads_client;
mod services;

use crate::cluster_connections::ClusterConnections;
use crate::commands::access_control_commands::*;
use crate::commands::cluster_commands::*;
use crate::commands::config_storage_commands::*;
use crate::commands::networking_commands::*;
use crate::commands::workloads_commands::*;

fn main() {
    let cluster_connections = Arc::new(Mutex::new(ClusterConnections::new()));
    tauri::Builder::default()
        .manage(cluster_connections)
        .invoke_handler(tauri::generate_handler![
            find_default_config_command,
            parse_kubeconfig,
            add_cluster_connection,
            remove_cluster_connection,
            get_context_overview,
            get_recent_events_command,
            get_pods_command,
            stream_pod_logs_command,
            get_deployments_command,
            restart_deployment_command,
            get_daemonsets_command,
            get_statefulsets_command,
            get_replicasets_command,
            get_jobs_command,
            get_cronjobs_command,
            get_replication_controllers_command,
            get_services_command,
            get_ingresses_command,
            get_ingress_classes_command,
            get_configmaps_command,
            get_secrets_command,
            get_persistent_volumn_claims_command,
            get_storage_classes_command,
            get_clusterroles_command,
            get_clusterrole_bindings_command,
            get_roles_command,
            get_role_bindings_command,
            get_service_accounts_command,
            get_namespaces_command,
            get_nodes_command,
            get_network_policies_command,
            get_endpoints_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
