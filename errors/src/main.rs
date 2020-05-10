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
    misc06();

}

fn misc06() {
    use std::fs::File;
    let f = File::open("helloZZ.txt").unwrap_or_else(
        |error| {
            panic!("Problem opening the file: ``{:?}``", error);
        }
    );
}



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
