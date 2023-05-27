use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeEvent {
    pub name: String,
    pub namespace: Option<String>,
    pub reason: Option<String>,
    pub message: Option<String>,
    pub warning_or_regular: String,
    pub source: Option<String>,
    pub reporting_component: Option<String>,
    pub count: i32,
    pub first_time: Option<String>,
    pub last_time: Option<String>,
}