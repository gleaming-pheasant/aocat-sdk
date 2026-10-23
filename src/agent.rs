use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::types::Uri;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Agent<'a> {
    name: Cow<'a, str>,
    identifier: Option<Uri<'a>>,
}

impl<'a> Agent<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>, identifier: Option<Uri<'a>>) -> Self {
        Self {
            name: name.into(),
            identifier,
        }
    }

    #[must_use]
    pub const fn new_const(name: &'static str, identifier: Option<Uri<'static>>) -> Self {
        Self {
            name: Cow::Borrowed(name),
            identifier,
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn identifier(&self) -> Option<&Uri<'_>> {
        self.identifier.as_ref()
    }
}
