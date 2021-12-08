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
    #[serde(default)]
    pub full_title: String,
    #[serde(default)]
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: Type,
    #[serde(default)]
    pub authors: Vec<Authors>,
    pub works: Vec<Works>,
    #[serde(default)]
    pub identifiers: Identifiers,
    #[serde(default)]
    #[serde(rename = "isbn_10")]
    pub isbn10: Vec<String>,
    #[serde(default)]
    #[serde(rename = "isbn_13")]
    pub isbn13: Vec<String>,
    #[serde(default)]
    pub lccn: Vec<String>,
    #[serde(default)]
    pub ocaid: String,
    #[serde(default)]
    #[serde(rename = "oclc_numbers")]
    pub oclc_numbers: Vec<String>,
    #[serde(default)]
    pub covers: Vec<isize>,
    #[serde(default)]
    pub links: String,
    #[serde(default)]
    pub languages: Vec<Language>,
    #[serde(default)]
    #[serde(rename = "by_statement")]
    pub by_statement: String,
    #[serde(default)]
    pub weight: Weight,
    #[serde(default)]
    #[serde(rename = "edition_name")]
    pub edition_name: EditionName,
    #[serde(default)]
    #[serde(rename = "number_of_pages")]
    pub number_of_pages: usize,
    #[serde(default)]
    pub pagination: String,
    #[serde(default)]
    #[serde(rename = "physical_dimensinos")]
    pub physical_dimensions: PhysicalDimensions,
    #[serde(default)]
    #[serde(rename = "physical_format")]
    pub physical_format: String,
    #[serde(default)]
    #[serde(rename = "publish_country")]
    pub publish_country: String,
    #[serde(default)]
    #[serde(rename = "publish_date")]
    pub publish_date: String,
    #[serde(default)]
    pub publish_places: Vec<String>,
    #[serde(default)]
    pub publishers: Vec<String>,
    #[serde(default)]
    pub contributions: Vec<String>,
    #[serde(default)]
    #[serde(rename = "dewey_decimal_class")]
    pub dewey_decimal_class: String,
    #[serde(default)]
    pub genres: Vec<String>,
    #[serde(default)]
    #[serde(rename = "lc_classifications")]
    pub lc_classifications: Vec<String>,
    #[serde(default)]
    #[serde(rename = "other_titles")]
    pub other_titles: Vec<String>,
    #[serde(default)]
    pub series: Vec<String>,
    #[serde(default)]
    #[serde(rename = "source_records")]
    pub source_records: Vec<String>,
    #[serde(default)]
    pub subjects: Vec<String>,
    #[serde(default)]
    #[serde(rename = "work_titles")]
    pub work_titles: String,
    #[serde(default)]
    #[serde(rename = "table_of_contents")]
    pub table_of_contents: TableOfContents,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    #[serde(rename = "first_sentence")]
    pub first_sentence: String,
    #[serde(default)]
    #[serde(deserialize_with = "string_or_struct")]
    pub notes: Notes,
    pub revision: usize,
    #[serde(default)]
    #[serde(rename = "latest_revision")]
    pub latest_revision: usize,
    pub created: Created,
    #[serde(rename = "last_modified")]
    pub last_modified: LastModified,
    #[serde(default)]
    #[serde(rename = "isbn_invalid")]
    pub isbn_invalid: Vec<String>,
    #[serde(default)]
    #[serde(rename = "ia_box_id")]
    pub ia_box_id: Vec<String>,
}

impl Edition {
    pub fn get_author (&self) -> Vec<Authors> {
        self.authors.clone()
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

// From https://serde.rs/string-or-struct.html
impl FromStr for Notes {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok( Notes {
            type_field: "/type/text".to_string(),
            value: s.to_string(),
        })
    }
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
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
    pub librarything: Vec<String>,
    pub goodreads: Vec<String>,
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
    pub type_field: String,
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
