use std::borrow::Cow;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::Error;
use crate::agent::Agent;
use crate::country::Country;
use crate::enums::{AriadneSubject, DataType, ResourceType};
use crate::geometry::Geometry;
use crate::image::{Image, Images};
use crate::language::Language;
use crate::temporal_region::TemporalRegion;
use crate::types::{Agents, AriadneSubjects, DataTypes, NativeTypes, Uri};

#[derive(Debug, Deserialize, Serialize)]
#[serde(bound(deserialize = "'de: 'a"), try_from = "ResourceBuilder")]
pub struct Resource<'a> {
    resource_type: ResourceType,
    access_rights: Cow<'a, str>,
    access_policy: Option<Cow<'a, str>>,
    original_id: Cow<'a, str>,
    #[serde(default)]
    identifiers: Vec<Cow<'a, str>>,
    title: Cow<'a, str>,
    description: Option<Cow<'a, str>>,
    country: Country<'a>,
    ariadne_subjects: AriadneSubjects,
    data_types: DataTypes,
    created: NaiveDateTime,
    issued: NaiveDateTime,
    modified: NaiveDateTime,
    language: Language,
    landing_page: Uri<'a>,
    #[serde(default)]
    named_places: Vec<Cow<'a, str>>,
    native_subjects: Vec<Cow<'a, str>>, // not default, as mandatory if AO_Individual_Data_Resource
    parent_id: Option<Cow<'a, str>>, // not default, as mandatory if AO_Individual_Data_Resource
    native_types: NativeTypes<'a>,
    #[serde(default)]
    temporal_regions: Vec<TemporalRegion<'a>>,
    #[serde(default)]
    geometries: Vec<Geometry>,
    publishers: Agents<'a>,
    owners: Agents<'a>,
    contributors: Agents<'a>,
    creators: Agents<'a>,
    responsibles: Agents<'a>,
    #[serde(default)]
    images: Images<'a>,
}

impl<'a> TryFrom<ResourceBuilder<'a>> for Resource<'a> {
    type Error = Error;

    fn try_from(value: ResourceBuilder<'a>) -> Result<Self, Self::Error> {
        value.build()
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct ResourceBuilder<'a> {
    #[serde(default)]
    resource_type: Option<ResourceType>,
    #[serde(default)]
    access_rights: Option<Cow<'a, str>>,
    #[serde(default)]
    access_policy: Option<Cow<'a, str>>,
    #[serde(default)]
    original_id: Option<Cow<'a, str>>,
    #[serde(default)]
    identifiers: Vec<Cow<'a, str>>,
    #[serde(default)]
    title: Option<Cow<'a, str>>,
    #[serde(default)]
    description: Option<Cow<'a, str>>,
    #[serde(default)]
    country: Option<Country<'a>>,
    #[serde(default)]
    ariadne_subjects: Vec<AriadneSubject>,
    #[serde(default)]
    data_types: Vec<DataType>,
    #[serde(default)]
    created: Option<NaiveDateTime>,
    #[serde(default)]
    issued: Option<NaiveDateTime>,
    #[serde(default)]
    modified: Option<NaiveDateTime>,
    #[serde(default)]
    language: Option<Language>,
    #[serde(default)]
    landing_page: Option<Cow<'a, str>>,
    #[serde(default)]
    named_places: Vec<Cow<'a, str>>,
    #[serde(default)]
    native_subjects: Vec<Cow<'a, str>>,
    #[serde(default)]
    parent_id: Option<Cow<'a, str>>,
    #[serde(default)]
    native_types: Vec<Cow<'a, str>>,
    #[serde(default)]
    temporal_regions: Vec<TemporalRegion<'a>>,
    #[serde(default)]
    geometries: Vec<Geometry>,
    #[serde(default)]
    publishers: Vec<Agent<'a>>,
    #[serde(default)]
    owners: Vec<Agent<'a>>,
    #[serde(default)]
    contributors: Vec<Agent<'a>>,
    #[serde(default)]
    creators: Vec<Agent<'a>>,
    #[serde(default)]
    responsibles: Vec<Agent<'a>>,
    #[serde(default)]
    images: Vec<Image<'a>>,
}

impl<'a> ResourceBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_resource_type(mut self, value: ResourceType) -> Self {
        self.resource_type = Some(value);
        self
    }

    pub fn set_access_rights(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.access_rights = Some(value.into());
        self
    }

    pub fn set_access_policy(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.access_policy = Some(value.into());
        self
    }

    pub fn set_original_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.original_id = Some(value.into());
        self
    }

    pub fn set_identifiers(mut self, value: Vec<Cow<'a, str>>) -> Self {
        self.identifiers = value;
        self
    }

    pub fn add_identifier(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.identifiers.push(value.into());
        self
    }

    pub fn set_title(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn set_description(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn set_country(mut self, value: Country<'a>) -> Self {
        self.country = Some(value);
        self
    }

    pub fn set_ariadne_subjects(mut self, value: Vec<AriadneSubject>) -> Self {
        self.ariadne_subjects = value;
        self
    }

    pub fn add_ariadne_subject(mut self, value: AriadneSubject) -> Self {
        self.ariadne_subjects.push(value);
        self
    }

    pub fn set_data_types(mut self, value: Vec<DataType>) -> Self {
        self.data_types = value;
        self
    }

    pub fn add_data_type(mut self, value: DataType) -> Self {
        self.data_types.push(value);
        self
    }

    pub fn set_created(mut self, value: NaiveDateTime) -> Self {
        self.created = Some(value);
        self
    }

    pub fn set_issued(mut self, value: NaiveDateTime) -> Self {
        self.issued = Some(value);
        self
    }

    pub fn set_modified(mut self, value: NaiveDateTime) -> Self {
        self.modified = Some(value);
        self
    }

    pub fn set_language(mut self, value: Language) -> Self {
        self.language = Some(value);
        self
    }

    pub fn set_landing_page(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.landing_page = Some(value.into());
        self
    }

    pub fn set_named_places(mut self, value: Vec<Cow<'a, str>>) -> Self {
        self.named_places = value;
        self
    }

    pub fn add_named_place(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.named_places.push(value.into());
        self
    }

    pub fn set_native_subjects(mut self, value: Vec<Cow<'a, str>>) -> Self {
        self.native_subjects = value;
        self
    }

    pub fn add_native_subject(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.native_subjects.push(value.into());
        self
    }

    pub fn set_parent_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    pub fn set_native_types(mut self, value: Vec<Cow<'a, str>>) -> Self {
        self.native_types = value;
        self
    }

    pub fn add_native_type(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.native_types.push(value.into());
        self
    }

    pub fn set_temporal_regions(mut self, value: Vec<TemporalRegion<'a>>) -> Self {
        self.temporal_regions = value;
        self
    }

    pub fn add_temporal_region(mut self, value: TemporalRegion<'a>) -> Self {
        self.temporal_regions.push(value);
        self
    }

    pub fn set_geometries(mut self, value: Vec<Geometry>) -> Self {
        self.geometries = value;
        self
    }

    pub fn add_geometry(mut self, value: Geometry) -> Self {
        self.geometries.push(value);
        self
    }

    pub fn set_publishers(mut self, value: Vec<Agent<'a>>) -> Self {
        self.publishers = value;
        self
    }

    pub fn add_publisher(mut self, value: Agent<'a>) -> Self {
        self.publishers.push(value);
        self
    }

    pub fn set_owners(mut self, value: Vec<Agent<'a>>) -> Self {
        self.owners = value;
        self
    }

    pub fn add_owner(mut self, value: Agent<'a>) -> Self {
        self.owners.push(value);
        self
    }

    pub fn set_contributors(mut self, value: Vec<Agent<'a>>) -> Self {
        self.contributors = value;
        self
    }

    pub fn add_contributor(mut self, value: Agent<'a>) -> Self {
        self.contributors.push(value);
        self
    }

    pub fn set_creators(mut self, value: Vec<Agent<'a>>) -> Self {
        self.creators = value;
        self
    }

    pub fn add_creator(mut self, value: Agent<'a>) -> Self {
        self.creators.push(value);
        self
    }

    pub fn set_responsibles(mut self, value: Vec<Agent<'a>>) -> Self {
        self.responsibles = value;
        self
    }

    pub fn add_responsible(mut self, value: Agent<'a>) -> Self {
        self.responsibles.push(value);
        self
    }

    pub fn set_images(mut self, value: Vec<Image<'a>>) -> Self {
        self.images = value;
        self
    }

    pub fn add_image(mut self, value: Image<'a>) -> Self {
        self.images.push(value);
        self
    }

    pub fn build(self) -> Result<Resource<'a>, Error> {
        if self.resource_type == Some(ResourceType::Individual) && self.native_subjects.is_empty() {
            return Err(Error::MissingField(
                "native_subjects (required for AO_Individual_Data_Resource)",
            ));
        }

        if self.resource_type == Some(ResourceType::Individual) && self.parent_id.is_none() {
            return Err(Error::MissingField(
                "parent_id (required for AO_Individual_Data_Resource)",
            ));
        }

        Ok(Resource {
            resource_type: self
                .resource_type
                .ok_or(Error::MissingField("resource_type"))?,
            access_rights: self
                .access_rights
                .ok_or(Error::MissingField("access_rights"))?,
            access_policy: self.access_policy,
            original_id: self.original_id.ok_or(Error::MissingField("original_id"))?,
            identifiers: self.identifiers,
            title: self.title.ok_or(Error::MissingField("title"))?,
            description: self.description,
            country: self.country.ok_or(Error::MissingField("country"))?,
            ariadne_subjects: self.ariadne_subjects.try_into()?,
            data_types: self.data_types.try_into()?,
            created: self.created.ok_or(Error::MissingField("created"))?,
            issued: self.issued.ok_or(Error::MissingField("issued"))?,
            modified: self.modified.ok_or(Error::MissingField("modified"))?,
            language: self.language.ok_or(Error::MissingField("language"))?,
            landing_page: self
                .landing_page
                .ok_or(Error::MissingField("landing_page"))?
                .try_into()?,
            named_places: self.named_places,
            native_subjects: self.native_subjects,
            parent_id: self.parent_id,
            native_types: self.native_types.try_into()?,
            temporal_regions: self.temporal_regions,
            geometries: self.geometries,
            publishers: self.publishers.try_into()?,
            owners: self.owners.try_into()?,
            contributors: self.contributors.try_into()?,
            creators: self.creators.try_into()?,
            responsibles: self.responsibles.try_into()?,
            images: self.images.try_into()?,
        })
    }
}
