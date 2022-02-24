use serde::{Deserialize, Serialize};
use serde_json::Result as SerdeResult;

mod link;
mod todo;
mod timer;

pub use link::Link;
pub use todo::Todo;
pub use timer::Timer;

#[derive(Serialize, Deserialize, Debug)]
pub struct Remembers {
    pub links: Vec<link::Link>,
    pub todos: Vec<todo::Todo>,
    pub timers: Vec<timer::Timer>,
}

impl Remembers {
    pub fn new() -> Remembers {
        Remembers {
            links: vec![],
            todos: vec![],
            timers: vec![],
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn print(&self) {
        // TODO Make a minimal output format to use for printing
        // TODO pass a writer to each mod (link, timer, todo) so it can print
        println!("Printing: {:?}", self);
    }
}

pub fn parse(data: &String) -> SerdeResult<Remembers> {
    serde_json::from_str(data)
}
