use crate::edition::Edition;
use crate::work::Work;
use crate::author::Author;

#[derive(Clone)]
pub struct Entity {
    olid: String,
    edition: Edition,
    work: Work,
    author: Author,
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

    pub fn get_author(&self) -> Author {
        self.author.clone()
    }

    pub fn get_author_name(&self) -> String {
        self.author.name.clone()
    }

    pub fn get_work(&self) -> Work {
        self.work.clone()
    }

    pub fn get_work_title(&self) -> String {
        self.work.title.clone()
    }

    pub fn get_edition(&self) -> Edition {
        self.edition.clone()
    }
}
