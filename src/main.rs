#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgMatches};
use std::error::Error;

use crate::aggregator::WaiterCollection;
use crate::file_waiter::FileWaiter;
use crate::process_waiter::ProcessWaiter;
use crate::sleeper::Sleeper;
use crate::steam_waiter::SteamWaiter;
use crate::waiter::*;

mod aggregator;
mod file_waiter;
mod process_waiter;
mod sleeper;
mod steam_waiter;
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
    let mut waiters = WaiterCollection::new(sleeper, &matches);

    waiters.add("pid", ProcessWaiter::start);
    waiters.add("file", FileWaiter::start);
    waiters.add("app_id", SteamWaiter::start);

    waiters.wait_for_all()
}

fn get_matches() -> ArgMatches<'static> {
    App::new("wait4")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Wait for events before terminating the program.")
        .arg(
            Arg::with_name("file")
                .long("file")
                .short("f")
                .help("Wait till the given file exists.")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("pid")
                .long("pid")
                .short("p")
                .help("Wait till process with the given pid terminates.")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("app_id")
                .long("steam")
                .short("s")
                .help("Wait until the steam download for the game with the given app id finishes.")
                .long_help(&format!("Wait until the steam download for the game with the given app id finishes. \
                                    The following tool helps with finding the right id: https://steamdb.info/apps/. \
                                    This option monitors the steam log file. Its filepath is read from the environment \
                                    variable {} (default {}).", SteamWaiter::env_var_log_file(), SteamWaiter::default_log_file_path()))
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
