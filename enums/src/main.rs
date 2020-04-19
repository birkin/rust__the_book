fn main() {
    // println!("Hello, world!");



    // let four = IPAddrKind::V4;
    // let six = IPAddrKind::V6;

    // println!("four, `{:?}`", four);
    // println!("six, `{:?}`", six);

    // let zz: () = four;

    // let bar = route(IPAddrKind::V4);
    // println!("bar, `{:?}`", bar);



    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // println!("home, ```{:?}```", home);

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };
    // println!("loopback, ```{:?}```", loopback);



    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // println!("home, ```{:?}```", home);

    // let loopback = IpAddr::V6(String::from("::1"));
    // println!("loopback, ```{:?}```", loopback);



    // let m = Message::Write(String::from("hello"));
    // // let zz: () = m;

    // m.call();



    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let zz: () = y;

    println!("x, `{:?}`; y, `{:?}`", x , y);
    // works; yields: """x, `5`; y, `Some(5)`""" -- but can't add x y

}

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         println!("foo");
//     }
// }



// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }



// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String
// }



// fn route(ip_kind: IpAddrKind) -> String {
//     println!("ip_kind, `{:?}`", ip_kind);
//     String::from( "foo" )
// }
