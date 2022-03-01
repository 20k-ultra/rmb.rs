use serde::{Deserialize, Serialize};
use serde_json::{Result, Error};

use std::str::FromStr;

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

}

impl ToString for Remembers {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl FromStr for Remembers {
    type Err = Error;
    fn from_str(data: &str) -> Result<Remembers> {
        serde_json::from_str(data)
    }
}
