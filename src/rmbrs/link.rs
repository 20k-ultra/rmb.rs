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
    url: String,
}

impl LinkList {
    pub fn new() -> LinkList {
        LinkList { links: vec![] }
    }

    // TODO accept a writer to print to
    pub fn print(&self) {
        println!("Links: {:?}", self.links)
    }

    pub fn add<'a>(&'a mut self, url: String) -> &'a mut LinkList {
        self.links.push(LinkMeta {
            link: Link { url },
            created: String::from("now"), // TODO make this a timestamp
        });
        self
    }
}
