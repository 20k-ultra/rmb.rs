use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Timer {
    what: String,
    when: String,
    created: String,
}

impl Timer {
    pub fn new(what: String, when: String) -> Timer {
        Timer {
            what,
            when,
            created: String::from("now") // TODO make this a timestamp
        }
    }

    // TODO accept a writer to print to
    pub fn print(&self) {
        println!("Timer: {:?}", self)
    }
}
