fn main() {

    // -- Setting up a TCP listener
    // main01();

    // -- Enhancing the listener
    main02();

}



// -- main02() -- Enhancing the listener

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main02() {
    let addr: &str = "127.0.0.1:7878";
    let listener = TcpListener::bind( addr ).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection( stream );
    }
}

fn handle_connection( mut stream: TcpStream ) {
    let mut buffer = [0; 1024];

    stream.read( &mut buffer ).unwrap();

    let contents: String = fs::read_to_string( "hello.html" ).unwrap();
    // let zz: () = contents;  // yields:

    // println!( "Request: ``{:?}``", String::from_utf8_lossy(&buffer[..]) );
    // let response: &str = "HTTP/1.1 200 OK\r\n\r\n";

    let response: String = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write( response.as_bytes() ).unwrap();
    stream.flush().unwrap();
}



// // -- main01() -- Setting up a TCP listener

// use std::net::TcpListener;

// fn main01() {
//     let addr: &str = "127.0.0.1:7878";
//     // let zz: () = addr;  // yields: expected `()`, found `&str`
//     let listener = TcpListener::bind( addr ).unwrap();

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         println!("Connection established!");
//         println!( "stream, {:?}", stream );
//     }
// }
