fn main() {

    // -- creating a thread with spawn
    // misc01();

    // -- allowing spawned thread to complete
    // misc02();

    // -- using data from the outer environment in a thread
    // misc03();

    // -- simple multiple-producer/single-consumer example
    // misc04();

    // -- getting the value from the receiving end of the channel
    // misc05();

    // -- sending multple messages through a channel
    // misc06();

    // -- spawning multiple messages from multiple threads (each with own sender channel) to a single receiver
    // misc07();

    // -- mutex in a single-threaded context
    // misc08();

    // -- mutex in a multi-threaded context
    misc09();
}



// -- misc09()

use std::sync::{Arc, Mutex};
use std::thread;

fn misc09() {
    let counter = Arc::new( Mutex::new(0) );
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone( &counter );
        let handle = thread::spawn( move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        } );
        handles.push( handle );
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!( "Result, ``{:?}``", *counter.lock().unwrap() );
}


// -- misc08()

// use std::sync::Mutex;

// fn misc08() {
//     let m = Mutex::new( 5 );

//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }

//     println!( "m, ``{:?}``", m );
// }



// -- misc07()

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn misc07() {
//     let (tx, rx) = mpsc::channel();

//     let tx1 = mpsc::Sender::clone( &tx );
//     thread::spawn( move || {
//         let vals = vec![
//             String::from( "1a-hi" ),
//             String::from( "1b-from" ),
//             String::from( "1c-the" ),
//             String::from( "1d-thread" ),
//         ];

//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep( Duration::from_secs(1) );
//         }
//     } );

//     thread::spawn( move || {
//         let vals = vec![
//             String::from( "2a-more" ),
//             String::from( "2b-messages" ),
//             String::from( "2c-for" ),
//             String::from( "2d-you" ),
//         ];

//         for val in vals {
//             tx.send( val ).unwrap();
//             thread::sleep( Duration::from_secs(1) );
//         }
//     } );

//     for received in rx {
//         println!( "Got: ``{:?}``", received );
//     }

// }



// -- misc06()

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn misc06() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn( move || {
//         let vals = vec![
//             String::from( "hi" ),
//             String::from( "from" ),
//             String::from( "the" ),
//             String::from( "thread" ),
//         ];

//         for val in vals {
//             tx.send( val ).unwrap();
//             thread::sleep( Duration::from_secs(1) );
//         }
//     } );

//     for received in rx {
//         println!( "Got ``{:?}``", received );
//     }
// }



// -- misc05()

/*
 * Listing 16-8
 */

// use std::sync::mpsc;
// use std::thread;

// fn misc05() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn( move || {
//         let val = String::from( "hi" );
//         tx.send( val ).unwrap();
//     } );

//     let received = rx.recv().unwrap();
//     println!( " Got: ``{:?}``", received );
// }



// -- misc04()
/*
 * Listing 16-7
 * doesn't print anything, likely because the main function ends before the spawned thread has time to do anything.
 */

// use std::sync::mpsc;
// use std::thread;

// fn misc04() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn( move || {
//         println!( "foo" );
//         let val = String::from( "hi" );
//         tx.send( val ).unwrap();
//         println!( "rx, ``{:?}``", rx );
//     } );

// }



// -- misc03()

// use std::thread;

// fn misc03() {

//     let v = vec![1, 2, 3];

//     let handle = thread::spawn( move || {
//         println!( "Here's a vector: ``{:?}``", v );
//     } );

//     handle.join().unwrap();

// }



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



