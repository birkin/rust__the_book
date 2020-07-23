use crate::List::{Cons, Nil};


fn main() {

    // -- simple box example
    // misc01();

    // -- recursion example
    misc02();

}


#[derive(Debug)]
enum List {
    Cons( i32, Box<List> ),
    Nil,
}

fn misc02() {
    let list = Cons( 1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))) );
    // let z: () = list;  // yields: found enum `List`
    println!( "list, ``{:?}``", list );  // output: list, ``Cons(1, Cons(2, Cons(3, Nil)))``
}



/*  Normally, an i32 value would be stored on the stack.
    Here, `b` has the value of a `Box` which is stored on the stack,
    and which points to the value of `5`, which is stored on the heap.
 */
// fn misc01() {
//     let b = Box::new(5);  // type check yields: found struct `std::boxed::Box`
//     println!( "b, ``{:?}``", b );  // yields: b, ``5``
// }

