use std::io;


fn main() {

    // -- straight panic --
    // misc01();

    // -- out of bounds --
    // misc02();

    // -- handling Result --
    // misc03();

    // -- handling different kinds of errors very explicitly --
    // let fl: std::fs::File = misc04();
    // println!("fl, ``{:?}``", fl);

    // -- the above handled the way an experienced developer --
    // let fl: std::fs::File = misc05();
    // println!("fl, ``{:?}``", fl);

    // -- unwrap-or-else experimentation --
    // misc06();

    // -- propogating errors: verbose & explicit --
    // let x: Result<String, io::Error> = misc07();
    // println!("x, ``{:?}``", x);  // yields: x, ``Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })``
    // // let zz: () = x;  // yields: found enum `std::result::Result<std::string::String, std::io::Error>`

    // -- propogating errors: the `?` operator --
    // let x: Result<String, io::Error> = misc08();
    // println!("x, ``{:?}``", x);  // yields: x, ``Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })``
    // let zz: () = x;  // yields: found enum `std::result::Result<std::string::String, std::io::Error>`

    // -- propogating errors: chaining method calls with the `?` operator --
    let x: Result<String, io::Error> = misc09();
    println!("x, ``{:?}``", x);  // yields: x, ``Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })``
    // let zz: () = x;  // yields: found enum `std::result::Result<std::string::String, std::io::Error>`

}


fn misc09() -> Result<String, io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut s = String::new();
    File::open("foo.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


// fn misc08() -> Result<String, io::Error> {
//     use std::fs::File;
//     use std::io::Read;

//     let mut f = File::open("foo.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }



// fn misc07() -> Result<String, io::Error> {
//     use std::fs::File;
//     use std::io::Read;

//     let f = File::open("foo.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),  // maybe file-not-found
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),  // maybe no-read-permissions
//     }   // I think that because there is no semicolon here...
//         // ...that an Ok would return the string from the file-read
// }



// fn misc06() {
//     use std::fs::File;
//     let f = File::open("helloZZ.txt").unwrap_or_else(
//         |error| {
//             panic!("Problem opening the file: ``{:?}``", error);
//         }
//     );
// }



// fn misc05() -> std::fs::File {
//     use std::fs::File;
//     use std::io::ErrorKind;

//     let f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: ``{:?}``", error);
//             })
//         } else {
//             panic!("Problem opening the file: ``{:?}``", error);
//         }
//     });

//     return f;
// }



// fn misc04() -> std::fs::File {
//     use std::fs::File;
//     use std::io::ErrorKind;

//     let f = File::open("hello.txt");

//     let f = match f {
//         Ok(file) => {
//             println!("file found and opened");
//             return file;
//         },
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => {
//                     println!("file was not found; was created successfully");
//                     fc
//                 },
//                 Err(e) => panic!("Problem creating the file: ``{:?}``", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: ``{:?}``", other_error)
//             }
//         },
//     };
//     // let zz: () = f;  // yields: found struct `std::fs::File`
//     return f;
// }



// fn misc03() {
//     use std::fs::File;
//     let f = File::open( "foo.txt" );
//     // let zz: () = f;  yields: found enum `std::result::Result<std::fs::File, std::io::Error>`

//     let f = match f {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening file: ``{:?}``", error)
//     };
// }



// fn misc02() {
//     let v = vec![1, 2, 3];
//     v[99];
// }



// fn misc01() {
//     panic!( "crash" );
// }
