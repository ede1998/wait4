#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgMatches, Values};
use itertools::{Either, Itertools};
use std::error::Error;
use std::fmt;

use crate::process_waiter::ProcessWaiter;
use crate::waiter::*;

mod process_waiter;
mod sleeper;
mod waiter;

fn main() {
    let result = run();
    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let matches = get_matches();

    let sleeper = Sleeper::from(&matches)?;
    let pids: Vec<_> = matches
        .values_of("pid")
        .unwrap_or(Values::default())
        .map(|pid| ProcessWaiter::start(pid, sleeper))
        .collect();

    let partition_result = |r| match r {
        Ok(v) => Either::Left(v),
        Err(v) => Either::Right(v),
    };
    let (waiter_handles, errors): (Vec<_>, Vec<_>) =
        pids.into_iter().partition_map(partition_result);

    if !errors.is_empty() {
        return Err(Box::new(AggregateError(errors)));
    }

    for waiter_handle in waiter_handles {
        let _ = waiter_handle.join();
    }

    Ok(())
}

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

fn get_matches() -> ArgMatches<'static> {
    App::new("wait4")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Wait for events before terminating the program.")
        .arg(
            Arg::with_name("pid")
                .long("pid")
                .short("p")
                .help("Wait till process with the given pid terminates.")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("milliseconds")
                .long("frequency")
                .short("F")
                .help("Configures the poll frequency in milliseconds.")
                .default_value(Sleeper::default_poll_frequency())
                .takes_value(true),
        )
        .get_matches()
}
