use serde::{Deserialize, Serialize};

pub mod link;
pub mod timer;
pub mod todo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Remembers {
    links: Vec<link::Link>,
    todos: Vec<todo::Todo>,
    timers: Vec<timer::Timer>,
}

// TODO Make a minimal output format to use for printing
// TODO pass a writer to each mod (link, timer, todo) so it can print
pub fn print(data: &String) {
    println!("Printing: {data}")
}
