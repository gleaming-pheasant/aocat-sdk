use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::Resource;

/// A `ResourceSet` is a collection of `Resource`s that aren't associated with the `api_id` and 
/// `id_path` that would be used to directly add them to a graph. Instead, this serves as a good 
/// wrapper for outputting resources in a format that might be better for mapping elsewhere, such 
/// as in an XML format suitable for X3ML.
#[derive(Debug, Serialize)]
pub struct ResourceSet<'a> {
    resources: Vec<Resource<'a>>,
}

impl<'a> ResourceSet<'a> {
    /// Creates a new `ResourceSet`.
    #[must_use]
    pub fn new(
        size_hint: Option<usize>
    ) -> Self {
        Self {
            resources: Vec::with_capacity(size_hint.unwrap_or_default()),
        }
    }

    #[must_use]
    pub fn resources(&self) -> &[Resource<'a>] {
        &self.resources
    }

    pub fn add_resource(&mut self, resource: Resource<'a>) {
        self.resources.push(resource);
    }

    /// Clear this `ResourceGraph` of all resources. This is useful for reusing the same
    /// `ResourceGraph` for multiple API calls; including paginating large datasets.
    pub fn flush(&mut self) {
        self.resources.clear();
    }

    #[cfg(feature = "xml")]
    /// # Errors
    /// If serialization to XML fails.
    pub fn as_xml(&self) -> Result<String, crate::Error> {
        let mut buf = String::new();
        let ser = quick_xml::se::Serializer::with_root(&mut buf, Some("resources"))?;
        self.serialize(ser)?;
        Ok(buf)
    }
}

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

    #[must_use]
    pub fn resources(&self) -> &[Resource<'a>] {
        &self.resources
    }

    #[must_use]
    pub fn api_id(&self) -> &str {
        &self.api_id
    }

    #[must_use]
    pub fn id_path(&self) -> &str {
        &self.id_path
    }

    pub fn add_resource(&mut self, resource: Resource<'a>) {
        self.resources.push(resource);
    }

    /// Clear this `ResourceGraph` of all resources. This is useful for reusing the same
    /// `ResourceGraph` for multiple API calls; including paginating large datasets.
    pub fn flush(&mut self) {
        self.resources.clear();
    }
}
