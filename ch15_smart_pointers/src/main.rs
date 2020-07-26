// use crate::List::{Cons, Nil};  // used by misc02()

use std::ops::Deref; // used by misc05() and misc06()


fn main() {

    // -- simple box example
    // misc01();

    // -- recursion example
    // misc02();

    // -- the dereference operator
    // misc03();

    // -- using Box<T> like a reference
    // misc04();

    // -- defining a smart pointer
    misc05();

    // -- implicit deref coercions with functions and methods
    // -- uses the struct MyBox from misc05()
    misc06();

    // -- drop
    misc07();

}


// -- for misc07()

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop ( &mut self ) {
        println!( "Dropping CustomSmartPointer with data ``{:?}``!", self.data );
    }
}

fn misc07() {
    let _c = CustomSmartPointer {
        data: String::from( "my stuff" ),
    };
    let _d = CustomSmartPointer {
        data: String::from( "other stuff" ),
    };
    println!( "CustomSmartPointers created." );
}


fn hello( name: &str ) {
    println!( "Hello, ``{:?}``", name );
}

fn misc06() {
    let m = MyBox::new( String::from("Rust") );
    hello( &m );  // the `&` reference only works because Deref is implemented for MyBox.
}


// -- for misc05() and misc06()
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new( x: T ) -> MyBox<T> {
        MyBox( x )
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref( &self ) -> &T {
        &self.0
    }
}

fn misc05() {
    let x = 5;
    let y = MyBox::new( x );

    assert_eq!( 5, x );
    assert_eq!( 5, *y );
}


// fn misc04() {
//     let x = 5;
//     let y = Box::new(x); // type-check yields: found struct `std::boxed::Box<{integer}>`

//     assert_eq!( 5, x );
//     assert_eq!( 5, *y );
// }


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

