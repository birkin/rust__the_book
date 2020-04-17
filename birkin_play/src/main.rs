#[macro_use]
extern crate log;
extern crate env_logger;

use std::thread::sleep;
use std::time::{Duration, Instant};

use std::env;
// use std::ffi;  // `Foreign Function Interface`
// use std::option;


/*

NEXT:
- Continue to work on loading-settings. At point where I've gotten the Result back -- now handle it.
    - most simply: fail with a helpful message if an env-var setting can't be loaded.
    - ideally, try all settings, and, if there are any failures, fail showing the list of settings that couldn't be loaded.
- Resources...
    - <https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html>
    - <https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html?highlight=result#recoverable-errors-with-result>
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

    env_logger::init();  // assumes ```export RUST_LOG="info"```
    debug!("logger debug test");
    info!("logger info test");
    error!("logger error test");


    /* settings */

    // get envar
    let some_var = load_setting( start_time );

    println!("some_var, `{:?}`", some_var);



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

fn load_setting( start_time: Instant ) -> String {

    let some_var_try: std::result::Result<std::string::String, std::env::VarError> = env::var("RUST_PLAY__SOME_VAR");
    // let some_var_try = env::var( "RUST_PLAY__SOME_VAR" ).is_err();



    println!("some_var_try, `{:?}`", some_var_try);

    // if some_var_try == None {
    //     println!("some_var not initialized; quitting");
    //     quit( start_time );
    // } else {
    //     println!("Something found");
    // }
    return "foo".to_string()
}

fn quit(start_time: Instant) {
    let duration = start_time.elapsed();
    println!("Time elapsed, `{:?}`", duration);
    std::process::exit(0);
}

fn expensive_function() {
    sleep(Duration::new(0, 1)); // (seconds, nanoseconds)
}
