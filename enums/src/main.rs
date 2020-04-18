fn main() {
    // println!("Hello, world!");


    // let four = IPAddrKind::V4;
    // let six = IPAddrKind::V6;

    // println!("four, `{:?}`", four);
    // println!("six, `{:?}`", six);

    // let zz: () = four;

    // let bar = route(IPAddrKind::V4);
    // println!("bar, `{:?}`", bar);


    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("home, ```{:?}```", home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
    println!("loopback, ```{:?}```", loopback);

}


#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

// fn route(ip_kind: IpAddrKind) -> String {
//     println!("ip_kind, `{:?}`", ip_kind);
//     String::from( "foo" )
// }
