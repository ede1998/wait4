#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;

use crate::process_waiter::ProcessWaiter;
use crate::waiter::*;
use clap::{App, Arg, ArgMatches};

mod process_waiter;
mod waiter;

const DEFAULT_POLL_FREQUENCY:u64 = 1000;

fn main() {
}

fn run() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let matches = get_matches();

    let ms = matches
        .value_of("milliseconds")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let sleeper = Sleeper::new(ms);
    let pids: Vec<_> = matches
        .values_of("pid")
        .unwrap()
        .map(|pid| ProcessWaiter::start(pid, sleeper))
        .collect();

    let errors = pids.iter().filter_map(|x| x.as_ref().err());

    for error in errors {
        eprintln!("{}", error);
    }

    let waiter_handles: Vec<_> = pids.into_iter().filter_map(|x| x.ok()).collect();

    for waiter_handle in waiter_handles {
        let _ = waiter_handle.join();
    }
}

fn make_sleeper(matches: &ArgMatches) -> Sleeper {
    let ms = matches.value_of("milliseconds")
        .unwrap_or(DEFAULT_POLL_FREQUENCY.to_string())
        .parse::<u64>()
        .unwrap();
    let sleeper = Sleeper::new(ms);
    sleeper
}

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
                .default_value("1000")
                .takes_value(true),
        )
        .get_matches()
}
