use kube::Client;
use std::collections::HashMap;

pub struct ClusterConnection {
    id: String,
    client: Client,
}

impl ClusterConnection {
    pub fn new(id: String, client: kube::Client) -> Self {
        Self { id, client }
    }

    pub fn client(&self) -> &kube::Client {
        &self.client
    }
}

pub struct ClusterConnections {
    connections: HashMap<String, ClusterConnection>,
}

impl ClusterConnections {
    // Creates a new empty ClusterConnections instance
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    // Adds a new cluster connection to the connections HashMap
    pub fn add_connection(&mut self, connection: ClusterConnection) {
        self.connections.insert(connection.id.clone(), connection);
    }

    // Removes a cluster connection from the connections HashMap by its ID
    pub fn remove_connection(&mut self, id: &str) {
        self.connections.remove(id);
    }

    // Retrieves a reference to a cluster connection by its ID
    pub fn get_connection(&self, id: &str) -> Option<&ClusterConnection> {
        self.connections.get(id)
    }
}
