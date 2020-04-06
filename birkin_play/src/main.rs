use std::thread::sleep;
use std::time::{Duration, Instant};

use std::env;
use std::ffi;  // `Foreign Function Interface`
use std::option;


fn main() {

    let start = Instant::now();

    // get envar
    let some_var: option::Option<ffi::OsString> = env::var_os("SOME_ENVAR");
    println!("some_var, `{:?}`", some_var);
    if some_var == None {
        println!("some_var not found; quitting");
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
