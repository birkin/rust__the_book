fn main() {

    // misc01();

    // misc02();

    misc03();

}

fn misc03() {
    // -- this works --
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2, ``{:?}``", s2);

    // -- this only works if the push uses a reference to `ss2`
    // -- ...because push_str() doesn't take ownership
    // -- ...of the string it's pushing.
    let mut ss1 = String::from("foo");
    let ss2 = String::from("bar");
    // ss1.push_str(ss2);
    ss1.push_str(&ss2);
    println!("ss2, ``{:?}``", ss2);

    // -- `push()` only takes a single codepoint and requires
    // -- ...single-quotes
    ss1.push( 'z' );
    println!("ss1, ``{:?}``", ss1);

    // -- output...
    // -- s2, ``"bar"``
    // -- ss2, ``"bar"``
    // -- ss1, ``"foobarz"``

}


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
