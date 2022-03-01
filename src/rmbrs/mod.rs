use serde::{Deserialize, Serialize};
use serde_json::Result as SerdeResult;

use std::fmt;

mod link;
mod timer;
mod todo;

pub use link::Link;
pub use timer::Timer;
pub use todo::Todo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Remembers {
    pub links: link::LinkList,
    pub todos: todo::TodoList,
    pub timers: timer::TimerList,
}

impl Remembers {
    pub fn new() -> Remembers {
        Remembers {
            links: link::LinkList(vec![]),
            todos: todo::TodoList(vec![]),
            timers: timer::TimerList(vec![]),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl fmt::Display for Remembers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Remembers\n   Links\n{}\n   Todos\n{}\n   Timers\n{}",
            self.links, self.todos, self.timers
        )
    }
}

pub fn parse(data: &String) -> SerdeResult<Remembers> {
    serde_json::from_str(data)
}
