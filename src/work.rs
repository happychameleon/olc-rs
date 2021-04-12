
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Work {
    pub key: String,
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    #[serde(rename = "type")]
    pub type_field: Type,
    #[serde(default)]
    pub authors: Vec<Authors>,
    #[serde(default)]
    pub covers: Vec<usize>,
    #[serde(default)]
    pub links: Vec<Link>,
    #[serde(default)]
    pub id: Id, //No Idea What this looks like in real
    #[serde(default)]
    #[serde(rename = "lc_classifications")]
    pub lc_classifications: LcClassifications, //No Idea What this looks like in real
    #[serde(default)]
    pub subjects: Vec<String>,
    #[serde(default)]
    #[serde(rename = "first_publish_date")]
    pub first_publish_date: String, //No Idea What this looks like in real
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub notes: Notes, //No Idea What this looks like in real
    pub revision: usize,
    #[serde(default)]
    #[serde(rename = "latest_revision")]
    pub latest_revision: usize,
    #[serde(default)]
    pub created: Created,
    #[serde(rename = "last_modified")]
    pub last_modified: LastModified,
}

impl Work {
    pub fn get_authors_ids (&self) -> Vec<String> {
        let mut author_ids: Vec<String> = Vec::new();

        for authors in self.authors.iter() {
            author_ids.push(authors.author.key.clone());
        }

        return author_ids;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authors {
    #[serde(rename = "type")]
    pub type_field: Type,
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
pub struct Notes {
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
