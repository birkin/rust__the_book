#[macro_use]
extern crate log;

extern crate env_logger;

// use serde::Deserialize;

use std::env;
use std::env::VarError;
use std::thread::sleep;
use std::time::{Duration, Instant};

// use std::option::Option;

// use std::ffi;  // `Foreign Function Interface`
// use std::option;


/*

NEXT:
- Continue to work on loading-settings.
    - i have a populated config-instance in main; now populate those values from envars
        - hmm... <https://github.com/softprops/envy> -- i abandoned this, can't remember why -- maybe look at it enough...
            ...to jot down why i abandoned it.
        - Q: is there a _simple_ way to pass to init the config log-level setting I've created?
            - madness? but possible -- when the log-level setting is loaded in the config-code...
                ...SET a "RUST_LOG" envar to that level! -- so that when the log is initialized, it "just reads it".
- Resources...
    - good config info: <https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html?highlight=constructor#the-trade-offs-of-using-clone>
    - <https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html>
    - Result: <https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html>
    - rust settings approaches: <https://users.rust-lang.org/t/how-do-rustaceans-handle-configuration-values/14003/4>
        - <https://crates.io/crates/dotenv>
*/


#[derive(Debug)]
struct Config {
    log_level: String,
    // logger_json_file_path: String
}

impl Config {
    fn new( start_time: Instant ) -> Config {
        /* Returns config-instance with attributes populated from envars.
           Quits on error loading envar.
           TODO: simplify, and figure out way to iterate over the attributes.  */
        let log_level_try: Result<String, VarError> = env::var("LOG_ROTATOR__LOG_LEVEL");
        match log_level_try {
            Ok(_) => {},
            Err(_err) => {
                println!("log_level setting not found; quitting");
                quit( start_time );
                std::process::exit(-1);  // should never get here, but need for compiler.
            }
        };
        let log_level = log_level_try.unwrap();
        Config { log_level }
    }
}


fn main() {

    let start_time = Instant::now();
    println!("start_time, `{:?}`", start_time);

    /* settings */
    let config = Config::new( start_time );
    println!("config, ``{:?}``", config);

    /* logging */
    env_logger::init();  // assumes ```export RUST_LOG="info"```
    debug!("logger debug test");
    info!("logger info test");
    error!("logger error test");  // only this will print if RUST_LOG is not set

    /* work */
    // do work
    expensive_function();


    // print duration
    let duration = start_time.elapsed();
    println!("Time elapsed in expensive_function() is, `{:?}`", duration);

}


// fn load_settings( start_time: Instant ) -> String  {
//     dotenv().ok();
//     match envy::prefixed("LOG_ROTATOR__").from_env::<Config>() {
//         Ok(config) => {
//             println!("config.log_level, ``{:?}``", config.log_level);
//             println!("{:#?}", config);
//             // config
//             "foo".to_string()
//         },
//         Err(the_error) => {
//             // println!("error parsing config from env: {}", err);
//             // err.to_string()
//             // let zz: () = err;
//             let message = "error, ```".to_string() + &the_error.to_string() + "```; quitting";
//             // println!( message );
//             println!("{:?}", message);
//             // "bar".to_string()
//             quit( start_time );
//             message
//         },
//     }
// }

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



fn quit(start_time: Instant) {
    let duration = start_time.elapsed();
    println!(" in quit(); duration, `{:?}`", duration);
    std::process::exit(0);
}



fn expensive_function() {
    sleep(Duration::new(0, 1)); // (seconds, nanoseconds)
}
