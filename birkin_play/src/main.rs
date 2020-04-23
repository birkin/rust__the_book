extern crate dotenv;

#[macro_use]
extern crate log;

extern crate env_logger;


use serde::Deserialize;

// use std::env;
// use std::env::VarError;
use std::thread::sleep;
use std::time::{Duration, Instant};

use dotenv::dotenv;

// use std::ffi;  // `Foreign Function Interface`
// use std::option;


/*

NEXT:
- Continue to work on loading-settings.
    - next, envy seems to be loading the envars, but how to access a Config property now? Existing attempts aren't working.
        - thought: build into load settings the return of a config-instance
        - hmm... <https://github.com/softprops/envy>
    - (again, ideal: the error message would show all that are not set)
- Resources...
    - <https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html>
    - Result: <https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html>
    - rust settings approaches: <https://users.rust-lang.org/t/how-do-rustaceans-handle-configuration-values/14003/4>
        - <https://crates.io/crates/dotenv>
*/


// struct Settings {
//     // log_level: String,
//     something_from_envvar: String,
//     // something_fixed: String,
// }

// impl Settings {
//     pub fn new() -> Settings {
//         let env_log_level = env::var_os("RUST_PLAY__SOME_VAR");
//         Settings {
//             log_level: env_log_level.into_string(),
//             initial_data: "coming",
//             something_fixed: "foo",
//         }
//     }
// }


#[derive(Deserialize)]
#[derive(Debug)]
struct Config {
    log_level: String,
    logger_json_file_path: String
}

// impl Config {
//     fn get_log_level() -> String {
//         log_level
//     }
// }

fn main() {

    let start_time = Instant::now();
    println!("start_time, `{:?}`", start_time);

    env_logger::init();  // assumes ```export RUST_LOG="info"```
    debug!("logger debug test");
    info!("logger info test");
    error!("logger error test");  // only this will print if RUST_LOG is not set


    // settings
    load_settings();
    // println!("Config::log_level, ``{:?}``", Config::log_level);  // doesn't work
    // println!("Config.log_level, ``{:?}``", Config.log_level);  // doesn't work

    // let cnfg = Config {};  // doesn't work
    // println!("cnfg, ``{:?}``", cnfg);

    // println!("log_level, {:?}", Config::get_log_level() );



    // do work
    expensive_function();


    // print duration
    let duration = start_time.elapsed();
    println!("Time elapsed in expensive_function() is, `{:?}`", duration);

}

fn load_settings() {
    dotenv().ok();
    match envy::prefixed("LOG_ROTATOR__").from_env::<Config>() {
        // Ok(config) => println!("provided config.bar {:?}", config.bar),
        Ok(config) => println!("updated config"),
        Err(err) => println!("error parsing config from env: {}", err),
    }
}

// fn load_settings() {

//     dotenv().ok();
//     for (key, value) in env::vars() {
//         println!("``{}``: ``{}``", key, value);
//     }


//     // // get envar
//     // let some_var = load_setting( start_time );
//     // println!("some_var, `{:?}`", some_var);
//     // if some_var == None {
//     //     quit( start_time );



//     // /* settings -- works
//     //     ...but env::var_os() yields an OsString type that I'm having trouble turning into a String for the setting
//     // */

//     // // get envar
//     // let some_var: option::Option<ffi::OsString> = env::var_os("RUST_PLAY__SOME_VAR");  // see <https://doc.rust-lang.org/std/ffi/index.html> -- I should handle Result( value, error) here.

//     // println!("some_var, `{:?}`", some_var);

//     // if some_var == None {
//     //     println!("some_var not initialized; quitting");
//     //     quit( start );
//     // } else {
//     //     println!("Something found");
//     // }

// }




// fn load_setting( start_time: Instant ) -> Option<String> {

//     // println!("start_time, `{:?}`", start_time);

//     let some_var_try: Result<String, VarError> = env::var("RUST_PLAY__SOME_VAR");

//     println!("some_var_try initially, `{:?}`", some_var_try);
//     // let zz: () = some_var_try;  // will not compile; shows type of some_var_try. Ok, "found enum `std::result::Result`"

//     match some_var_try {
//         Ok(the_string) => the_string,
//         // Err(the_error) => {
//         //     panic!("Problem accessing the envar: {:?}", the_error)
//         // },  // works, but I want to handle this
//         // Err(the_error) => "problem".to_string(),  // works, but why can't I can't return None?
//         Err(the_error) => {
//             let message = "error, ```".to_string() + &the_error.to_string() + "```";  // hmm... figured out string-substitution, like in println()
//             println!("message, {:?}", message);
//             quit( start_time );
//             message

//         },
//     };

//     // if some_var_try == None {
//     //     println!("some_var not initialized; quitting");
//     //     quit( start_time );
//     // } else {
//     //     println!("Something found");
//     // }

//     // return Some( "foo".to_string() )
//     return None
// }



// fn quit(start_time: Instant) {
//     let duration = start_time.elapsed();
//     println!(" in quit(); duration, `{:?}`", duration);
//     std::process::exit(0);
// }



fn expensive_function() {
    sleep(Duration::new(0, 1)); // (seconds, nanoseconds)
}
