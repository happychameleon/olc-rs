use std::{fmt::format, fs::File};
use std::io::Write;
use surf::http::url;

use async_std::task::block_on;

use serde_json::{Value};

use crate::edition::Edition;
use crate::work::Work;
use crate::author::Author;
use crate::entity::Entity;

pub enum Paths {
    Works,
    Authors,
    Books,
    ISBN,
}

pub enum QueryType {
    OLID(String),
    ISBN(String),
}

pub enum CoverSize {
    S,
    M,
    L,
}

pub enum CoverKey {
    ISBN(String),
    OCLC(String),
    LCCN(String),
    OLID(String),
    ID(String),
}

pub struct Client {
    base_url: String,
}

impl Client {
    pub fn new() -> Self {
        return Self{
            base_url: String::from("https://openlibrary.org"),
        };
    }

    pub async fn get_edition(&self, query_type: QueryType) -> Result<Edition, surf::Error> {
        match query_type {
            QueryType::OLID(key) => {
                let surf_client = surf::client().with(surf::middleware::Redirect::default());
                let uri = compose_ol_uri(QueryType::OLID(key));
                let req = surf_client.get(uri);

                let edition_json: Edition = block_on(surf_client.recv_json(req))?;
                
                Ok(edition_json)
            }
            QueryType::ISBN(key) => {
                let surf_client = surf::client().with(surf::middleware::Redirect::default());
                let uri = compose_ol_uri(QueryType::ISBN(key));
                let req = surf_client.get(uri);

                let edition_json: Edition = block_on(surf_client.recv_json(req))?;
                
                Ok(edition_json)
            }
        }        
    }

    pub async fn get_author(&self, query_type: QueryType) -> Result<Author, surf::Error> {
        match query_type {
            QueryType::OLID(key) => {
                let surf_client = surf::client().with(surf::middleware::Redirect::default());
                let author_uri = compose_ol_uri(QueryType::OLID(key));
                let author_uri_req = surf_client.get(author_uri);
                
                let author_json: Author = surf_client.recv_json(author_uri_req).await?;
                
                Ok(author_json)
            }
            QueryType::ISBN(key) => {
                let surf_client = surf::client().with(surf::middleware::Redirect::default());
                let uri = compose_ol_uri(QueryType::ISBN(key));
                let req = surf_client.get(uri);

                let edition_json: Edition = block_on(surf_client.recv_json(req))?;
                let work_ids = edition_json.get_works_ids();

                let work_uri = compose_ol_uri(QueryType::OLID(work_ids[0].clone()));
                let work_uri_req = surf_client.get(work_uri);

                let work_json: Work = block_on(surf_client.recv_json(work_uri_req))?;
                let author_ids = work_json.get_authors_ids();

                let author_uri = compose_ol_uri(QueryType::OLID(author_ids[0].clone()));
                let author_uri_req = surf_client.get(author_uri);
                
                let author_json: Author = surf_client.recv_json(author_uri_req).await?;
                
                Ok(author_json)
            }
        }        
    }

    pub async fn get_work(&self, query_type: QueryType) -> Result<Work, surf::Error> {
        match query_type {
            QueryType::OLID(key) => {
                let surf_client = surf::client().with(surf::middleware::Redirect::default());
                let work_uri = compose_ol_uri(QueryType::OLID(key));
                let work_uri_req = surf_client.get(work_uri);

                let work_json: Work = surf_client.recv_json(work_uri_req).await?;

                Ok(work_json)
            }
            QueryType::ISBN(key) => {
                let surf_client = surf::client().with(surf::middleware::Redirect::default());
                let uri = compose_ol_uri(QueryType::ISBN(key));
                let req = surf_client.get(uri);

                let edition_json: Edition = block_on(surf_client.recv_json(req))?;
                let work_ids = edition_json.get_works_ids();

                let work_uri = compose_ol_uri(QueryType::OLID(work_ids[0].clone()));
                let work_uri_req = surf_client.get(work_uri);

                let work_json: Work = surf_client.recv_json(work_uri_req).await?;

                Ok(work_json)
            }
        }
    }

    pub async fn save_cover(&self, cover_size: CoverSize, path: String, cover_key: CoverKey) -> Result<(), surf::Error>{
        let surf_client = surf::client().with(surf::middleware::Redirect::default());

        match cover_key {
            CoverKey::ISBN(isbn) => {
                let uri = construct_cover_uri(cover_size, &isbn);
                let req = surf::get(uri);
                let cover_image = surf_client.recv_bytes(req).await?;
                assert!(cover_image.len() > 0);

                let path_str = format!("{}", path);
                let mut cover_file = File::create(path_str)?;
                cover_file.write_all(&cover_image)?;
            }
            CoverKey::LCCN(isbn) => {}
            CoverKey::OCLC(isbn) => {}
            CoverKey::OLID(isbn) => {}
            CoverKey::ID(isbn) => {}           
        }
        
        Ok(())
    }

    pub async fn entity_by_isbn(self, isbn: &str) -> Result<Entity, surf::Error> {
        let edition_json: Edition = block_on(self.get_edition(QueryType::ISBN(String::from(isbn))))?;
        let work_ids = edition_json.get_works_ids();

        let work_json: Work = block_on(self.get_work(QueryType::OLID(work_ids[0].clone())))?;
        let author_ids = work_json.get_authors_ids();
        
        let author_json: Author = block_on(self.get_author(QueryType::OLID(author_ids[0].clone())))?;

        let entity: Entity = Entity::new(process_olid_key(&edition_json.key), edition_json, work_json, author_json);

        Ok(entity)
    }


}

fn process_olid_key(json_olid: &str) -> String {
    let index = json_olid.rfind('/');

    let olid = match index {
        Some(index) => {
            let (book, s_olid) = json_olid.split_at(index + 1);
            String::from(s_olid)
        }
        None => String::from(json_olid),
    };

    return olid;
}

fn compose_ol_uri (query_type: QueryType) -> String {
    let base_url = String::from("https://openlibrary.org");
    let url_end = String::from(".json");

    let uri = match query_type {
        QueryType::OLID(key) => format!("{}/{}/{}{}", base_url, "q", process_olid_key(&key), url_end),
        QueryType::ISBN(key) => format!("{}/{}/{}{}", base_url, "isbn", key, url_end),
    };

    return uri;
}

// https://covers.openlibrary.org/b/isbn/9781849352826-L.jpg
// https://covers.openlibrary.org/b/$key/$value-$size.jpg
fn construct_cover_uri(cover_size: CoverSize, isbn: &str) -> String {
    let cover_size = match cover_size {
        CoverSize::L => "L",
        CoverSize::M => "M",
        CoverSize::S => "S",
    };
    
    let cover_url = String::from("https://covers.openlibrary.org/b");
    let cover_path = "/isbn/";
    let cover_end = ".jpg";

    let uri = format!("{}{}{}-{}{}", cover_url, cover_path, isbn, cover_size, cover_end);

    return uri;
}
