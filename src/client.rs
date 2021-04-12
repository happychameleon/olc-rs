use std::{fmt::format, fs::File};
use std::io::Write;
use surf::http::url;

use async_std::task::block_on;

use serde_json::{Value};

use crate::edition::Edition;
use crate::work::Work;
use crate::author::Author;

pub enum Paths {
    works,
    authors,
    books,
}

pub enum ApiType {
    Isbn,
    Works,
    Editions,
}

pub enum CoverSize {
    S,
    M,
    L,
}

pub enum CoverKey {
    ISBN,
    OCLC,
    LCCN,
    OLID,
    ID,
}

pub struct Client {
    base_url: String,
}

#[derive(Clone)]
pub struct Entity {
    olid: String,
    edition: Edition,
    work: Work,
    author: Author,
}

impl Client {
    pub fn new() -> Self {
        return Self{
            base_url: String::from("https://openlibrary.org"),
        };
    }

    pub async fn get_edition_by_isbn(){}

    pub async fn get_edition(&self, api_type: ApiType, search_key: String) -> Result<Edition, surf::Error> {
        let surf_client = surf::client().with(surf::middleware::Redirect::default());
        let uri = construct_uri(api_type, &search_key);
        println!("{}", uri);
        let req = surf_client.get(uri);

        let edition_json: Edition = block_on(surf_client.recv_json(req))?;
        
        Ok(edition_json)
    }

    pub async fn get_author(&self, ol_id: String) -> Result<Author, surf::Error> {
        let surf_client = surf::client().with(surf::middleware::Redirect::default());
        let author_uri = construct_ol_uri(&ol_id);
        println!("{}", author_uri);
        let author_uri_req = surf_client.get(author_uri);
        
        let author_json: Author = surf_client.recv_json(author_uri_req).await?;

        println!("is this empty?");
        
        Ok(author_json)
    }

    pub async fn get_work(&self, ol_id: String) -> Result<Work, surf::Error> {
        let surf_client = surf::client().with(surf::middleware::Redirect::default());
        let work_uri = construct_ol_uri(&ol_id);
        println!("{}", work_uri);
        let work_uri_req = surf_client.get(work_uri);

        let work_json: Work = surf_client.recv_json(work_uri_req).await?;

        Ok(work_json)
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
        let edition_json: Edition = block_on(self.get_edition(ApiType::Isbn, String::from(isbn)))?;
        let work_ids = edition_json.get_works_ids();

        let work_json: Work = block_on(self.get_work(work_ids[0].clone()))?;
        let author_ids = work_json.get_authors_ids();
        
        let author_json: Author = block_on(self.get_author(author_ids[0].clone()))?;

        let entity: Entity = Entity::new(process_olid_key(&edition_json.key), edition_json, work_json, author_json);

        Ok(entity)
    }


}



fn process_olid_key(json_olid: &String) -> String {
    let index = json_olid.rfind('/').unwrap();
    let (book, s_olid) = json_olid.split_at(index + 1);
    let olid = String::from(s_olid);

    return olid;
}

fn construct_ol_uri(ol_id: &str) -> String {
    let base_url = String::from("https://openlibrary.org");
    let url_end = String::from(".json");

    let uri = format!("{}{}{}", base_url, ol_id, url_end);

    return uri;
}

fn construct_uri(api_type: ApiType, isbn: &str) -> String {
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
    pub fn new(olid: String, edition: Edition, work: Work, author: Author) -> Self {
        return Self {
            olid: olid,
            edition: edition,
            work: work,
            author: author,
        };
    }
    pub fn get_olid(&self) -> String {
        self.olid.clone()
    }
}
