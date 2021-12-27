use std::fmt;
use std::collections::BTreeMap as Map;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};
use void::Void;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub key: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "alternate_names")]
    pub alternate_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<Bio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "birth_date")]
    pub birth_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "death_date")]
    pub death_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>, //No Idea What this looks like in real
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>, //No Idea What this looks like in real
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entity_type")]
    pub entity_type: Option<EntityType>, //No Idea What this looks like in real
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fuller_name")]
    pub fuller_name: Option<FullerName>, //No Idea What this looks like in real
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "personal_name")]
    pub personal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>, //According to https://openlibrary.org/authors/OL29497A.json looks like a String
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photos: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "remote_ids")]
    pub remote_ids: Option<RemoteIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia: Option<Wikipedia>, //No Idea What this looks like in real
    pub revision: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "latest_revision")]
    pub latest_revision: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Created>,
    #[serde(rename = "last_modified")]
    pub last_modified: LastModified,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bio {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(untagged)]
pub enum BioEnum {
    BioString(String),
    BioType(Bio)
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(rename = "type")]
    pub type_field: String,
    pub examples: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityType {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "enum")]
    pub enum_field: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullerName {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub url: String,
    #[serde(rename = "type")]
    pub type_field: Type,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteIds {
    pub viaf: String,
    pub wikidata: String,
    pub isni: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wikipedia {
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Created {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastModified {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}
