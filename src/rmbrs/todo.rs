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
            created: chrono::Utc::now().to_rfc2822(),
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO figure out how to print this..
        write!(f, "{}", self.task)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList(pub Vec<Todo>);

impl TodoList {
    pub fn add<'a>(&'a mut self, task: String) -> &'a mut TodoList {
        self.0.push(Todo::new(task));
        self
    }

    pub fn remove<'a>(&'a mut self, index: usize) -> &'a mut TodoList {
        self.0.remove(index);
        self
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .enumerate()
                .fold(String::from(""), |accum, (index, todo)| {
                    match accum.is_empty() {
                        true => format!("[{}] {}", index + 1, todo.task),
                        false => format!("{accum}\n[{}] {}", index + 1, todo.task),
                    }
                })
        )
    }
}
