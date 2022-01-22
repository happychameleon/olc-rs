use std::fmt;
use std::collections::BTreeMap as Map;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};
use void::Void;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edition {

    pub key: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<Authors>>,
    pub works: Vec<Works>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Identifiers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isbn_10")]
    pub isbn10: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isbn_13")]
    pub isbn13: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lccn: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "oclc_numbers")]
    pub oclc_numbers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub covers: Option<Vec<isize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<Language>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "by_statement")]
    pub by_statement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<Weight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "edition_name")]
    pub edition_name: Option<String>, //OL16341636M uses a string here instead of a EditionName struct
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "number_of_pages")]
    pub number_of_pages: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "physical_dimensinos")]
    pub physical_dimensions: Option<PhysicalDimensions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "physical_format")]
    pub physical_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "publish_country")]
    pub publish_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "publish_date")]
    pub publish_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_places: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publishers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dewey_decimal_class")]
    pub dewey_decimal_class: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lc_classifications")]
    pub lc_classifications: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "other_titles")]
    pub other_titles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source_records")]
    pub source_records: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "work_titles")]
    pub work_titles: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "table_of_contents")]
    pub table_of_contents: Option<Vec<TableOfContents>>, //https://openlibrary.org/books/OL26443497M/Anarchism_in_Latin_America
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_sentence")]
    pub first_sentence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Note>,
    pub revision: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "latest_revision")]
    pub latest_revision: Option<usize>,
    pub created: Created,
    #[serde(rename = "last_modified")]
    pub last_modified: LastModified,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isbn_invalid")]
    pub isbn_invalid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ia_box_id")]
    pub ia_box_id: Option<Vec<String>>,
}

impl Edition {
    pub fn get_author (&self) -> Vec<Authors> {
        self.authors.as_ref().unwrap().clone()
    }

    pub fn get_works_ids (&self) -> Vec<String> {
        let mut work_ids: Vec<String> = Vec::new();

        for works in self.works.iter() {
            work_ids.push(works.key.clone());
        }

        return work_ids;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notes {
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Note {
    NoteType(Notes),
    NoteString(String),
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtitle {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authors {
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Works {
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifiers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub librarything: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goodreads: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ByStatement {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weight {
    #[serde(rename = "type")]
    pub type_field: String,
    pub examples: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditionName {
    #[serde(rename = "type")]
    pub type_field: String,
    pub examples: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberOfPages {
    #[serde(rename = "type")]
    pub type_field: usize,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalDimensions {
    #[serde(rename = "type")]
    pub type_field: String,
    pub examples: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalFormat {
    #[serde(rename = "type")]
    pub type_field: String,
    pub examples: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishDate {
    #[serde(rename = "type")]
    pub type_field: String,
    pub examples: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableOfContents {
    #[serde(rename = "type")]
    pub type_field: Type,
    pub level: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagenum: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Revision {
    #[serde(rename = "type")]
    pub type_field: String,
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
