fn main() {

    // using raw pointers
    main01();

}



// main01() -- using raw pointers

fn main01() {

    let mut num = 5;
    println!( "num, ``{:?}``", num );

    let r1 = &num as *const i32;
    let r2  = &mut num as *mut i32;

    // let zz: () = r1;  // yields: found raw pointer `*const i32`
    // let zz:  () = r2;  // yields: found raw pointer `*mut i32`

    println!( "r1, ``{:?}``", r1 );
    println!( "r2, ``{:?}``", r2 );

    let address = 0x012345usize;
    let r = address as *const i32;

    println!( "r, ``{:?}``", r );  // works; yields r, ``0x12345``
    // println!( "dereferenced_r, ``{:?}``", *r );  // no -- error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block

    // -- example of unsafe used badly
    // unsafe {
    //     println!( "dereferenced_r, ``{:?}``", *r );  // passes cargo-check, but cargo-run yields `Segmentation fault: 11`
    // }

    // -- example of unsafe used safely
    unsafe {
        println!( "dereferenced-r1, ``{:?}``", *r1 );  // yields: dereferenced-r1, ``5``
        println!( "dereferenced-r2, ``{:?}``", *r2 );  // yields: dereferenced-r2, ``5``
    }

}
