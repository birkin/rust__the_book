use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!( "Searching for string, ``{}`` in filename, ``{}``", query, filename );

    let contents = fs::read_to_string( filename )
        .expect( "Problem reading the file." );

    println!( "With text:\n``\n{}``", contents );
}
