fn main() {

    // misc01();

    // misc02();

    // misc03();

    // misc04();

    // misc05();

    misc06;

}

fn misc06() {
    // -- fails compilation --
    let s1 = String::from("hello");
    let h = s1[0];
}



// fn misc05() {
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     let s = format!("{}-{}-{}", s1, s2, s3);  // no ownership of s1, or s2, or s3 is maintained, so they can be re-used.

//     println!("s, ``{:?}``", s);
//     println!("s1, ``{:?}``", s1);
//     println!("s3, ``{:?}``", s3);
// }



// fn misc04() {
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world");
//     let s3 = s1 + &s2;  // s1 has been moved here and can no longer be used...
//                         // ...because of the way the "+" operator works here.
//     println!("s3, ``{:?}``", s3);
// }



// fn misc03() {
//     // -- this works --
//     let mut s1 = String::from("foo");
//     let s2 = "bar";
//     s1.push_str(s2);
//     println!("s2, ``{:?}``", s2);

//     // -- this only works if the push uses a reference to `ss2`
//     // -- ...because push_str() doesn't take ownership
//     // -- ...of the string it's pushing.
//     let mut ss1 = String::from("foo");
//     let ss2 = String::from("bar");
//     // ss1.push_str(ss2);
//     ss1.push_str(&ss2);
//     println!("ss2, ``{:?}``", ss2);

//     // -- `push()` only takes a single codepoint and requires
//     // -- ...single-quotes
//     ss1.push( 'z' );
//     println!("ss1, ``{:?}``", ss1);

//     // -- output...
//     // -- s2, ``"bar"``
//     // -- ss2, ``"bar"``
//     // -- ss1, ``"foobarz"``

// }



// fn misc02() {
//     let mut s = String::from("foo");
//     s.push_str("bar");
//     println!("s, ``{:?}``", s);
// }



// fn misc01() {
//     let mut s = String::new();
//     // let s = String::new();
//     println!("s, ``{:?}``", s);

//     let data = "initial contents";
//     // let s = data.to_string();
//     s = data.to_string();
//     println!("s, ``{:?}``", s);

//     // let s = "second contents".to_string();
//     s = "second contents".to_string();
//     println!("s, ``{:?}``", s);

//     s = String::from("third_contents");
//     println!("s, ``{:?}``", s);

//     s = String::from("“iñtërnâtiønàlĭzætiФn”");
//     println!("s, ``{:?}``", s);

// }
