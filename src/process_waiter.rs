use crate::waiter::*;
use boolinator::Boolinator;
use std::thread::{self, JoinHandle};
use sysinfo::{Pid, System, SystemExt};
use std::fmt;

pub struct ProcessWaiter {
    pid: Pid,
    system: System,
    sleeper: Sleeper,
}

impl Waiter for ProcessWaiter {
    fn start(argument: &str, sleeper: Sleeper) -> Result<JoinHandle<()>, String> {
        info!("Initializing ProcessWaiter for PID [{}]", argument);
        let pid = argument
            .parse::<i32>()
            .map_err(|pid| format!("Pid [{}] is not a valid integer.", pid))?;
        let mut waiter = ProcessWaiter::new(pid, sleeper);
        let result = waiter.continue_waiting();
        error!("result for [{}] is [{}]", pid, result);
        result.ok_or(format!("Process with Pid [{}] does not exist.", pid))?;
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

#[derive(Clone, Copy,Debug)]
pub enum ProcessWaiterError {
    NotAValidInteger(&str),
    NoProcessExists(Pid),
}
