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
    #[serde(default)]
    #[serde(rename = "alternate_names")]
    pub alternate_names: Vec<String>,
    #[serde(default)]
    #[serde(deserialize_with = "string_or_struct")]
    pub bio: Bio,
    #[serde(default)]
    #[serde(rename = "birth_date")]
    pub birth_date: String,
    #[serde(default)]
    #[serde(rename = "death_date")]
    pub death_date: String,
    #[serde(default)]
    pub location: Location, //No Idea What this looks like in real
    #[serde(default)]
    pub date: Date, //No Idea What this looks like in real
    #[serde(default)]
    #[serde(rename = "entity_type")]
    pub entity_type: EntityType, //No Idea What this looks like in real
    #[serde(default)]
    #[serde(rename = "fuller_name")]
    pub fuller_name: FullerName, //No Idea What this looks like in real
    #[serde(default)]
    #[serde(rename = "personal_name")]
    pub personal_name: String,
    #[serde(default)]
    pub title: String, //According to https://openlibrary.org/authors/OL29497A.json looks like a String
    #[serde(default)]
    pub photos: Vec<usize>,
    #[serde(default)]
    pub links: Vec<Link>,
    #[serde(default)]
    #[serde(rename = "remote_ids")]
    pub remote_ids: RemoteIds,
    #[serde(default)]
    pub wikipedia: Wikipedia, //No Idea What this looks like in real
    pub revision: usize,
    #[serde(default)]
    #[serde(rename = "latest_revision")]
    pub latest_revision: usize,
    #[serde(default)]
    pub created: Created,
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

// From https://serde.rs/string-or-struct.html
impl FromStr for Bio {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok( Bio {
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
