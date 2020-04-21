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



    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // // let zz: () = y;

    // println!("x, `{:?}`; y, `{:?}`", x , y);
    // // works; yields: """x, `5`; y, `Some(5)`""" -- but can't add x y



    // let nckl = Coin::Nickel;  // works
    // let value = value_in_cents( nckl );
    // println!("value, `{:?}`", value);

    // let value = value_in_cents( Coin::Quarter(UsState::Alaska) );  // works
    // println!("value, `{:?}`", value);

    // assert_eq!( nckl, Coin::Nickel );  // hmm... fails with "error[E0369]: binary operation `==` cannot be applied to type `Coin`" -- and -- "note: an implementation of `std::cmp::PartialEq` might be missing for `Coin`"
    // let zz: () = nckl; // "found enum `Coin`"
    // let zz: () = Coin::Nickel; // "found enum `Coin`"



    // let five = Some(5);
    // let six = plus_one(five);
    // println!("six, ``{:?}``", six);

    // let none = plus_one(None);
    // println!("none, ``{:?}``", none);



    let some_u8_value = 0u8;
    println!("some_u8_value, ``{:?}``", some_u8_value);

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three" ),
        _ => (),
    }

}



// fn plus_one(x: Option<i32>) -> Option<i32> {
//     println!("x, ```{:?}```", x);
//     match x {
//         None => None,
//         Some(i) => {
//             println!("i, ``{:?}``", i);
//             Some( i + 1)
//         }
//     }
// }



// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     Delaware,
// }

// #[derive(Debug)]
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => {
//             println!("found a nickle");
//             5
//         },
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("Has state, `{:?}`", state);
//            25
//         },
//     }
// }



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
