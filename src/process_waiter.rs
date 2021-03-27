use crate::sleeper::Sleeper;
use crate::waiter::*;
use boolinator::Boolinator;
use std::error::Error;
use std::fmt;
use std::thread::{self, JoinHandle};
use sysinfo::{Pid, System, SystemExt};

pub struct ProcessWaiter {
    pid: Pid,
    system: System,
    sleeper: Sleeper,
}

impl Waiter for ProcessWaiter {
    fn start(argument: &str, sleeper: Sleeper) -> Result<JoinHandle<()>, Box<dyn Error>> {
        info!("Initializing ProcessWaiter for PID [{}]", argument);
        let pid = argument
            .parse::<i32>()
            .map_err(|_| Box::new(ProcessWaiterError::NotAValidInteger(argument.to_string())))?;
        let mut waiter = ProcessWaiter::new(pid, sleeper);
        waiter
            .continue_waiting()
            .ok_or(Box::new(ProcessWaiterError::NoProcessExists(pid)))?;
        info!("Starting thread for ProcessWaiter [{:?}].", waiter);
        Ok(thread::spawn(move || waiter.run()))
    }

    fn get_sleeper(&self) -> &Sleeper {
        &self.sleeper
    }

    fn continue_waiting(&mut self) -> bool {
        debug!("Checking if process with pid [{}] still exists.", self.pid);
        self.system.refresh_process(self.pid)
    }
}

impl ProcessWaiter {
    fn new(pid: i32, sleeper: Sleeper) -> Self {
        let mut system = System::new();
        system.refresh_processes();
        ProcessWaiter {
            system,
            pid,
            sleeper,
        }
    }
}

impl fmt::Debug for ProcessWaiter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProcessWaiter")
            .field("pid", &self.pid)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub enum ProcessWaiterError {
    NotAValidInteger(String),
    NoProcessExists(i32),
}

impl fmt::Display for ProcessWaiterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ProcessWaiterError::*;
        match self {
            NotAValidInteger(value) => write!(f, "Pid [{}] is not a valid integer.", value),
            NoProcessExists(pid) => write!(f, "Process with Pid [{}] does not exist.", pid),
        }
    }
}

impl Error for ProcessWaiterError {}
