use serde::{Deserialize, Serialize};

use crate::types::Uri;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Agent {
    name: String,
    identifier: Option<Uri>,
}

impl Agent {
    pub fn new(name: String, identifier: Option<Uri>) -> Self {
        Agent { name, identifier }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn identifier(&self) -> Option<&Uri> {
        self.identifier.as_ref()
    }
}
