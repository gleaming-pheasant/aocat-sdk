use chrono::NaiveDateTime;

use crate::country::Country;
use crate::enums::{AriadneSubject, DataType, ResourceType};

#[derive(Debug, Default)]
pub struct ResourceBuilder {
    resource_type: Option<ResourceType>,
    access_rights: Option<String>,
    access_policy: Option<String>,
    original_id: Option<String>,
    identifiers: Vec<String>,
    title: Option<String>,
    description: Option<String>,
    country: Option<Country>,
    ariadne_subjects: Vec<AriadneSubject>,
    data_types: Vec<DataType>,
    created: Option<NaiveDateTime>,
    issued: Option<NaiveDateTime>,
    modified: Option<NaiveDateTime>,
    language: Option<String>,
}

impl ResourceBuilder {
    pub fn new() -> Self {
        Default::default()
    }
}
