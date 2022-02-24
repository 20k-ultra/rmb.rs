use serde::{Deserialize, Serialize};

use std::fmt;

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
            created: String::from("now"), // TODO make this a timestamp
        }
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO figure out how to print this..
        write!(f, "[{} Timer]", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimerList(pub Vec<Timer>);

impl TimerList {
    pub fn push<'a>(&'a mut self, what: String, when: String) -> &'a mut TimerList {
        self.0.push(Timer::new(what, when));
        self
    }
}

impl fmt::Display for TimerList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO make this pretty
        write!(f, "[{:?}]", self)
    }
}
