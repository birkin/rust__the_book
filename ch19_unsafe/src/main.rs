fn main() {

    // -- using raw pointers
    // main01();

    // -- calling unsafe functions
    // main02();

    // -- calling external code
    // main03();

    // -- Accessing or Modifying a Mutable Static Variable
    main04();

}



// -- main 04() -- Accessing or Modifying a Mutable Static Variable

static STATIC_NAME: &str = "Foo";

static mut COUNTER: u32 = 2;  // (1) static variables, unlike constants, can be mutable. (2) a "safe" way of doing this would be to use a Mutex, as shown in <https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html#using-mutexes-to-allow-access-to-data-from-one-thread-at-a-time>

fn main04() {
    println!( "static-name is ``{:?}``", STATIC_NAME );
    // let zz: () = STATIC_NAME;  // yields: found `&str`

    add_to_count( 3 );

    unsafe {
        println!( "COUNTER, ``{:?}``", COUNTER );
    }
}

fn add_to_count( increment: u32 ) {
    unsafe {
        COUNTER += increment;
    }
}



// -- main03() -- calling external code

// fn main03() {
//     unsafe {
//         println!( "Absolute value of -3 according to C: ``{:?}``", abs(-3) );
//     }
// }

// extern "C" {
//     fn abs( input: i32 ) -> i32;
// }



// -- main02() -- calling unsafe functions

// fn main02() {
//     unsafe {
//         let result = dangerous();
//         println!( "result, ``{:?}``", result );
//     }
// }

// unsafe fn dangerous() -> std::string::String {
//     String::from( "be careful")
// }



// -- main01() -- using raw pointers

// fn main01() {

//     let mut num = 5;
//     println!( "num, ``{:?}``", num );

//     let r1 = &num as *const i32;
//     let r2  = &mut num as *mut i32;

//     // let zz: () = r1;  // yields: found raw pointer `*const i32`
//     // let zz:  () = r2;  // yields: found raw pointer `*mut i32`

//     println!( "r1, ``{:?}``", r1 );
//     println!( "r2, ``{:?}``", r2 );

//     let address = 0x012345usize;
//     let r = address as *const i32;

//     println!( "r, ``{:?}``", r );  // works; yields r, ``0x12345``
//     // println!( "dereferenced_r, ``{:?}``", *r );  // no -- error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block

//     // -- example of unsafe used badly
//     // unsafe {
//     //     println!( "dereferenced_r, ``{:?}``", *r );  // passes cargo-check, but cargo-run yields `Segmentation fault: 11`
//     // }

//     // -- example of unsafe used safely
//     unsafe {
//         println!( "dereferenced-r1, ``{:?}``", *r1 );  // yields: dereferenced-r1, ``5``
//         println!( "dereferenced-r2, ``{:?}``", *r2 );  // yields: dereferenced-r2, ``5``
//     }

// }
