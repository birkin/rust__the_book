// use crate::List::{Cons, Nil};  // used by misc02()


fn main() {

    // -- simple box example
    // misc01();

    // -- recursion example
    // misc02();

    // -- the dereference operator
    // misc03();

    // -- using Box<T> like a reference
    misc04();

}


fn misc04() {
    let x = 5;
    let y = Box::new(x); // type-check yields: found struct `std::boxed::Box<{integer}>`

    assert_eq!( 5, x );
    assert_eq!( 5, *y );
}


// fn misc03() {
//     let x = 5;  // type-check yields: found integer
//     let y = &x;  // type-check yields: found `&{integer}`

//     assert_eq!( 5, x );
//     assert_eq!( 5, *y );
// }


// -- used by misc02()
// #[derive(Debug)]
// enum List {
//     Cons( i32, Box<List> ),
//     Nil,
// }

// fn misc02() {
//     let list = Cons( 1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))) );
//     // let z: () = list;  // yields: found enum `List`
//     println!( "list, ``{:?}``", list );  // output: list, ``Cons(1, Cons(2, Cons(3, Nil)))``
// }



/*  Normally, an i32 value would be stored on the stack.
    Here, `b` has the value of a `Box` which is stored on the stack,
    and which points to the value of `5`, which is stored on the heap.
 */
// fn misc01() {
//     let b = Box::new(5);  // type check yields: found struct `std::boxed::Box`
//     println!( "b, ``{:?}``", b );  // yields: b, ``5``
// }

