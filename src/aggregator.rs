use std::error::Error;
use std::fmt;
use std::thread::JoinHandle;

use crate::sleeper::Sleeper;
use crate::waiter::WaiterStartResult;
use clap::{ArgMatches, Values};

#[derive(Debug)]
pub struct AggregateError(Vec<Box<dyn Error>>);

impl fmt::Display for AggregateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, error) in self.0.iter().enumerate() {
            write!(f, "{}", error)?;
            if index + 1 < self.0.len() {
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

impl Error for AggregateError {}

pub struct WaiterCollection<'s> {
    sleeper: Sleeper,
    errors: Vec<Box<dyn Error>>,
    handles: Vec<JoinHandle<()>>,
    matches: &'s ArgMatches<'s>,
}

impl<'s> WaiterCollection<'s> {
    pub fn new(sleeper: Sleeper, matches: &'s ArgMatches) -> Self {
        WaiterCollection {
            sleeper,
            matches,
            handles: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn add<F>(&mut self, argument_name: &str, builder: F)
    where
        F: Fn(&str, Sleeper) -> WaiterStartResult,
    {
        info!(
            "Adding [{}] waiters for argument type [{}].",
            self.matches.occurrences_of(argument_name),
            argument_name
        );

        let waiter_results: Vec<_> = self
            .matches
            .values_of(argument_name)
            .unwrap_or(Values::default())
            .map(|arg| builder(arg, self.sleeper.clone()))
            .collect();

        for result in waiter_results {
            match result {
                Ok(handle) => self.handles.push(handle),
                Err(error) => self.errors.push(error),
            }
        }
    }

    pub fn wait_for_all(self) -> Result<(), Box<dyn Error>> {
        if self.errors.is_empty() {
            info!("No error occurred during initialization. Starting wait now.");
            for waiter_handle in self.handles {
                let _ = waiter_handle.join();
            }
            Ok(())
        } else {
            error!("At least one error occurred. Not waiting.");
            Err(Box::new(AggregateError(self.errors)))
        }
    }
}
