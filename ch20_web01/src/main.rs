fn main() {

    // -- Setting up a TCP listener
    // main01();

    // -- Enhancing the listener
    // main02();

    // -- Selectively responding
    // main03();

    // -- Refactoring the response
    main04();

}



// main04() -- Refactoring the response
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main04() {
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

    let get_root: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    // let zz: () = get_root;  // yields: expected `()`, found `&[u8; 16]`

    let response_status_line: &str;
    let filename: &str;

    if buffer.starts_with( get_root ) {
        println!( "Detected root path" );
        response_status_line = "HTTP/1.1 200 OK\r\n\r\n";
        filename = "hello.html";
    }
    else {
        println!( "Detected alternate path" );
        response_status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        filename = "404.html";
    }

    let contents: String = fs::read_to_string( filename ).unwrap();
    let response: String = format!( "{}{}", response_status_line, contents );

    stream.write( response.as_bytes() ).unwrap();
    stream.flush().unwrap();

}



// // main03() -- Selectively responding

// use std::fs;
// use std::io::prelude::*;
// use std::net::TcpListener;
// use std::net::TcpStream;

// fn main03() {
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

//     let get_root: &[u8; 16] = b"GET / HTTP/1.1\r\n";
//     // let zz: () = get_root;  // yields: expected `()`, found `&[u8; 16]`

//     if buffer.starts_with( get_root ) {
//         println!( "Detected root path" );
//         let contents: String = fs::read_to_string( "hello.html" ).unwrap();

//         let response: String = format!(
//             "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//             contents.len(),
//             contents
//         );

//         println!( "Returning good response.", );
//         stream.write( response.as_bytes() ).unwrap();
//         stream.flush().unwrap();

//     } else {
//         println!( "Detected some other path" );
//         let status_line: &str = "HTTP/1.1 404 NOT FOUND\r\n";
//         let contents: String = fs::read_to_string( "404.html" ).unwrap();

//         let response: String = format!( "{}{}", status_line, contents );

//         println!( "Returning not-found response.", );
//         stream.write( response.as_bytes() ).unwrap();
//         stream.flush().unwrap();

//     }

// }



// // -- main02() -- Enhancing the listener

// use std::fs;
// use std::io::prelude::*;
// use std::net::TcpListener;
// use std::net::TcpStream;

// fn main02() {
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

//     let contents: String = fs::read_to_string( "hello.html" ).unwrap();
//     // let zz: () = contents;  // yields:

//     // println!( "Request: ``{:?}``", String::from_utf8_lossy(&buffer[..]) );
//     // let response: &str = "HTTP/1.1 200 OK\r\n\r\n";

//     let response: String = format!(
//         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//         contents.len(),
//         contents
//     );

//     stream.write( response.as_bytes() ).unwrap();
//     stream.flush().unwrap();
// }



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
