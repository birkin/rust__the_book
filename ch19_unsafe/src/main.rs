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
}
