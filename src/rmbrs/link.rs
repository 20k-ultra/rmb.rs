use serde::{Deserialize, Serialize};

use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct LinkList {
    links: Vec<LinkMeta>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LinkMeta {
    link: Link,
    created: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub url: String,
}

// TODO this parse is the same as the parse from other items..make generic parse
fn parse(json: &str) -> Result<LinkList, Box<dyn Error>> {
    // Read the JSON contents of the file as an instance of `LinkList`.
    let data = serde_json::from_str(json)?;
    // Return the data.
    Ok(data)
}

// TODO parse, modify (add) then return new String
pub fn add(link: &Link, data: &String) -> String {
    println!("adding link: {:?} to data: {data}", link);
    "New data".to_string()
}

// TODO accept a writer to print to
pub fn print(data: &String) {
    let list = parse(data);
    println!("Links: {:?}", list)
}
