use serde::{Deserialize, Serialize};

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

// TODO parse, modify (add) then return new String
pub fn add(timer: &Timer, data: &super::Remembers) -> String {
    println!(
        "Adding a timer to {} at {} in {:?}",
        timer.what, timer.when, data
    );
    "Adding timer".to_string()
}

// TODO accept a writer to print to
pub fn print(data: &super::Remembers) {
    println!("Timers: {:?}", data.timers)
}
