#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::sync::Arc;
use tokio::sync::Mutex;

mod cluster_connections;
mod commands;
mod kube_model;
mod common;
mod services;
mod kube_workloads_client;
mod kube_networking_client;

use crate::cluster_connections::ClusterConnections;
use crate::commands::workloads_commands::*;
use crate::commands::networking_commands::*;

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
            get_services_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
