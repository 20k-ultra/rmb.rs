use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    todos: Vec<TodoMeta>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoMeta {
    task: Todo,
    created: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub task: String,
}

// TODO parse, modify (add) then return new String
pub fn add(todo: &Todo, data: &super::Remembers) -> String {
    println!("adding todo: {:?} to {:?}", todo, data);
    "Adding todo".to_string()
}

// TODO accept a writer to print to
pub fn print(data: &super::Remembers) {
    println!("Todos: {:?}", data.todos)
}

pub fn empty() -> TodoList {
    TodoList { todos: vec![] }
}
