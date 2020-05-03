use std::time::{Duration, Instant};

fn main() {

    misc01();

}


fn misc01() {
    let start_time = Instant::now();

    let duration: Duration = start_time.elapsed();
    println!("duration, {:?}", duration); // works: duration, 21ns

    let message = format!( "duration, {:?}", duration );  // works for "{:?}", not for "{}"
    println!("message, {:?}", message);

    // let test_message = format!( "hello, {}", "world" );
    // println!("test_message, ``{:?}``", test_message);  // works: test_message, ``"hello, world"``

}
