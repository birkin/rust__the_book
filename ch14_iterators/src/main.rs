fn main() {

    // -- initial play
    // misc01();

    // -- map iterator
    misc02();

}


fn misc02() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map( |x| x + 1 ).collect();

    assert_eq!( v2, vec![2, 3, 4] );
}


// fn misc01() {
//     let v1 = vec![1, 2, 3];  // yields: struct `std::vec::Vec`
//     println!("v1, ``{:?}``", v1);  // v1, ``[1, 2, 3]``

//     let v1_iter = v1.iter();  // yields: struct `std::slice::Iter`
//     println!("v1_iter, ``{:?}``", v1_iter);  // v1_iter, ``Iter([1, 2, 3])``

//     for val in v1_iter {  // val yields: `&{integer}`
//         println!( "Got, ``{:?}``", val );
//     }

// }



/*

Notes...

- An example of an object that, if I recall correctly, is a kind of iterator object -- that wouldn't print via the normal println!() technique, and how I got it to print:

    <https://github.com/birkin/rust_marc_experimentation_code/blob/914be66aa5f0b084fbd219ae51194d0fadb0c7b0/exp01_list_marc_files/src/main.rs#L12-L21>

 */
