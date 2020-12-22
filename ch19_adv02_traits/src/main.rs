fn main() {

    // -- Specifying Placeholder Types in Trait Definitions with Associated Types
    // main01();

    // -- Default Generic Type Parameters and Operator Overloading
    // main02();

    // -- Customizing the `Rhs` of the Output for operator-overloading
    // main03();

    // -- Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
    // main04();

    // -- Disambiguating methods with the same name with associated-types that are part of traits, which don't have a `self` parameter
    // main05();

    // -- Using Supertraits
    // main06();

    // -- Implementing External Traits on External Types
    main07();
}



// -- main07() -- Implementing External Traits on External Types

use std::fmt;

struct Wrapper( Vec<String> );

impl fmt::Display for Wrapper {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "[{}]", self.0.join(", ") )
    }
}

fn main07() {
    let w = Wrapper( vec![String::from("hello"), String::from("world")] );
    println!( "w = ``{}``", w);
}



// // -- main06() -- Using Supertraits

// use std::fmt;

// trait OutlinePrint: fmt::Display {
//     fn outline_print( &self ) {
//         let output = self.to_string();
//         let len = output.len();
//         println!( "{}", "*".repeat(len + 4) );
//         println!( "*{}*", " ".repeat(len + 2) );
//         println!( "* {} *", output );
//         println!( "*{}*", " ".repeat(len + 2) );
//         println!( "{}", "*".repeat(len + 4) );
//     }
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl OutlinePrint for Point {}

// impl fmt::Display for Point {
//     fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
//         write!( f, "({}, {})", self.x, self.y )
//     }
// }

// fn main06() {
//     let p = Point{ x: 111, y: 333 };
//     p.outline_print();
// }



// // -- main05() -- Disambiguating methods with the same name with associated-types that are part of traits, which don't have a `self` parameter

// trait Animal {
//     fn baby_name() -> String;
// }

// struct Dog;

// impl Dog {
//     fn baby_name() -> String {
//         String::from( "Spot" )
//     }
// }

// impl Animal for Dog {
//     fn baby_name() -> String {
//         String::from( "puppy" )
//     }
// }

// fn main05() {
//     println!( "A baby dog is called a ``{:?}``", Dog::baby_name() );  // compiles, but wrong; outputs: A baby dog is called a ``"Spot"``
//     println!( "A baby dog is called a ``{:?}``", <Dog as Animal>::baby_name() );  // what we want; outputs: A baby dog is called a ``"puppy"``
// }



// // -- main04() -- Disambiguating methods with the same name

// trait Pilot {
//     fn fly( &self );
// }

// trait Wizard {
//     fn fly( &self );
// }

// struct Human;

// impl Pilot for Human {
//     fn fly( &self ) {
//         println!( "This is your captain speaking" );
//     }
// }

// impl Wizard for Human {
//     fn fly( &self ) {
//         println!( "Up!" );
//     }
// }

// impl Human {
//     fn fly( &self ) {
//         println!( "*** waving arms furiosuly ***" );
//     }
// }

// fn main04() {
//     let person = Human;
//     person.fly();           // outputs ``*** waving arms furiosuly ***``
//     Pilot::fly( &person );  // outputs ``This is your captain speaking``
//     Wizard::fly( &person ); // outputs ``Up!``
// }



// -- main03() -- Customizing the `Rhs` of the Output for operator-overloading

// use std::ops::Add;

// # [derive( Debug, PartialEq )]
// struct Millimeters( u32 );

// struct Meters( u32 );

// impl Add<Meters> for Millimeters {
//     type Output = Millimeters;

//     fn add( self, other: Meters ) -> Millimeters {
//         Millimeters( self.0 + (other.0 * 1000) )
//     }
// }

// fn main03() {
//     let summed_millimeters: Millimeters = Millimeters(500) + Meters( 1 );
//     println!( "summed_millimeters, ``{:?}``", summed_millimeters );
//     assert_eq!( summed_millimeters, Millimeters(1500) );
// }



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
