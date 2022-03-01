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
            created: chrono::Utc::now().to_rfc2822(),
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
    pub fn add<'a>(&'a mut self, what: String, when: String) -> &'a mut TimerList {
        self.0.push(Timer::new(what, when));
        self
    }

    pub fn remove<'a>(&'a mut self, index: usize) -> &'a mut TimerList {
        self.0.remove(index);
        self
    }
}

impl fmt::Display for TimerList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .enumerate()
                .fold(String::from(""), |accum, (index, timer)| {
                    match accum.is_empty() {
                        true => format!("[{}] {} in {}", index + 1, timer.what, timer.when),
                        false => {
                            format!("{accum}\n[{}] {} in {}", index + 1, timer.what, timer.when)
                        }
                    }
                })
        )
    }
}
