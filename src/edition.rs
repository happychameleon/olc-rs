#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edition {

    pub key: String,
    pub title: String,
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
    pub notes: String,
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
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
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
