#[macro_use]
extern crate log;

extern crate env_logger;
extern crate glob;

use serde::Deserialize;
use serde_json::{Value};

use std::env;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

use glob::glob;  // <https://docs.rs/glob/0.3.0/glob/>

// use serde_json::{json, Value};
// use std::thread::sleep;






/*

NEXT:
- next:
    √ call the function that will initiate the loop
    √ document what I think is happening in load_log_paths()
    √ have that loop function pass each item to another function that will manage each step of processing.
    √ at: determine parent directory -- now that I have path as a String, see if I can get determine-parent-directory()
          ...to work (maybe I'll have to return back a full string instead of a reference)
    -> get sorted list of files from directory
        - i'm getting there; next...
            - convert the path &str into a String
            - append that to a mutable Vector
            - return the Vector of Strings
        - <https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html>
        - <https://stackoverflow.com/questions/26076005/how-can-i-list-files-of-a-directory-in-rust>
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
    /*  Iterates through the log_paths_obj, sending each item to a function...
        ...which will manage the steps of processing the item.
        Called by: main() */
    for item in log_paths_obj {
        // println!("\nitem, ``{:?}``", item);  // yields (EG): item, ``Object({"path": String("/foo/the.log")})``
        // let z: () = item;  // yields: found `&serde_json::value::Value`
        manage_item( item );
    }
}


fn manage_item( item: &serde_json::value::Value ) {
    /*  Manages the steps to process the log entry.
        Steps...
        - check file size and bail if it's not big enough.
        - determine parent-directory from path.
        - read all the files in the directory.
        - in reverse-alphabetical-order, rename 09 through 01.
        - rename the original file.
        - create a new empty file.
        Called by: process_logs() */

    // debug!( "{}", format!("item from within manage_item, ``{:?}``", item) );  // yields (EG): item, ``Object({"path": String("/foo/the.log")})``

    let path_rfrnc = item["path"].as_str().unwrap_or_else( || {panic!("problem reading path from json-obj -- ``{:?}``");} );
    // println!("path_rfrnc, ``{:?}``", path_rfrnc);
    // let zz: () = path_rfrnc;  // yields: found `&str`

    let path: String = path_rfrnc.into();
    // println!("path, ``{:?}``", path);
    // let zz: () = path;  // yields: found struct `std::string::String`

    if check_existence( &path ) == false {
        return;
    }
    // println!("checking I can still reference path, ``{:?}``", path);

    if check_big_enough( &path ) == false {
        return;
    }

    info!( "{}", format!("PROCEEDING to process path, ``{:?}``", path) );

    // -- TODO...
    //    Try something like: let mut parent_path = std::Path;
    //    Then maybe sending the empty parent_path to the prep-function and returning it won't cause lifetime errors.
    //    ...but getting a String works for now; so this try will be a refactor.
    let parent_path = determine_directory( &path );

    let file_list = prep_file_list( parent_path );

}


fn prep_file_list( parent_path: String ) -> Vec<String> {

    // let paths = fs::read_dir( parent_path ).unwrap_or_else( |err| {
    //     panic!("could not read the parent_path; error, ``{}``", err);
    // });
    // println!("paths, ``{:?}``", paths);

    let pattern = format!( "{}/*.log", parent_path );
    debug!( "{}", format!("pattern, ``{:?}``", pattern) );

    let paths = glob( &pattern ).unwrap_or_else( |err| {
        panic!("could not glob the pattern; error, ``{}``", err);
    });
    // let zz: () = paths;  // yields (before unwrap): found enum `std::result::Result<glob::Paths, glob::PatternError>`

    for entry in paths {
        let path = entry.unwrap_or_else( |err| {  // path without unwrap is: enum `std::result::Result<std::path::PathBuf, glob::GlobError>`
            panic!("could not access the path; error, ``{}``", err);
        });
        println!("path.display(), ``{:?}``", path.display());
        // let zz: () = path;  // yields: found struct `std::path::PathBuf`
        // let zz: () = path.display();  // yields: found struct `std::path::Display`

        let path_string = path.to_str().unwrap_or_else( || {
            panic!("could turn the path into a string");
        });
        println!("path_string, ``{:?}``", path_string);
        let zz: () = path_string;  // yields: found `&str`
    }


    let v = vec!["aa".into(), "cc".into()];
    println!("v, ``{:?}``", v);
    v


    // extern crate glob;
    // use self::glob::glob;

    // let files:Vec<Path> = glob("*").collect();
}


fn determine_directory(  path: &str ) -> String {
    let parent = Path::new(path).parent().unwrap_or_else( || {
        panic!("no parent found");
    });
    // let zz: () = parent;  // yields: found `&std::path::Path`
    // debug!( "{}", format!("parent, ``{:?}``", parent) );

    let parent_str = parent.to_str().unwrap_or_else( || {
        panic!("could not get &str from parent-Path");
    });
    // let zz: () = parent_str;  // yields: found `&str`
    // debug!( "{}", format!("parent_str, ``{:?}``", parent_str) );

    let parent_string = parent_str.to_string();
    // let zz: () = parent_string;  // yields: found struct `std::string::String`  👍
    debug!( "{}", format!("parent_string, ``{:?}``", parent_string) );

    parent_string
}


// fn determine_directory(  path: &str ) -> &std::path::Path {
//     let parent = Path::new(path).parent().unwrap_or_else(|| {
//         panic!("no parent found");
//     });
//     // let zz: () = parent;  // yields: found `&std::path::Path`
//     debug!( "{}", format!("parent, ``{:?}``", parent) );

//     parent
// }


fn check_big_enough( path: &str ) -> bool {
    /*  Checks that file is big enough.
        Called by manage_item().
        TODO: check against config setting */

    const THRESHOLD: u64 = 1000;
    let mut result = false;

    let metadata = fs::metadata(path);
    // println!("metadata, ``{:?}``", metadata);

    match metadata {
        Ok(metadata) => {
            let file_size: u64 = metadata.len() / 1000;
            debug!( "{}", format!("file_size in Kb, ``{}``", file_size) );
            // let zz: () = file_size;  // yields: found `u64`
            if file_size > THRESHOLD {
                debug!( "file_size big enough to process" );
                result = true;
            } else {
                debug!( "file_size not big enough to process" );
            }
        },
        Err(err) => {
            error!( "{}", format!("could not get metadata for path, ``{}``; error, ``{}``", path, err) );
        }
    };

    return result;
}


fn check_existence( path: &str ) -> bool {
    /*  Checks that file exists.
        Called by manage_item() */
    if Path::new(path).exists() == false {
        error!( "{}", format!("path, ``{}`` does not exist", path) );
        false
    } else {
        debug!( "{}", format!("path, ``{}`` exists", path) );
        true
    }
}


fn load_log_paths( logger_json_file_path: &std::string::String ) -> std::vec::Vec<serde_json::value::Value> {
    /*  Loads json list of paths into an iterable json-object.
        Called by: main()  */

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
