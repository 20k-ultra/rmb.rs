use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    todos: Vec<Todo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    task: String,
    created: String,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { todos: vec![] }
    }

    // TODO accept a writer to print to
    pub fn print(&self) {
        println!("Todos: {:?}", self.todos)
    }

    pub fn add<'a>(&'a mut self, task: String) -> &'a mut TodoList {
        self.todos.push(Todo {
            task,
            created: String::from("now"), // TODO make this a timestamp
        });
        self
    }
}
