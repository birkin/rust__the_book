#[macro_use]
extern crate log;
extern crate env_logger;

use std::thread::sleep;
use std::time::{Duration, Instant};

use std::env;
use std::ffi;  // `Foreign Function Interface`
use std::option;


/*

NEXT:
- load settings

*/


struct Settings {
    log_level: String,
    initial_data: String,
    something_fixed: String,
}

impl Settings {
    fn load_fixed(&self) {
        something_fixed = "foo";
    }

    fn load_from_envars(&self) {
        self.width * self.height
    }
}


fn main() {

    let start = Instant::now();

    env_logger::init();  // assumes ```export RUST_LOG="info"```
    debug!("logger debug test");
    info!("logger info test");
    error!("logger error test");

    // get envar
    let some_var: option::Option<ffi::OsString> = env::var_os("RUST_PLAY__SOME_VAR");  // see <https://doc.rust-lang.org/std/ffi/index.html> -- I should handle Result( value, error) here.
    println!("some_var, `{:?}`", some_var);
    if some_var == None {
        println!("some_var not initialized; quitting");
        quit( start );
    } else {
        println!("Something found");
    }

    // do work
    expensive_function();

    // print duration
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is, `{:?}`", duration);
}

fn quit(start: Instant) {
    let duration = start.elapsed();
    println!("Time elapsed, `{:?}`", duration);
    std::process::exit(0);
}

fn expensive_function() {
    sleep(Duration::new(0, 1)); // (seconds, nanoseconds)
}
