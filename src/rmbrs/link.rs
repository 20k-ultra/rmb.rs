use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub url: String,
    created: String,
}

impl Link {
    pub fn new(url: String) -> Link {
        Link {
            url,
            created: chrono::Utc::now().to_rfc2822(),
        }
    }
}

impl ToString for Link {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkList(pub Vec<Link>);

impl LinkList {
    pub fn add<'a>(&'a mut self, url: String) -> &'a mut LinkList {
        self.0.push(Link::new(url));
        self
    }

    pub fn remove<'a>(&'a mut self, index: usize) -> &'a mut LinkList {
        self.0.remove(index);
        self
    }
}

impl fmt::Display for LinkList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .enumerate()
                .fold(String::from(""), |accum, (index, link)| {
                    match accum.is_empty() {
                        true => format!("[{}] {}", index + 1, link.url),
                        false => format!("{accum}\n[{}] {}", index + 1, link.url),
                    }
                })
        )
    }
}
