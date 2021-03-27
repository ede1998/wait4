use crate::sleeper::Sleeper;
use std::error::Error;
use std::thread::JoinHandle;

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
