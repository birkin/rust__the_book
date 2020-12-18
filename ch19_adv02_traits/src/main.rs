fn main() {

    // -- Specifying Placeholder Types in Trait Definitions with Associated Types
    // main01();

    // -- Default Generic Type Parameters and Operator Overloading
    // main02();

    // -- Customizing the `Rhs` of the Output
    main03();

}



// -- main03()

use std::ops::Add;

# [derive( Debug, PartialEq )]
struct Millimeters( u32 );

struct Meters( u32 );

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add( self, other: Meters ) -> Millimeters {
        Millimeters( self.0 + (other.0 * 1000) )
    }
}

fn main03() {
    let summed_millimeters: Millimeters = Millimeters(500) + Meters( 1 );
    println!( "summed_millimeters, ``{:?}``", summed_millimeters );
    assert_eq!( summed_millimeters, Millimeters(1500) );
}



// // -- main02() -- Default Generic Type Parameters and Operator Overloading

// use std::ops::Add;

// # [derive( Debug, PartialEq )]
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Add for Point {
//     type Output = Point;

//     fn add( self, other: Point ) -> Point {
//         println!( "hereB" );
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// fn main02() {
//     println!( "hereA" );
//     let added_p: Point = Point{ x: 1, y: 0 } + Point{ x: 2, y: 3 };
//     println!( "hereC" );
//     assert_eq!(
//         added_p,
//         Point{ x: 3, y: 3 }
//     );

// }



// // -- main 01() -- Specifying Placeholder Types in Trait Definitions with Associated Types
// /*
//  Revisiting Listing 13-15.
//  Implementing Iterator uses the associated-type Item.
//  */

// #[derive(Debug)]
// struct Counter {
//     count: u32,
// }

// impl Counter {
//     fn new() -> Counter {
//         Counter { count: 0 }
//     }
// }

// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         // --snip--
//         if self.count < 5 {
//             self.count += 1;
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }

// fn main01() {
//     let mut counter = Counter::new();

//     println!( "counter, {:?}", counter );
//     assert_eq!( counter.next(), Some(1) );
//     println!( "counter, {:?}", counter );
//     assert_eq!( counter.next(), Some(2) );
//     println!( "counter, {:?}", counter );
// }
