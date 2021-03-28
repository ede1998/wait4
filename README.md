# wait4

A simple program that allows waiting for specific events.
At the moment, 3 types of events are implemented:
- process finishes
- file is created
- steam finishes a game download

In the future, this could be expanded.

The intended usage is the following:
`wait4 -p 1337 && echo 'Process finshed'`

For ease of use there are the following helper scripts. Under the hood, they use python and fzf.
- `wait4pid`: Allows selection of process from a list of all running processes.
- `wait4steam`: Allows selection of a steam game from a list of all steam games.
The scripts then call `wait4` with the selected id.

## Help
```
wait4 1.0.0
ede1998 <online@erik-hennig.me>
Wait for events before terminating the program.

USAGE:
    wait4 [OPTIONS]

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


OPTIONS:
    -s, --steam <app_id>...           
            Wait until the steam download for the game with the given app id finishes. The following tool helps with
            finding the right id: https://steamdb.info/apps/. This option monitors the steam log file. Its filepath is
            read from the environment variable STEAM_LOG_FILE (default ~/.local/share/Steam/logs/content_log.txt).
    -f, --file <file>...              
            Wait till the given file exists.

    -F, --frequency <milliseconds>    
            Configures the poll frequency in milliseconds. [default: 1000]

    -p, --pid <pid>...                
            Wait till process with the given pid terminates.
```

## Architecture
The program checks preconditions for each given parameter. For instance, a given process id must exist.
If any precondition check fails, the program immediately terminates with status 1 and prints to stderr all precondition checks that failed.

If all precondition checks succeed, a new thread is launched for each parameter. This thread polls every n seconds whether the event it watches has happened.
If it did, the thread terminates. If all threads terminated, the program terminates with status 0.

For logging, the programm uses [env_logger](https://crates.io/crates/env_logger). Therefore logging can be enabled by setting the environment variable to the desired log level:
`RUST_LOG=DEBUG`
