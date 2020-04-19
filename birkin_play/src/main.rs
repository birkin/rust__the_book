#[macro_use]
extern crate log;
extern crate env_logger;

use std::thread::sleep;
use std::time::{Duration, Instant};

use std::env;
use std::env::VarError;

// use std::ffi;  // `Foreign Function Interface`
// use std::option;


/*

NEXT:
- Continue to work on loading-settings. Can now detect an envar-load-failure.
    - next, run through a list of envar-settings and fail with nice message if any don't load.
    - ideally, try all settings, and, if there are any failures, fail showing the list of settings that couldn't be loaded.
- Resources...
    - <https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html>
    - Result: <https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html>
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



fn main() {

    let start_time = Instant::now();
    println!("start_time, `{:?}`", start_time);

    env_logger::init();  // assumes ```export RUST_LOG="info"```
    debug!("logger debug test");
    info!("logger info test");
    error!("logger error test");  // only this will print if RUST_LOG is not set


    /* settings */

    // get envar
    let some_var = load_setting( start_time );
    println!("some_var, `{:?}`", some_var);
    if some_var == None {
        quit( start_time );
    }


    // /* settings -- works
    //     ...but env::var_os() yields an OsString type that I'm having trouble turning into a String for the setting
    // */

    // // get envar
    // let some_var: option::Option<ffi::OsString> = env::var_os("RUST_PLAY__SOME_VAR");  // see <https://doc.rust-lang.org/std/ffi/index.html> -- I should handle Result( value, error) here.

    // println!("some_var, `{:?}`", some_var);

    // if some_var == None {
    //     println!("some_var not initialized; quitting");
    //     quit( start );
    // } else {
    //     println!("Something found");
    // }


    // do work
    expensive_function();

    // print duration
    let duration = start_time.elapsed();
    println!("Time elapsed in expensive_function() is, `{:?}`", duration);
}

fn load_setting( start_time: Instant ) -> Option<String> {

    // println!("start_time, `{:?}`", start_time);

    let some_var_try: Result<String, VarError> = env::var("RUST_PLAY__SOME_VAR");

    println!("some_var_try initially, `{:?}`", some_var_try);
    // let zz: () = some_var_try;  // will not compile; shows type of some_var_try. Ok, "found enum `std::result::Result`"

    match some_var_try {
        Ok(the_string) => the_string,
        // Err(the_error) => {
        //     panic!("Problem accessing the envar: {:?}", the_error)
        // },  // works, but I want to handle this
        // Err(the_error) => "problem".to_string(),  // works, but why can't I can't return None?
        Err(the_error) => {
            let message = "error, ```".to_string() + &the_error.to_string() + "```";  // hmm... figured out string-substitution, like in println()
            println!("message, {:?}", message);
            quit( start_time );
            message

        },
    };




    // if some_var_try == None {
    //     println!("some_var not initialized; quitting");
    //     quit( start_time );
    // } else {
    //     println!("Something found");
    // }

    // return Some( "foo".to_string() )
    return None
}

fn quit(start_time: Instant) {
    let duration = start_time.elapsed();
    println!(" in quit(); duration, `{:?}`", duration);
    std::process::exit(0);
}

fn expensive_function() {
    sleep(Duration::new(0, 1)); // (seconds, nanoseconds)
}
