

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new( value: i32 ) -> Guess {
        println!("\HERE-A");
        if value < 1 || value > 100 {
            panic!( "The secret number must be between 1 and 100; got ``{}``.", value );
        }

        println!("\HERE-B");
        Guess { value }
    }

    pub fn value( &self ) -> i32 {
        println!("\HERE-C");
        self.value
    }
}


fn main() {

    // -- handles problem 'manually'
    // let num = 101;
    // misc01( num );

    // -- builds check into the type
    let g = Guess::new( 50 );
    misc02( g.value() );  // odd -- I can also call g.value here and get the struct property directly, even though the book says...
                          // ..."This public method is necessary because the value field of the Guess struct is private."
}


fn misc02( guess: i32 ) {
    println!( "We know the number ``{}`` is good.", guess );
}



// fn misc01( guess: i32 ) {
//     /* Input must be between 1 and 100 */

//     if guess < 1 || guess > 100 {
//         panic!("The secret number must be between 1 and 100; got ``{}``.", guess);
//     }

//     println!("\nall ok; continuing");
// }
