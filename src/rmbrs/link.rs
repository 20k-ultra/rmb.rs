use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkList {
    links: Vec<LinkMeta>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkMeta {
    link: Link,
    created: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub url: String,
}

// TODO parse, modify (add) then return new String
pub fn add(link: &Link, data: &super::Remembers) -> String {
    println!("adding link: {:?} to data: {:?}", link, data);
    "New data".to_string()
}

// TODO accept a writer to print to
pub fn print(data: &super::Remembers) {
    println!("Links: {:?}", data.links)
}

pub fn empty() -> LinkList {
    LinkList { links: vec![] }
}
