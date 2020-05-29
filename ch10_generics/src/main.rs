fn main() {

    // -- functions ripe for refactoring to generics
    // misc01();

    // -- above converted to use generics
    // misc02();  // doesn't yet compile due to error book says we'll address later

    // -- generics in structs
    // misc03();

    // -- generics in structs allowing for different types
    // misc04();

    // -- example of generics in structs, _and_ in a struct-method
    misc05();
}

fn misc05() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup( p2 );
    println!("p3.x, ``{:?}``; p3.y, ``{:?}``", p3.x, p3.y );
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V,W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}


// fn misc04() {
//     let both_integer = Point { x: 5, y: 10 };
//     let both_float = Point { x: 1.0, y: 4.0 };
//     let integer_and_float = Point { x: 5, y: 4.0 };

//     println!("both_integer-point, ``{:?}``", both_integer);
//     println!("both_float-point, ``{:?}``", both_float);
//     println!("integer_and_float-point, ``{:?}``", integer_and_float);
// }

// #[derive(Debug)]
// struct Point<T, U> {
//     x: T,
//     y: U,
// }



// fn misc03() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };

//     // let wont_work = Point { x: 5, y: 4.0 };  // yields: expected integer, found floating-point number

//     println!("integer-point, ``{:?}``", integer);
//     println!("float-point, ``{:?}``", float);
// }

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }



// fn misc02() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest( &number_list );
//     println!("The largest number is ``{:?}``", result);

//     let char_list = vec![ 'y', 'm', 'a', 'q'];
//     let result = largest( &char_list );
//     println!("The largest char is ``{:?}``", result);

// }

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }



// fn misc01() {

//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest_i32( &number_list );
//     println!("The largest number is ``{:?}``", result);

//     let char_list = vec![ 'y', 'm', 'a', 'q'];
//     let result = largest_char( &char_list );
//     println!("The largest char is ``{:?}``", result);

// }

// fn largest_i32( list: &[i32] ) -> i32 {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char( list: &[char] ) -> char {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
