fn main() {

    // /* works, but no inherent linkage between params */
    // let width1 = 30;
    // let height1 = 50;
    // println!(
    //     "The area of the rectangle is {:?} square pixels",
    //     area( width1, height1 )
    //     );



    // /* works, but loses sense of meaning of params in function */
    // let rect1 = (30, 50);
    // println!(
    //     "The area of the rectangle is {:?} square pixels.",
    //     area(rect1) );



    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {:?} square pixels.",
        area(&rect1) );
    println!("rect1 instance, ```{:#?}```", rect1 );  // initially fails, yielding "`Rectangle` doesn't implement `std::fmt::Debug`"
    // println!("rect1, `{}`", rect1 );  // initially fails, yielding "`Rectangle` doesn't implement `std::fmt::Display`"

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area( rectangle: &Rectangle ) -> u32 {
    rectangle.width * rectangle.height
}


// fn area( dimensions: (u32, u32) ) -> u32 {
//     dimensions.0 * dimensions.1
// }



// fn area( width: u32, height: u32 ) -> u32 {
//     width * height
// }
