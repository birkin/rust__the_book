fn main() {

    // -- Show limitations of single-threading.
    // main01();

    // -- (temp) Unlimited threads
    main02();

}



// main02()-- (temp) Unlimited threads

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main02() {
    let addr: &str = "127.0.0.1:7878";
    let listener = TcpListener::bind( addr ).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!( "starting stream, ``{:?}``", stream );
        /* note
        - the above yields: starting stream, ``TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:PORTNUM, fd: 4 }``
        - so, that 'peer' PORTNUM can be read, later, to be able to tell exactly which thread completed and when.
        */

        thread::spawn( || {
            handle_connection( stream );
        });
    }
}

fn handle_connection( mut stream: TcpStream ) {
    let mut buffer = [0; 1024];
    stream.read( &mut buffer ).unwrap();

    let root_path: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    let sleep_path: &[u8; 21] = b"GET /sleep HTTP/1.1\r\n";

    let response_status_line: &str;
    let filename: &str;

    if buffer.starts_with( root_path ) {
        println!( "detected root path" );
        response_status_line = "HTTP/1.1 200 OK\r\n\r\n";
        filename = "hello.html";
    } else if buffer.starts_with( sleep_path ) {
        println!( "detected sleep path" );
        thread::sleep( Duration::from_secs(5) );
        response_status_line = "HTTP/1.1 200 OK\r\n\r\n";
        filename = "hello.html";
    } else {
        println!( "detected other path" );
        response_status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        filename = "404.html";
    }

    println!( "ending stream, ``{:?}``", stream );
    /* note
    - the above yields: starting stream, ``TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:PORTNUM, fd: 4 }``
    - I've read this 'peer' PORTNUM above, in main02(), so this clearly shows how the sleeping thread executes after the time-delay.
    */

    let contents: String = fs::read_to_string( filename ).unwrap();
    let response: String = format!( "{}{}", response_status_line, contents );

    stream.write( response.as_bytes() ).unwrap();
    stream.flush().unwrap();

}



// // main01() -- Single-threading limitation example

// use std::fs;
// use std::io::prelude::*;
// use std::net::TcpListener;
// use std::net::TcpStream;
// use std::thread;
// use std::time::Duration;

// fn main01() {
//     let addr: &str = "127.0.0.1:7878";
//     let listener = TcpListener::bind( addr ).unwrap();

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         handle_connection( stream );
//     }
// }

// fn handle_connection( mut stream: TcpStream ) {
//     let mut buffer = [0; 1024];
//     stream.read( &mut buffer ).unwrap();

//     let root_path: &[u8; 16] = b"GET / HTTP/1.1\r\n";
//     let sleep_path: &[u8; 21] = b"GET /sleep HTTP/1.1\r\n";

//     let response_status_line: &str;
//     let filename: &str;

//     if buffer.starts_with( root_path ) {
//         println!( "detected root path" );
//         response_status_line = "HTTP/1.1 200 OK\r\n\r\n";
//         filename = "hello.html";
//     } else if buffer.starts_with( sleep_path ) {
//         println!( "detected sleep path" );
//         thread::sleep( Duration::from_secs(5) );
//         response_status_line = "HTTP/1.1 200 OK\r\n\r\n";
//         filename = "hello.html";
//     } else {
//         println!( "detected other path" );
//         response_status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
//         filename = "404.html";
//     }

//     let contents: String = fs::read_to_string( filename ).unwrap();
//     let response: String = format!( "{}{}", response_status_line, contents );

//     stream.write( response.as_bytes() ).unwrap();
//     stream.flush().unwrap();

// }
