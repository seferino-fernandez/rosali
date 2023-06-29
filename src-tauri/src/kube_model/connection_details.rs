use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionDetails {
    pub connection_id: String,
    pub name: String,
    pub path: String,
    pub namespace: String,
}
