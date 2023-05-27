use std::env;
use dirs::home_dir;
use std::fs;
use kube::config::Kubeconfig;
use serde::{Serialize};
use serde_yaml;

#[derive(Clone, Debug, Serialize)]
pub struct Config {
    pub name: String,
    pub path: Option<String>,
    pub cluster: Option<String>,
    pub user: Option<String>,
    pub namespace: Option<String>,
}

// Tries to find the default kubeconfig file path
pub fn find_default_kubeconfig() -> Option<String> {
    // Check the KUBECONFIG environment variable
    if let Ok(kubeconfig_env) = env::var("KUBECONFIG") {
        return Some(kubeconfig_env);
    }

    // Check the default location in the user's home directory
    if let Some(home_dir) = home_dir() {
        let default_path = home_dir.join(".kube/config");
        if default_path.exists() {
            return Some(default_path.to_string_lossy().to_string());
        }
    }

    None
}

// Parses a kubeconfig file from a given path
pub fn parse_kubeconfig(path: &str) -> Result<Kubeconfig, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)
        .map_err(|err: std::io::Error| format!("Failed to read kubeconfig file at {}: {}", path, err))?;
    let kubeconfig: Kubeconfig = serde_yaml::from_str(&contents)
        .map_err(|err| format!("Failed to parse kubeconfig file at {}: {}", path, err))?;
    Ok(kubeconfig)
}

pub fn find_kubeconfig_for_context(context_name: &str) -> Result<String, String> {
    let kubeconfig = get_default_kubeconfig().map_err(|e| e.to_string())?;

    // Verify the context exists
    kubeconfig
        .contexts
        .iter()
        .find(|c| c.name == context_name)
        .ok_or_else(|| format!("Context {} not found", context_name))?;

    // Get the path of the kubeconfig file
    let kubeconfig_path = find_default_kubeconfig()
        .ok_or_else(|| "Failed to find default kubeconfig file".to_string())?;

    Ok(kubeconfig_path)
}

pub fn get_default_kubeconfig() -> Result<Kubeconfig, Box<dyn std::error::Error>> {
    let path = find_default_kubeconfig()
        .ok_or("Unable to find default kubeconfig")?;
    let kubeconfig = parse_kubeconfig(&path)?;
    Ok(kubeconfig)
}
