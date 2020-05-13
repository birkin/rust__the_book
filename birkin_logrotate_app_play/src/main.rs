#[macro_use]
extern crate log;

extern crate env_logger;

use serde::Deserialize;
// use serde_json::{json, Value};
use serde_json::{Value};

use std::env;
use std::fs;
// use std::thread::sleep;
use std::time::{Duration, Instant};


/*

NEXT:
- next:
    √ call the function that will initiate the loop
    √ document what I think is happening in load_log_paths()
    - have that loop function pass each item to another function that will manage each step of processing.
- flow: for each entry:
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

    /* setup settings */
    let config = Config::new();
    // println!("config, ``{:?}``", config);

    /* setup logging */
    env_logger::init();  // assumes ```export RUST_LOG="info"```; only error! will work if no RUST_LOG-level is set
    debug!( "{}", format!("config, ``{:#?}``", config) );  // debug! needs a string literal  :(

    /* load log-paths json-object */
    let log_paths_obj: std::vec::Vec<serde_json::value::Value> = load_log_paths( &config.logger_json_file_path );
    debug!( "{}", format!("log_paths_obj, ``{:#?}``", log_paths_obj) );

    /* process files */
    process_logs( &log_paths_obj );

    /* output */
    let duration: Duration = start_time.elapsed();
    info!( "{}", format!("elapsed-time, ``{:?}``", duration) );

}


fn process_logs( log_paths_obj: &std::vec::Vec<serde_json::value::Value> ) {
    /*  this will pass the log_paths_obj to a function...
        ...that'll loop through each path and call another function that will handle each step. */
    for item in log_paths_obj {
        // println!("\nitem, ``{:?}``", item);  // yields (EG): item, ``Object({"path": String("/foo/the.log")})``
        // let z: () = item;  // yields: found `&serde_json::value::Value`
        process_path( item );
    }
}


fn process_path( item: &serde_json::value::Value ) {
    println!("\nitem from within process_path(), ``{:?}``", item);  // yields (EG): item, ``Object({"path": String("/foo/the.log")})``

}



fn load_log_paths( logger_json_file_path: &std::string::String ) -> std::vec::Vec<serde_json::value::Value> {

    // --- read file ---
    let jsn: String = fs::read_to_string( &logger_json_file_path ).unwrap_or_else(|error| {
        panic!("Problem reading the json-file -- ``{:?}``", error);
    });
    println!("\njsn, ``{:?}``", jsn);  // yields: jsn, ``"[\n  {\n    \"path\": \"/foo/the.log\"\n  },\n  {\n    \"path\": \"/path/to/logs/addto_refworks_logs/addto_refworks.log\"\n  },\n  {\n    \"path\": \"/path/to/logs/annex_counts_logs/annex_counts.log\"\n  }\n]\n"``
    // let zz: () = jsn;  // yields: found struct `std::string::String`

    // --- turn String into json-object ---
    let paths_obj: Value = serde_json::from_str(&jsn).unwrap_or_else(|error| {
        panic!("Problem converting the json-file to an object -- maybe invalid json? -- ``{:?}``", error);
    });
    println!("\npaths_obj, ``{:?}``", paths_obj); // yields: paths_obj, ``Array([Object({"path": String("/foo/the.log")}), Object({"path": String("/path/to/logs/addto_refworks_logs/addto_refworks.log")}), Object({"path": String("/path/to/logs/annex_counts_logs/annex_counts.log")})])``
    // let zz: () = paths_obj;  // yields: found enum `serde_json::value::Value`

    // --- turns the json-object in to a Vector(reference) ---
    // Question: why wasn't I able to iterate over this?
    let paths_obj_array = paths_obj.as_array().unwrap_or_else(|| {  // as_array() returns Option -- <https://docs.serde.rs/serde_json/value/enum.Value.html#method.as_array>
        panic!("Problem handling paths_obj");
    });
    println!("\npaths_obj_array, ``{:?}``", paths_obj_array);  // yields: paths_obj_array, ``[Object({"path": String("/foo/the.log")}), Object({"path": String("/path/to/logs/addto_refworks_logs/addto_refworks.log")}), Object({"path": String("/path/to/logs/annex_counts_logs/annex_counts.log")})]``
    // let zz: () = paths_obj_array;  // yields found reference `&std::vec::Vec<serde_json::value::Value>`

    // -- turns the Vector-reference into a Vector-Struct
    // Only this allowed me to pass the returned-result to another function: process_logs()
    // Just skimmed a _great_ post that I should re-read to refactor this function: <https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html>
    let real_array = paths_obj_array.to_vec();
    println!("\nreal_array, ``{:?}``", real_array);  // yields: real_array, ``[Object({"path": String("/foo/the.log")}), Object({"path": String("/path/to/logs/addto_refworks_logs/addto_refworks.log")}), Object({"path": String("/path/to/logs/annex_counts_logs/annex_counts.log")})]``
    // let zz: () = real_array;  // yields: found struct `std::vec::Vec<serde_json::value::Value>`

    return real_array;
}



// fn backup_files( log_directory: &serde_json::value::Value ) {

//     // -- get a filepath
//     // let first_filepath: &serde_json::value::Value = &log_directory[0]["path"];
//     let first_filepath = &log_directory[0]["path"];
//     // let zz: () = first_filepath;  // yields: found `&serde_json::value::Value`
//     debug!( "{}", format!("first_filepath, ``{:#?}``", first_filepath) );
//     // assert_eq!( first_filepath, String::from("/path/to/logs/addto_refworks_logs/addto_refworks.log") );

//     // let destination_filepath = first_filepath.to_string() + "_02";
//     // let destination_filepath = get_destination_filepath( first_filepath )
//     // println!("destination_filepath, ``{:?}``", destination_filepath);

//     // fs::copy("foo.txt", "bar.txt")?;

//     sleep(Duration::new(0, 1)); // (seconds, nanoseconds)
//     debug!( "end of backup_files()" )
// }



// fn quit(start_time: Instant) {
//     let duration = start_time.elapsed();
//     println!(" in quit(); duration, `{:?}`", duration);
//     std::process::exit(0);
// }
