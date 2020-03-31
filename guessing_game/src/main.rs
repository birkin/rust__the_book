use std::io;
use rand::Rng;
// use log::{info, trace, warn};


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range( 1, 101 );
    println!("secret_number, ```{:?}```", secret_number );

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");

    println!("You guessed: {}", guess);

    // let x = io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to readline");
    // println!("x, ```{}```", x );

}
