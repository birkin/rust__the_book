
/* works, because the equal_to_x closure has access to x. */
fn main() {
    let x = 4;

    let equal_to_x = |z| { z == x };

    let y = 4;

    assert!( equal_to_x(y) );
}



/* fails, because x wasn't passed in to the equal_to_x function. */
// fn main() {
//     let x = 4;

//     fn equal_to_x( z: i32 ) -> bool {
//         z == x
//     }

//     let y = 4;

//     assert!( equal_to_x(y) );
// }
