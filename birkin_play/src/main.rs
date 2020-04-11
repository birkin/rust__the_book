#[macro_use]
extern crate log;
extern crate env_logger;

use std::thread::sleep;
use std::time::{Duration, Instant};

use std::env;
use std::ffi;  // `Foreign Function Interface`
use std::option;


//

/*

NEXT: logging!

    if RUST_LOG is not set, output is:
    [2020-04-11T08:56:48Z ERROR birkin_play] logger error test

    for ```export RUST_LOG="info"```, output is:
    [2020-04-11T08:52:27Z INFO  birkin_play] logger info test
    [2020-04-11T08:52:27Z ERROR birkin_play] logger error test

    for ```export RUST_LOG="debug"```, output is:
    [2020-04-11T08:54:28Z DEBUG birkin_play] logger debug test
    [2020-04-11T08:54:28Z INFO  birkin_play] logger info test
    [2020-04-11T08:54:28Z ERROR birkin_play] logger error test

    ok, I'm setting RUST_LOG because of the instructions at <https://docs.rs/env_logger/0.7.1/env_logger/>...
    ...but why does the current github example at <https://github.com/sebasmagri/env_logger/blob/master/examples/default.rs>...
    ...show the env-var 'MY_LOG_LEVEL'?

    interesting -- the above date-stamps are UTC. The info at:
    - <https://docs.rs/env_logger/0.7.1/env_logger/#tweaking-the-default-format> and...
    - <https://docs.rs/env_logger/0.7.1/env_logger/fmt/index.html> are worth investigating.

    Ok, also see:
    - <https://github.com/rust-lang/log/pull/48>
    - but really, specifically: <https://github.com/rust-lang/log/pull/48#issuecomment-152821764>

*/

fn main() {

    let start = Instant::now();

    env_logger::init();  // assume ```export RUST_LOG="info"```
    debug!("logger debug test");
    info!("logger info test");
    error!("logger error test");

    // get envar
    let some_var: option::Option<ffi::OsString> = env::var_os("SOME_ENVAR");  // see <https://doc.rust-lang.org/std/ffi/index.html> -- I should handle Result( value, error) here.
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
