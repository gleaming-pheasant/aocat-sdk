use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum ResourceType {
    #[serde(rename = "AO_Collection")]
    Collection,
    #[serde(rename = "AO_Individual_Data_Resource")]
    Individual,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum AriadneSubject {
    #[serde(rename = "Site/monument")]
    SiteMonument,
    Fieldwork,
    #[serde(rename = "Fieldwork report")]
    FieldworkReport,
    #[serde(rename = "Fieldwork archive")]
    FieldworkArchive,
    #[serde(rename = "Scientific analysis")]
    ScientificAnalysis,
    Date,
    Artefact,
    Coin,
    #[serde(rename = "Building survey")]
    BuildingSurvey,
    Maritime,
    Inscription,
    #[serde(rename = "Rock Art")]
    RockArt,
    Burial,
    #[serde(rename = "E-Publication")]
    EPublication,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum DataType {
    #[serde(rename = "Structured Data")]
    StructuredData,
    Geospatial,
    #[serde(rename = "Still Image")]
    StillImage,
    Text,
    CAD,
    Numeric,
    Software,
    #[serde(rename = "3D")]
    ThreeD,
    Video,
    Other,
    Audio,
}
