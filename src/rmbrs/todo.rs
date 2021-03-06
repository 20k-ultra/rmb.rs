use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub task: String,
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

impl ToString for Todo {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList(pub Vec<Todo>);

impl TodoList {
    pub fn add(&mut self, task: String) -> &mut TodoList {
        self.0.push(Todo::new(task));
        self
    }

    pub fn remove(&mut self, index: usize) -> &mut TodoList {
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
