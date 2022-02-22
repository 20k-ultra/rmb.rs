use serde::{Deserialize, Serialize};

use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    todos: Vec<TodoMeta>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoMeta {
    task: Todo,
    created: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub task: String,
}

// TODO this parse is the same as the parse from other items..make generic parse
fn parse(json: &str) -> Result<TodoList, Box<dyn Error>> {
    // Read the JSON contents of the file as an instance of `TodoList`.
    let data = serde_json::from_str(json)?;
    // Return the data.
    Ok(data)
}

// TODO parse, modify (add) then return new String
pub fn add(todo: &Todo, data: &String) {
    println!("adding todo: {:?} to {data}", todo)
}

// TODO accept a writer to print to
pub fn print(data: &String) {
    let list = parse(data);
    println!("Todos: {:?}", list)
}
