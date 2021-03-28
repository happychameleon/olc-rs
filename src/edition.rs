use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Edition {
    pub other_titles: Vec<String>,//["El anarquismo en Am\u00e9rica latina"],
    pub publishers: Vec<String>,//["AK Press"],
    number_of_pages: usize,
    contributors: Vec<Contributor>, //[{"role": "Translator", "name": "Gabriel Palmer-Fern\u00e1ndez"}], 
    covers: Vec<usize>, //[8162971],
    physical_format: String,
    last_modified: ModEvent, //{"type": "/type/datetime", "value": "2020-08-22T12:23:31.164154"},
    latest_revision: usize, 
    pub key: String,
    publish_places: Vec<String>, //["Chico, USA"], 
    isbn_13: Vec<String>, //["9781849352826"], 
    classifications: Classifications, //{"lccn_permalink": ["https://lccn.loc.gov/2017936242"]}, 
    source_records: Vec<String>, //["marc:marc_openlibraries_sanfranciscopubliclibrary/sfpl_chq_2018_12_24_run06.mrc:132376748:2932", "amazon:1849352828"], 
    title: String,
    lccn: Vec<String>, //["2017936242"], 
    translation_of: String,
    identifiers: Identifiers,//{"google": ["qOdKAQAACAAJ"], "librarything": ["21118985"], "goodreads": ["34381034"]}, 
    created: CreationEvent, //{"type": "/type/datetime", "value": "2018-04-22T19:38:47.930796"}, 
    languages: Vec<Language>,//[{"key": "/languages/eng"}], 
    local_id: Vec<String>, //["urn:sfpl:31223123562416"], 
    publish_date: String,
    oclc_numbers: Vec<String>, //["974677647"], 
    works: Vec<Work>, //[{"key": "/works/OL17859861W"}],
    #[serde(rename = "type")]
    pub t_ype: Kind, //{"key": "/type/edition"}, 
    revision: usize, //7
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Kind {
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct Work {
    key: String,
}

#[derive(Serialize, Deserialize)]
pub struct Language {
    key: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreationEvent {
    #[serde(rename = "type")]
    t_ype: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Identifiers {
    google: Vec<String>,
    librarything: Vec<String>,
    goodreads: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Classifications {
    lccn_permalink: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ModEvent {
    #[serde(rename = "type")]
    t_ype: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Contributor{
    role: String,
    name: String,
}
