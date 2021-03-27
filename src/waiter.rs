use std::error::Error;
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub trait Waiter {
    fn start(argument: &str, sleeper: Sleeper) -> Result<JoinHandle<()>, Box<dyn Error>>;
    fn continue_waiting(&mut self) -> bool;

    fn run(&mut self)
    where
        Self: std::fmt::Debug,
    {
        while self.continue_waiting() {
            self.get_sleeper().sleep();
        }
        info!("Finished waiting for [{:?}].", self);
    }

    fn get_sleeper(&self) -> &Sleeper;
}

#[derive(Debug, Copy, Clone)]
pub struct Sleeper {
    duration: Duration,
}

impl Sleeper {
    pub fn new(ms: u64) -> Self {
        Sleeper {
            duration: Duration::from_millis(ms),
        }
    }

    pub fn sleep(&self) {
        thread::sleep(self.duration);
    }
}
