use crate::sleeper::Sleeper;
use crate::waiter::*;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::thread;

pub struct SteamWaiter {
    file: File,
    sleeper: Sleeper,
    app_id: String,
}

fn create_download_finished_log_line(app_id: &str) -> String {
    format!("AppID {} state changed : Fully Installed", app_id)
}

const DEFAULT_LOG_FILE_PATH: &str = "~/.local/share/Steam/logs/content_log.txt";
const ENV_VAR_LOG_FILE: &str = "STEAM_LOG_FILE";

fn expand_default_path() -> PathBuf {
    let default_log_file_path = PathBuf::from(DEFAULT_LOG_FILE_PATH.replace("~", "."));
    match dirs::home_dir() {
        Some(home) => home.join(default_log_file_path),
        None => PathBuf::new(),
    }
}

impl Waiter for SteamWaiter {
    fn start(argument: &str, sleeper: Sleeper) -> WaiterStartResult {
        info!("Initializing SteamWaiter for app id [{}]", argument);
        let fp = env::var(ENV_VAR_LOG_FILE)
            .map(|fp| PathBuf::from(fp))
            .unwrap_or_else(|_| expand_default_path());
        debug!("Log file path is [{}]", fp.display());
        let mut waiter = SteamWaiter::try_new(&fp, argument, sleeper)
            .ok_or(Box::new(SteamWaiterError::new(fp)))?;

        info!("Starting thread for [{:?}].", waiter);
        Ok(thread::spawn(move || waiter.run()))
    }

    fn get_sleeper(&self) -> &Sleeper {
        &self.sleeper
    }

    fn continue_waiting(&mut self) -> bool {
        debug!("Checking if download finished for app id [{}]", self.app_id);
        let mut lines = String::new();
        let result = self.file.read_to_string(&mut lines);
        if let Err(e) = result {
            error!("Error occurred while trying to read: [{}]", e);
            return true;
        }
        let log_line = create_download_finished_log_line(&self.app_id);
        !lines.contains(&log_line)
    }
}

impl SteamWaiter {
    fn try_new(file_path: &Path, app_id: &str, sleeper: Sleeper) -> Option<Self> {
        let mut file = File::open(file_path).ok()?;
        let app_id = app_id.to_owned();
        file.seek(SeekFrom::End(0)).ok()?;
        Some(SteamWaiter {
            file,
            sleeper,
            app_id,
        })
    }

    pub fn default_log_file_path() -> &'static str {
        DEFAULT_LOG_FILE_PATH
    }

    pub fn env_var_log_file() -> &'static str {
        ENV_VAR_LOG_FILE
    }
}

impl fmt::Debug for SteamWaiter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SteamWaiter")
            .field("app_id", &self.app_id)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct SteamWaiterError {
    file_path: PathBuf,
}

impl SteamWaiterError {
    fn new(file_path: PathBuf) -> Self {
        SteamWaiterError { file_path }
    }
}

impl fmt::Display for SteamWaiterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error reading file [{}].", self.file_path.display())
    }
}

impl Error for SteamWaiterError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_tilde_in_path() {
        let result = expand_default_path();

        println!("Path: {}", result.display());
        assert!(result.starts_with(Path::new("/home")));
        assert!(result.ends_with(Path::new(".local/share/Steam/logs/content_log.txt")));
    }
}
