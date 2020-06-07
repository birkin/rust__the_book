use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];
    // let zz = &args[99];  // panics at runtime

    println!( "Searching for string, ``{}`` in filename, ``{}``", query, filename );

}
