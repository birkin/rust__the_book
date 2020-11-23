fn main() {

    // -- conditional `if let` expressions
    // main01();

    // -- while let Conditional Loops
    // main02()

    // -- for-loops
    main03();
}



// -- main03() -- for-loops
fn main03() {
    let v = vec!['a', 'b', 'c'];

    for ( index, value ) in v.iter().enumerate() {
        println!( "``{}`` is at index ``{}``", value, index );
    }
}



// -- main02() -- while let Conditional Loops

// fn main02() {
//     let mut stack = Vec::new();

//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     while let Some( top ) = stack.pop() {
//         println!( "top, ``{:?}``", top );
//     }
// }



// -- main01() -- conditional `if let` expressions

// use std::num::ParseIntError;

// fn main01() {
//     let favorite_color: Option< &str > = None;
//     // let favorite_color: Option< &str > = Some( "blue" );
//     let is_tuesday = false;
//     // let is_tuesday = true;
//     // let age: Result< u8, _ > = "34".parse();  // as written
//     // let age: Result< u8, ParseIntError > = "34".parse();  // reminding myself of Result output
//     let age: Result< u8, ParseIntError > = "foo".parse();  // reminding myself of Result output
//     println!( "age, ``{:?}``", age );

//     if let Some( color ) = favorite_color {
//         println!( "Using your favorite color, ``{:?}``, as the background", color );
//     } else if is_tuesday {
//         println!( "Tuesday is green day!" );
//     } else if let Ok( age ) = age {
//         if age > 30 {
//             println!( "Using purple as the background color" );
//         } else {
//             println!( "Using orange as the background color" );
//         }
//     } else {
//         println!( "Using blue as the background color" );
//     }

// }
