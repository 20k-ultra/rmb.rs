use serde::{Deserialize, Serialize};

use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct TimerList {
    timers: Vec<TimerMeta>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TimerMeta {
    timer: Timer,
    created: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timer {
    pub what: String,
    pub when: String,
}

// TODO this parse is the same as the parse from other items..make generic parse
fn parse(json: &str) -> Result<TimerList, Box<dyn Error>> {
    // Read the JSON contents of the file as an instance of `TimerList`.
    let data = serde_json::from_str(json)?;
    // Return the data.
    Ok(data)
}

// TODO parse, modify (add) then return new String
pub fn add(timer: &Timer, data: &String) -> String {
    println!(
        "Adding a timer to {} at {} in {data}",
        timer.what, timer.when
    );
    "Adding timer".to_string()
}

// TODO accept a writer to print to
pub fn print(data: &String) {
    let list = parse(data);
    println!("Timers: {:?}", list)
}
