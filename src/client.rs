use std::fs::File;
use std::io::Read;
use surf::http::url;

use serde_json::{Value};

use crate::edition::{Edition};

pub struct Client {
    base_url: String,
}

#[derive(Clone)]
pub struct Entity {
    olid: String,
}

impl Client {
    pub fn new() -> Self {
        return Self{
            base_url: String::from("https://openlibrary.org"),
        };
    }

    pub async fn entity_by_isbn(&self, isbn: &str) -> Result<Entity, surf::Error> {
        
        let mut edition_schema = File::open("src/schemata/edition.schema.json").unwrap();
        let mut json_edition_string = String::new();
        edition_schema.read_to_string(&mut json_edition_string).unwrap();

        let json: Value = serde_json::from_str(&json_edition_string)?;

        let uri = construct_uri(isbn);

        let surf_client = surf::client().with(surf::middleware::Redirect::default());

        let req = surf::get(uri);
        let edition_json: Edition = surf_client.recv_json(req).await?;

        let work: Entity = Entity::new(process_olid_key(&edition_json.key));

        Ok(work)
    }
}

fn process_olid_key(json_olid: &String) -> String {
    let index = json_olid.rfind('/').unwrap();
    let (book, s_olid) = json_olid.split_at(index + 1);
    let olid = String::from(s_olid);

    return olid;
}

fn construct_uri(isbn: &str) -> String{
    let isbn_path = "/isbn/";
    let base_url = String::from("https://openlibrary.org");
    let url_end = ".json";
    let uri = format!("{}{}{}{}", base_url, isbn_path, isbn, url_end);
    
    return uri;
}

impl Entity {
    pub fn new(olid: String) -> Self {
        return Self {
            olid: olid,
        };
    }
    pub fn get_olid(&self) -> String {
        self.olid.clone()
    }
}
