use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::Resource;

#[derive(Debug, Deserialize, Serialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct ResourceGraph<'a> {
    api_id: Cow<'a, str>,
    id_path: Cow<'a, str>,
    resources: Vec<Resource<'a>>,
}

impl<'a> ResourceGraph<'a> {
    pub fn new(
        size_hint: usize,
        api_id: impl Into<Cow<'a, str>>,
        id_path: impl Into<Cow<'a, str>>,
    ) -> Self {
        ResourceGraph {
            api_id: api_id.into(),
            id_path: id_path.into(),
            resources: Vec::with_capacity(size_hint),
        }
    }

    pub fn resources(&self) -> &[Resource<'a>] {
        &self.resources
    }

    pub fn api_id(&self) -> &str {
        &self.api_id
    }

    pub fn id_path(&self) -> &str {
        &self.id_path
    }

    pub fn add_resource(&mut self, resource: Resource<'a>) {
        self.resources.push(resource);
    }
}
