fn main() {

    // -- duration --
    // misc01();

    // -- file paths --
    // misc02();

    // -- last three characters via Vector --
    // misc03();

    // -- last three characters as string --
    // misc04();

    // -- load json-file & reading an element from it --
    //    In hindsight, I coud have just created a json string and passed it to fs::read_to_string()...
    //    ...instead of actually loading a file
    // misc05();

    // -- looping through a json-object array --
    misc06();

}

fn misc06() {
    use serde_json::{Value};

    // -- create the object --
    let jsn = r#"[{"aa": "11"}, {"bb": "22"}]"#;
    println!("jsn, ``{:?}``", jsn);  // yields: jsn, ``"[{\"aa\": \"11\"}, {\"bb\": \"22\"}]"`` -- looks good.
    // let zz: () = jsn;  // yields: found `&str`
    let json_obj: Value = serde_json::from_str( &jsn ).unwrap_or_else(|error| {
        panic!("Problem reading the jsn string -- ``{:?}``", error);
    });
    println!("\njson_obj, ``{:?}``", json_obj);  // yields: json_obj, ``Array([Object({"aa": String("11")}), Object({"bb": String("22")})])`` -- good!
    // let zz: () = json_obj;  // yields: found enum `serde_json::value::Value`

    // -- process the object --
    let v = json_obj.as_array().unwrap_or_else(|| {  // as_array() returns Option -- <https://docs.serde.rs/serde_json/value/enum.Value.html#method.as_array>
        panic!("Problem handling json_obj");
    });
    println!("\nv, ``{:?}``", v);  // yields: v, ``[Object({"aa": String("11")}), Object({"bb": String("22")})]``
    /*  This is good. Before the unwrap, I was getting the 'v' contents wrapped in a Some(),
        ... so when I tried iterating through, there was only one item which was the whole thing. */
    // let zz: () = v;  // yields: found reference `&std::vec::Vec<serde_json::value::Value>`
    for item in v {
        println!("item, ``{:?}``", item);
    }
    /*  good! yields...
        item, ``Object({"aa": String("11")})``
        item, ``Object({"bb": String("22")})`` */

}



// fn misc05() {
//     use std::env;

//     // let mut cwd_path = String::new();

//     /* -- get the current working directory as a PathBuf struct -- */
//     let cwd = env::current_dir();
//     let cwd: std::path::PathBuf = match cwd {
//         Ok( the_cwd ) => the_cwd,
//         Err( the_err ) => panic!( "problem perceiving the cwd: ``{:?}``", the_err ),
//     };
//     println!("\ncwd, ``{:?}``", cwd);  // yields: cwd, ``"/the/cwd/path"``
//     // let zz: () = cwd;  // yields: found struct `std::path::PathBuf`

//     /* -- update the path-var to the json file to an &str -- */
//     let cwd_str = cwd.to_str().unwrap();
//     println!("\ncwd_str, ``{:?}``", cwd_str);  // yields: cwd_str, ``"/the/cwd/path"``
//     // let zz: () = cwd_str;  // yields:  expected `()`, found `&str`

//     /* -- create the full-path to the json-directory-list file -- */
//     let full_path = cwd_str.to_owned() + "/../../log_file_list/log_list.json";
//     println!("\nfull_path, ``{:?}`", full_path);
//     let full_path_str = full_path.as_str();
//     println!("\nfull_path_str, ``{:?}``", full_path_str);
//     // let zz: () = full_path_str;  // yields: found `&str` -- good!

//     /* -- read json file into String -- */
//     use std::fs;

//     let jsn = fs::read_to_string( full_path_str );

//     let jsn = match jsn {
//         Ok( the_json_string ) => the_json_string,
//         Err( the_err ) => panic!( "problem loading json-file: ``{:?}``", the_err ),
//     };
//     println!( "\njsn, ``{:?}``", jsn );
//     // let zz: () = jsn;  // yields: found struct `std::string::String`

//     /* -- read json String into json Object -- */
//     use serde_json::{Value};

//     // let directory_lst: Value = serde_json::from_str( &jsn ).unwrap();  // serde_json::value::Value -- Array([Object({"path": String("/foo/bar.log")}),...
//     let directory_lst: Value = serde_json::from_str( &jsn ).unwrap_or_else(|error| {
//         panic!("Problem reading the jsn string -- perhaps invalid json? -- ``{:?}``", error);
//     });
//     println!("\ndirectory_lst, ``{:?}``", directory_lst);
//     // let zz: () = directory_lst;  // yields: found enum `serde_json::value::Value`

//     /* -- get the first path-element -- */
//     let path01 = &directory_lst[0]["path"];
//     println!("\npath01, ``{:?}`", path01);  // yields: path01, ``String("/foo/the.log")`
//     // let zz: () = path01;  yields: found `&serde_json::value::Value`

//     /* -- get the first path-element as a String or &str -- */
//     let path01_str = path01.as_str().unwrap_or_default();  // 'as_str() returns an Option, so this returns the value, or nothing.
//     println!("\npath01_str, ``{:?}``", path01_str); // yields: path01_str, ``"/foo/the.log"``
//     // let zz: () = path01_str;  // yields: expected `()`, found `&str`  👍

// }



// fn misc04() {

//     // let text = String::from( "hello_world" );
//     // let zz: () = text;  // found struct `std::string::String`

//     let text = "hello_world".to_string();
//     // let zz: () = text;  // found struct `std::string::String`

//     let end = text.len();
//     let start = end - 3;

//     let slice = &text[ start..end ];

//     // let mut new_text = "init".to_string();
//     let mut new_text = String::new();
//     println!("new_text, ``{:?}``", new_text);

//     // if slice == "foo" {
//     //     new_text = "bar".to_string();
//     // } else if slice == "rld" {
//     //     new_text = format!( "{}_02", text );
//     // } else {
//     //     println!("what happened?");
//     // }

//     match slice {
//         "_01" => {
//             new_text = "_02".to_string();
//         },
//         "_02" => {
//             new_text = "_03".to_string();
//         },
//         _ => {
//             println!("HERE!!");
//             new_text = format!( "{}_02", text );
//         }
//     }

//     println!("slice, ``{}``", slice);
//     println!("new_text, ``{}``", new_text);

// }



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
