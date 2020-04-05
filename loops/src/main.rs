use std::time::Instant;

fn main() {

    // loop {
    //     println!("Hello, world!");
    // }



    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is `{:?}`", result );



    let mut number = 3;
    let start = Instant::now();

    while number != 0 {
        println!("{:?}", number );
        number -= 1;
    }

    println!("LIFTOFF!!");
    println!( "elapsed time, `{:?}`", start.elapsed() );



    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // let start = Instant::now();
    // while index < 5 {
    //     println!("a[index], `{:?}`", a[index]);
    //     index += 1;
    // }
    // let duration = start.elapsed();
    // println!("elapsed time, `{:?}`", duration);

    // let start = Instant::now();
    // for element in a.iter() {
    //     println!("element is, `{:?}`", element);
    // }
    // let duration = start.elapsed();
    // println!("elapsed time, `{:?}`", duration);  // this is six times as fast as the `while` loop



    let start = Instant::now();
    for number in (1..4).rev() {
        println!("{:?}!", number);

    }
    println!("LIFTOFF!!");
    println!( "elapsed time, `{:?}`", start.elapsed() );  // eight times as fast as the first liftoff





}
