use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    task: String,
    created: String,
}

impl Todo {
    pub fn new(task: String) -> Todo {
        Todo {
            task,
            created: String::from("now") // TODO make this a timestamp
        }
    }

    // TODO accept a writer to print to
    pub fn print(&self) {
        println!("Todo: {:?}", self)
    }
}
