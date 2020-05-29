#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {

    misc1();
    // misc2();
    // misc3();
    // misc4();
    // let x = misc5();
    // println!("x, ``{:?}``", x);

    let x: Vec<SpreadsheetCell> = misc6();
    println!("x, ``{:#?}``", x);

}

fn misc6() -> Vec<SpreadsheetCell> {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    row
}

// fn misc5() {
//     let mut v = vec![100,32,57];
//     for i in &mut v {
//         *i += 50;
//     }
//     println!("v in misc5, ``{:?}``", v);
//     // v    // can't return v even with adding Vec<&i32> to the signature
//             // returns "missing lifetime specifier",
//             // which I don't know about yet

//     // interesting... that "dereference operator" (*) -- maybe I could
//     // use that to simplify the Config struct
// }



// fn misc4() {
//     let v = vec![100, 32, 57];
//     for i in &v {
//         println!("i, ``{:?}``", i);
//     }
//     // Running `target/debug/collections`
//     // i, ``100``
//     // i, ``32``
//     // i, ``57``
// }



// fn misc3() {

//     let mut v = vec![1, 2, 3, 4, 5];

//     let first = &v[0];

//     v.push(6);

//     println!("first, ``{:?}``", first);

    /*
    The above fails with an explanation at: "...why should a reference
    to the first element care about what changes at the end of the vector?"
    But I can't see how to change it to work. (Tried a few things.)
    */
// }



// fn misc2() {

//     let v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     println!("third, ``{:?}``", third);

    /*
    interesting...

    ``let third: &i32 = &v[8];`` does compile, even though it seems the
    compiler should know this is wrong (especially given that v here is
    not mutable). It then fails with an "index out of bounds" error.

    let zz: () = third;  // shows "found `&i32`", not Option, showing
    that "index out of bounds" errors aren't _required_ to be handled

    Ah; I jumped the gun; the other way of accesing an element does
    return an Option:

    v.get(2); returns `std::option::Option<&i32>`
    */


    // match v.get(2) {
    //     Some(third) => println!("third in match, ``{:?}``", third),
    //     None => println!("no match for requested element")
    // }

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // let second = v[1];
    // println!("second, ``{:?}``", second);

    // let second_2 = v[1];
    // println!("second_2, ``{:?}``", second_2);

    /*
    Why is the default non-get way of accessing a value -- a reference?
    The non-reference way works, too.
    */


// }



fn misc1() {

    let v: Vec<i32> = Vec::new();
    println!("v, ``{:?}``", v);

    let v2 = vec![1, 2, 3];
    println!("v2, ``{:?}``", v2);

    let mut v3 = Vec::new();
    println!("v3, ``{:?}``", v3);
    v3.push(5);
    v3.push(6);
    println!("v3 now, ``{:?}``", v3);
}
