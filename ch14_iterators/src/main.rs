fn main() {

    // -- initial play
    misc01();

}


fn misc01() {
    let v1 = vec![1, 2, 3];  // yields: struct `std::vec::Vec`
    println!("v1, ``{:?}``", v1);  // v1, ``[1, 2, 3]``

    let v1_iter = v1.iter();  // yields: struct `std::slice::Iter`
    println!("v1_iter, ``{:?}``", v1_iter);  // v1_iter, ``Iter([1, 2, 3])``

    for val in v1_iter {  // val yields: `&{integer}`
        println!( "Got, ``{:?}``", val );
    }

}
