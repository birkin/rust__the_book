fn main() {

    // -- simple closure working
    misc01();

    // -- equivalent function failing
    // misc02();

    // -- closure with move, prompting fail
    // misc03();

}


// fn misc03() {
//     /*
//         Doesn't compile, because x is moved, and then the println attempts to access it.
//     */

//     let x = vec![1, 2, 3];

//     let equal_to_x = move |z| z == x;

//     println!( "can't use x here: ``{:?}``", x );  // fails here

//     let y = vec![1, 2, 3];

//     assert!( equal_to_x(y) );
// }


// fn misc02() {
//     /*
//         Doesn't compile.
//         Fails with: help: use the `|| { ... }` closure form instead
//     */
//     let x = 4;

//     fn equal_to_x( z: i32 ) -> bool {
//         z == x
//     }

//     let y = 4;

//     assert!( equal_to_x(y) );
// }


fn misc01() {
    let x = 4;

    let equal_to_x = |z| { z == x };

    let y = 4;

    let rslt = equal_to_x( y );
    // let zz: () = rslt;  // yields: expected `()`, found `bool`

    assert_eq!( true, rslt );
}


/* works, because the equal_to_x closure has access to x. */
// fn main() {
//     let x = 4;

//     let equal_to_x = |z| { z == x };

//     let y = 4;

//     assert!( equal_to_x(y) );
// }



/* fails, because x wasn't passed in to the equal_to_x function. */
// fn main() {
//     let x = 4;

//     fn equal_to_x( z: i32 ) -> bool {
//         z == x
//     }

//     let y = 4;

//     assert!( equal_to_x(y) );
// }
