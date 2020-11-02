fn main() {

    // -- conditional `if let` expressions
    main01();
}


// -- main01() -- conditional `if let` expressions

use std::num::ParseIntError;

fn main01() {
    let favorite_color: Option< &str > = None;
    let is_tuesday = false;
    // let age: Result< u8, _ > = "34".parse();  // as written
    let age: Result< u8, ParseIntError > = "34".parse();  // reminding myself of Result output

    if let Some( color ) = favorite_color {
        println!( "Using your favorite color, ``{:?}``, as the bacground", color );
    } else if is_tuesday {
        println!( "Tuesday is green day!" );
    } else if let Ok( age ) = age {
        if age > 30 {
            println!( "Using purple as the background color" );
        } else {
            println!( "Using orange as the background color" );
        }
    } else {
        println!( "Using blue as the background color" );
    }

}
