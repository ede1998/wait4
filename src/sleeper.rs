use clap::ArgMatches;
use std::error::Error;
use std::fmt;
use std::thread;
use std::time::Duration;

const DEFAULT_POLL_FREQUENCY_STR: &'static str = "1000";

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

    pub fn from(matches: &ArgMatches) -> Result<Sleeper, Box<dyn Error>> {
        let ms_str = matches
            .value_of("milliseconds")
            .unwrap_or(DEFAULT_POLL_FREQUENCY_STR)
            .to_owned();
        let ms = ms_str
            .parse::<u64>()
            .map_err(|_| Box::new(SleeperParseError(ms_str)))?;
        Ok(Sleeper::new(ms))
    }

    pub fn default_poll_frequency() -> &'static str {
        DEFAULT_POLL_FREQUENCY_STR
    }
}

#[derive(Clone, Debug)]
pub struct SleeperParseError(String);

impl fmt::Display for SleeperParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Poll frequency [{}] is not a valid integer.", self.0)
    }
}

impl Error for SleeperParseError {}
