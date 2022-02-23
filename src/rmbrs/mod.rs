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

pub fn parse(data: &String) -> Result<Remembers> {
    serde_json::from_str(data)
}

// TODO Make a minimal output format to use for printing
// TODO pass a writer to each mod (link, timer, todo) so it can print
pub fn print(data: &Remembers) {
    println!("Printing: {:?}", data)
}

pub fn empty() -> Remembers {
    Remembers {
        links: link::empty(),
        todos: todo::empty(),
        timers: timer::empty(),
    }
}
