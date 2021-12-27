use std::fmt;
use std::collections::BTreeMap as Map;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};
use void::Void;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Work {
    pub key: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<Authors>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub covers: Option<Vec<isize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>, //No Idea What this looks like in real
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lc_classifications")]
    pub lc_classifications: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_publish_date")]
    pub first_publish_date: Option<String>, //No Idea What this looks like in real
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<DescEnum>, //It looks like OL59863W a discription type instead of a string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub revision: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "latest_revision")]
    pub latest_revision: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Created>,
    #[serde(rename = "last_modified")]
    pub last_modified: LastModified,
}

impl Work {
    pub fn get_authors_ids (&self) -> Vec<String> {
        let mut author_ids: Vec<String> = Vec::new();

        let authors = self.authors.as_ref().unwrap();

        for authors in authors.iter() {
            author_ids.push(authors.author.key.clone());
        }

        return author_ids;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum DescEnum {
    DescString(String),
    DescType(Description),
}

// From https://serde.rs/string-or-struct.html
impl FromStr for Description {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok( Description {
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
pub struct Type {
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authors {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_field: Type,
    #[serde(default)]
    pub author: Author,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    url: String,
    #[serde(rename = "type")]
    pub type_field: Type,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Id {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcClassifications {
    #[serde(rename = "$ref")]
    pub ref_field: String,
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
