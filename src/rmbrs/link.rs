use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub url: String,
    created: String,
}

impl Link {
    pub fn new(url: String) -> Link {
        Link {
            url,
            created: String::from("now") // TODO make this a timestamp
        }
    }

    // TODO accept a writer to print to
    pub fn print(&self) {
        println!("Link: {:?}", self)
    }
}
