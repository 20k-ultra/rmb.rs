use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TimerList {
    timers: Vec<Timer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timer {
    what: String,
    when: String,
    created: String,
}

impl TimerList {
    pub fn new() -> Self {
        TimerList { timers: vec![] }
    }

    // TODO accept a writer to print to
    pub fn print(&self) {
        println!("Timers: {:?}", self.timers)
    }

    pub fn add<'a>(&'a mut self, what: String, when: String) -> &'a mut TimerList {
        self.timers.push(Timer {
            what,
            when,
            created: String::from("now"), // TODO make this a timestamp
        });
        self
    }
}
