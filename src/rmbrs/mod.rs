use serde::{Deserialize, Serialize};
use serde_json::Result;

pub mod link;
pub mod timer;
pub mod todo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Remembers {
    pub links: Vec<link::Link>,
    pub todos: Vec<todo::Todo>,
    pub timers: Vec<timer::Timer>,
}

pub fn parse(data: &String) -> Result<Remembers> {
    serde_json::from_str(data)
}

// TODO Make a minimal output format to use for printing
// TODO pass a writer to each mod (link, timer, todo) so it can print
pub fn print(data: &Remembers) {
    println!("Printing: {:?}", data)
}
