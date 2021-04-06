use std::{fmt::format, fs::File};
use std::io::Write;
use surf::http::url;

use serde_json::{Value};

use crate::edition::{Edition};

pub enum ApiType {
    Isbn,
    Works,
    Books,
}

pub enum CoverSize {
    S,
    M,
    L,
}

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

    pub async fn get_edition(&self, api_type: ApiType, search_key: String) -> Result<Edition, surf::Error> {
        let uri = construct_uri(api_type, &search_key);

        let surf_client = surf::client().with(surf::middleware::Redirect::default());

        let req = surf::get(uri);
        let edition_json: Edition = surf_client.recv_json(req).await?;

        Ok(edition_json)
    }

    pub async fn save_cover(&self, cover_size: CoverSize, path: String, isbn: String) -> Result<(), surf::Error>{
        let uri = construct_cover_uri(cover_size, &isbn);

        let surf_client = surf::client().with(surf::middleware::Redirect::default());
        let req = surf::get(uri);
        let cover_image = surf_client.recv_bytes(req).await?;
        assert!(cover_image.len() > 0);

        let path_str = format!("{}{}.jpg", path, isbn);
        let mut cover_file = File::create(path_str).unwrap();
        cover_file.write_all(&cover_image).unwrap();

        Ok(())
    }

    pub async fn entity_by_isbn(&self, isbn: &str) -> Result<Entity, surf::Error> {
        let uri = construct_uri(ApiType::Isbn, isbn);

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

fn construct_uri(api_type: ApiType, isbn: &str) -> String{

    
    let base_url = String::from("https://openlibrary.org");

    let isbn_path = "/isbn/";
    let url_end = ".json";
    let uri = format!("{}{}{}{}", base_url, isbn_path, isbn, url_end);

    println!("{}", uri);
    
    return uri;
}

// https://covers.openlibrary.org/b/isbn/9781849352826-L.jpg
// https://covers.openlibrary.org/b/$key/$value-$size.jpg
fn construct_cover_uri(cover_size: CoverSize, isbn: &str) -> String {
    let cover_url = String::from("https://covers.openlibrary.org/b");
    let cover_path = "/isbn/";
    let cover_end = ".jpg";

    let uri = format!("{}{}-{}{}", cover_url, cover_path, isbn, cover_end);

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
