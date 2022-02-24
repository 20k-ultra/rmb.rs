use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    task: String,
    created: String,
}

impl Todo {
    pub fn new(task: String) -> Todo {
        Todo {
            task,
            created: String::from("now"), // TODO make this a timestamp
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO figure out how to print this..
        write!(f, "[{} Todo]", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList(pub Vec<Todo>);

impl TodoList {
    pub fn push<'a>(&'a mut self, task: String) -> &'a mut TodoList {
        self.0.push(Todo::new(task));
        self
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO make this pretty
        write!(f, "[{:?}]", self)
    }
}
