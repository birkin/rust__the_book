fn main() {

    // misc1();

    misc2();

}


fn misc2() {

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("third, ``{:?}``", third);

    /*
    interesting...

    ``let third: &i32 = &v[8];`` does compile, even though it seems the
    compiler should know this is wrong (especially given that v here is
    not mutable). It then fails with an "index out of bounds" error.

    let zz: () = third;  // shows "found `&i32`", not Option, showing
    that "index out of bounds" errors aren't _required_ to be handled

    */


    match v.get(2) {
        Some(third) => println!("third in match, ``{:?}``", third),
        None => println!("no match for requested element")
    }
}


// fn misc1() {

//     let v: Vec<i32> = Vec::new();
//     println!("v, ``{:?}``", v);

//     let v2 = vec![1, 2, 3];
//     println!("v2, ``{:?}``", v2);

//     let mut v3 = Vec::new();
//     println!("v3, ``{:?}``", v3);
//     v3.push(5);
//     v3.push(6);
//     println!("v3 now, ``{:?}``", v3);
// }
