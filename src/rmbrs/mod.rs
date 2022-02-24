use serde::{Deserialize, Serialize};
use serde_json::Result;

pub mod link;
pub mod timer;
pub mod todo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Remembers {
    pub links: link::LinkList,
    pub todos: todo::TodoList,
    pub timers: timer::TimerList,
}

impl Remembers {
    pub fn new() -> Remembers {
        Remembers {
            links: link::LinkList::new(),
            todos: todo::TodoList::new(),
            timers: timer::TimerList::new(),
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

pub fn parse(data: &String) -> Result<Remembers> {
    serde_json::from_str(data)
}
