/*
    Usage:
    - To find the word `to` in the file poem.txt,
      `$ cargo run to ./poem.txt`
    - Search results will be case-sensitive unless
      an envar CASE_INSENSITIVE exists and is set to `true`.
 */

use std::env;
use std::process;

use ch13d_minigrep_better::Config;


fn main() {
    // let args: Vec<String> = env::args().collect();

    // let config = Config::new( &args ).unwrap_or_else( |err| {
    //     eprintln!("Problem parsing arguments: ``{:?}``", err);
    //     process::exit( 1 );
    // });

    let config = Config::new( env::args() ).unwrap_or_else( |err| {
        eprintln!("Problem parsing arguments: ``{:?}``", err);
        process::exit( 1 );
    });

    println!( "Searching for string, ``{}`` in filename, ``{}``", config.query, config.filename );

    if let Err(e) = ch13d_minigrep_better::run(config) {
        eprintln!( "Application error, ``{:?}``", e );

        process::exit(1);
    }
}
