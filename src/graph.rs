use serde::{Deserialize, Serialize};

use crate::Resource;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceGraph {
    api_id: String,
    id_path: String,
    resources: Vec<Resource>
}

impl ResourceGraph {
    pub fn new(size_hint: usize, api_id: String, id_path: String) -> Self {
        ResourceGraph {
            api_id,
            id_path,
            resources: Vec::with_capacity(size_hint)
        }
    }

    pub fn resources(&self) -> &Vec<Resource> {
        &self.resources
    }

    pub fn api_id(&self) -> &String {
        &self.api_id
    }

    pub fn id_path(&self) -> &String {
        &self.id_path
    }

    pub fn add_resource(&mut self, resource: Resource) {
        self.resources.push(resource);
    }
}
