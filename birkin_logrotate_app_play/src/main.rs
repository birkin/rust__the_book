#[macro_use]
extern crate log;

extern crate env_logger;

use serde::Deserialize;
// use serde_json::{json, Value};
use serde_json::{Value};

use std::env;
use std::fs;
use std::thread::sleep;
use std::time::{Duration, Instant};


/*

NEXT:
- next: build a destination filepath where foo.log->foo_01.log, foo_01.log->foo_02.log, etc
- for each entry:
    - make a reverse-time-sorted list of the existing log-files
    - delete the oldest if there are more than MAX
    - bump the name/number up by 1 (and re-save?)
    - for the last in the list, save an empty file.
*/


#[derive(Deserialize, Debug)]
struct Config {
    log_level: String,
    logger_json_file_path: String,
    max_entries: i8  // this could be added to the json-file instead
}

impl Config {
    /*  forgive the "RUST_LOG" hack; i really wanted to use the envar project-prefix to set the log-level,
        ...and couldn't figure out how to specify an alternative prefix for env_logger::init() */
    fn new() -> Config {
        match envy::prefixed("LOG_ROTATOR__").from_env::<Config>() {  // https://github.com/softprops/envy
            Ok(config) => {
                env::set_var( "RUST_LOG", &config.log_level);
                let log_level = config.log_level;  // not used, but still useful to set, for panic-message if it's missing
                let logger_json_file_path = config.logger_json_file_path;
                let max_entries = config.max_entries;
                Config { log_level, logger_json_file_path, max_entries }
            },
            Err(error) => panic!("{:#?}", error) // this shows the missing envar
        }
    }
}


fn main() {

    let start_time = Instant::now();
    // println!("start_time, `{:?}`", start_time);

    /* settings */
    let config = Config::new();
    // println!("config, ``{:?}``", config);

    /* logging */
    env_logger::init();  // assumes ```export RUST_LOG="info"```; only error! will work if no RUST_LOG-level is set
    debug!( "{}", format!("config, ``{:#?}``", config) );  // debug! needs a string literal

    /* work */

    let log_directory: Value = load_log_directory( &config );
    // debug!( "{}", format!("config access check, ``{:#?}``", config) );  // just to make sure I still can access config

    backup_files( &log_directory );

    /* output */
    let duration: Duration = start_time.elapsed();
    info!( "{}", format!("elapsed-time, ``{:?}``", duration) );

}


fn load_log_directory( config: &Config ) -> Value {
    let s: String = fs::read_to_string( &config.logger_json_file_path ).unwrap();
    let log_directory: Value = serde_json::from_str(&s).unwrap();  // serde_json::value::Value -- Array([Object({"path": String("/foo/bar.log")}),...
    debug!( "{}", format!("log_directory, ``{:#?}``", log_directory) );  // println!("first-path, ``{:?}``", directory[0]["path"]);
    return log_directory
}


fn backup_files( log_directory: &serde_json::value::Value ) {
    // let zz: () = log_directory;
    let first_filepath = &log_directory[0]["path"];
    debug!( "{}", format!("first_filepath, ``{:#?}``", first_filepath) );

    let destination_filepath = first_filepath.to_string() + "_02";
    println!("destination_filepath, ``{:?}``", destination_filepath);

    // fs::copy("foo.txt", "bar.txt")?;

    sleep(Duration::new(0, 1)); // (seconds, nanoseconds)
    debug!( "end of backup_files()" )
}


// fn quit(start_time: Instant) {
//     let duration = start_time.elapsed();
//     println!(" in quit(); duration, `{:?}`", duration);
//     std::process::exit(0);
// }
