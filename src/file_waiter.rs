use crate::sleeper::Sleeper;
use crate::waiter::*;
use boolinator::Boolinator;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use std::thread;

pub struct FileWaiter {
    file_path: PathBuf,
    sleeper: Sleeper,
}

impl Waiter for FileWaiter {
    fn start(argument: &str, sleeper: Sleeper) -> WaiterStartResult {
        info!("Initializing FileWaiter for file [{}]", argument);
        let fp = PathBuf::from(argument);
        let mut waiter = FileWaiter::new(fp.clone(), sleeper);
        waiter
            .continue_waiting()
            .ok_or(Box::new(FileExistsError::new(fp)))?;
        info!("Starting thread for [{:?}].", waiter);
        Ok(thread::spawn(move || waiter.run()))
    }

    fn get_sleeper(&self) -> &Sleeper {
        &self.sleeper
    }

    fn continue_waiting(&mut self) -> bool {
        debug!("Checking if file [{}] exists", self.file_path.display());
        !self.file_path.exists()
    }
}

impl FileWaiter {
    fn new(file_path: PathBuf, sleeper: Sleeper) -> Self {
        FileWaiter { file_path, sleeper }
    }
}

impl fmt::Debug for FileWaiter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileWaiter")
            .field("file_path", &self.file_path)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct FileExistsError {
    file_path: PathBuf,
}

impl FileExistsError {
    fn new(file_path: PathBuf) -> Self {
        FileExistsError { file_path }
    }
}

impl fmt::Display for FileExistsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File [{}] already exists.", self.file_path.display())
    }
}

impl Error for FileExistsError {}
