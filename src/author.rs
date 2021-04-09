
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
    pub title: Title, //No Idea What this looks like in real
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
