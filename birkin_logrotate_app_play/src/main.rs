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
- read json file. HERE
    - I'm reading the json file, next... review the other may-3 evening emails I sent myself showing json-load options.
- for each entry:
    - make a reverse-time-sorted list of the existing log-files
    - delete the oldest if there are more than MAX
    - bump the name/number up by 1 (and re-save?)
    - for the last in the list, save an empty file.
*/


#[derive(Deserialize, Debug)]
struct Config {
    log_level: String,
    logger_json_file_path: String
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
                Config { log_level, logger_json_file_path }
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
    let s: String = fs::read_to_string( &config.logger_json_file_path ).unwrap();
    let directory: Value = serde_json::from_str(&s).unwrap();  // serde_json::value::Value -- Array([Object({"path": String("/foo/bar.log")}),...
    println!("directory, ``{:?}``", directory);
    println!("first-path, ``{:?}``", directory[0]["path"]);

    // load_directory();


    expensive_function();

    /* output */
    let duration: Duration = start_time.elapsed();
    info!( "{}", format!("elapsed-time, ``{:?}``", duration) );

}


// fn load_directory( &config ) {
//     let s = fs::read_to_string( &config.logger_json_file_path ).unwrap();
//     let cfg: Value = serde_json::from_str(&s).unwrap();
//     println!("cfg, ``{:?}``", cfg);
//     println!("first-path, ``{:?}``", cfg[0]["path"]);
// }


fn expensive_function() {
    sleep(Duration::new(0, 1)); // (seconds, nanoseconds)
}


// fn quit(start_time: Instant) {
//     let duration = start_time.elapsed();
//     println!(" in quit(); duration, `{:?}`", duration);
//     std::process::exit(0);
// }
