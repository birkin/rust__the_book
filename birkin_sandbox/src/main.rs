
fn main() {

    // -- duration --
    // misc01();

    // -- file paths --
    // misc02();

    // -- last three characters via Vector --
    // misc03();

    // -- last three characters as string --
    misc04();

}

fn misc04() {

    // let text = String::from( "hello_world" );
    // let zz: () = text;  // found struct `std::string::String`

    let text = "hello_world".to_string();
    // let zz: () = text;  // found struct `std::string::String`

    let end = text.len();
    let start = end - 3;

    let slice = &text[ start..end ];

    // let mut new_text = "init".to_string();
    let mut new_text = String::new();
    println!("new_text, ``{:?}``", new_text);

    // if slice == "foo" {
    //     new_text = "bar".to_string();
    // } else if slice == "rld" {
    //     new_text = format!( "{}_02", text );
    // } else {
    //     println!("what happened?");
    // }

    match slice {
        "_01" => {
            new_text = "_02".to_string();
        },
        "_02" => {
            new_text = "_03".to_string();
        },
        _ => {
            println!("HERE!!");
            new_text = format!( "{}_02", text );
        }
    }

    println!("slice, ``{}``", slice);
    println!("new_text, ``{}``", new_text);

}



// fn misc03() {
//     let s = String::from("hello world");

//     let mut last_three: Vec<char> = s.chars().rev().take(3).collect();

//     println!("last_three initially, ``{:?}``", last_three);

//     &last_three.reverse();

//     println!("last_three now, ``{:?}``", last_three);

// }



// fn misc02() {

//     use std::path::Path;
//     use std::ffi::OsStr;

//     // Note: this example does work on Windows
//     let path = Path::new("/aa/bb/foo/barXYZ.txt");

//     let parent = path.parent();
//     // println!("parent, ``{:?}``", parent);
//     assert_eq!( parent, Some(Path::new("/aa/bb/foo")) );

//     let file_stem = path.file_stem();
//     assert_eq!( file_stem, Some(OsStr::new("barXYZ")) );

//     let extension = path.extension();
//     assert_eq!( extension, Some(OsStr::new("txt")) );
// }



// fn misc01() {

//     use std::time::{Duration, Instant};

//     let start_time = Instant::now();

//     let duration: Duration = start_time.elapsed();
//     println!("duration, {:?}", duration); // works: duration, 21ns

//     let message = format!( "duration, {:?}", duration );  // works for "{:?}", not for "{}"
//     println!("message, {:?}", message);

//     // let test_message = format!( "hello, {}", "world" );
//     // println!("test_message, ``{:?}``", test_message);  // works: test_message, ``"hello, world"``

// }
