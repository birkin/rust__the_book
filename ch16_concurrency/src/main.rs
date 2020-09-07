fn main() {

    // -- creating a thread with spawn
    // misc01();

    // -- allowing spawned thread to complete
    // misc02();

    // -- using data from the outer environment in a thread
    misc03();
}



// -- misc03()

use std::thread;

fn misc03() {

    let v = vec![1, 2, 3];

    let handle = thread::spawn( move || {
        println!( "Here's a vector: ``{:?}``", v );
    } );

    handle.join().unwrap();

}



// -- misc02()

// use std::thread;
// use std::time::Duration;

// fn misc02() {
//     let handle = thread::spawn( || {
//         for i in 1..10 {
//             println!( "hi number ``{:?}`` from the spawned thread!", i );
//             thread::sleep( Duration::from_millis(1) );
//         }
//     } );

//     // -- putting ``handle.join().unwrap();`` here would prevent interleaving the spawned threads with the main threads.

//     for i in 1..5 {
//         println!( "hi number ``{:?}`` from the main thread!", i );
//         thread::sleep( Duration::from_millis(1) );
//     }

//     handle.join().unwrap();
// }



// -- misc01()

// use std::thread;
// use std::time::Duration;

// fn misc01() {
//     thread::spawn( || {
//         for i in 1..10 {
//             println!( "hi number ``{:?}`` from the spawned thread!", i );
//             thread::sleep( Duration::from_millis(1) );
//         }
//     } );

//     for i in 1..5 {
//         println!( "hi number ``{:?}`` from the main thread!", i );
//         thread::sleep( Duration::from_millis(1) );
//     }
//     println!( "main thread ending" );
// }



