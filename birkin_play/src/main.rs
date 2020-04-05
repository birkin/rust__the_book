#[macro_use]
extern crate simple_error;

use std::thread::sleep;
use std::time::{Duration, Instant};
use std::error::Error;

use std::env;
use std::ffi::OsString;
use std::option::Option;

fn main() {
    let start = Instant::now();

    // let some_var = env::var("SOME_ENVAR").is_err();
    // let some_var: std::option::Option<std::ffi::OsString> = env::var_os("SOME_ENVAR");
    // let some_var: option::Option<std::ffi::OsString> = env::var_os("SOME_ENVAR");
    let some_var: Option<OsString> = env::var_os("SOME_ENVAR");
    println!("some_var, `{:?}`", some_var);

    if some_var == None {
        println!("`None` found");
        bail!("envars not set");
    } else {
        println!("Something found");
    }

    expensive_function();

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is, `{:?}`", duration);
}

fn expensive_function() {
    sleep(Duration::new(0, 1)); // (seconds, nanoseconds)
}
