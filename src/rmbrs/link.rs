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
            created: String::from("now"), // TODO make this a timestamp
        }
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO figure out how to print this..
        write!(f, "[{} Link]", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkList(pub Vec<Link>);

impl LinkList {
    pub fn push<'a>(&'a mut self, url: String) -> &'a mut LinkList {
        self.0.push(Link::new(url));
        self
    }
}

impl fmt::Display for LinkList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO make this pretty
        write!(f, "[{:?}]", self)
    }
}
