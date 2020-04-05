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



    // let mut number = 3;

    // while number != 0 {
    //     println!("{:?}", number );
    //     number -= 1;
    // }

    // println!("LIFTOFF!!");



    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    use std::time::Instant;

    let start = Instant::now();
    while index < 5 {
        println!("a[index], `{:?}`", a[index]);
        index += 1;
    }
    let duration = start.elapsed();
    println!("elapsed time, `{:?}`", duration);

    // this is six times as fast as the `while` loop
    let start = Instant::now();
    for element in a.iter() {
        println!("element is, `{:?}`", element);
    }
    let duration = start.elapsed();
    println!("elapsed time, `{:?}`", duration);
}
