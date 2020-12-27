fn main() {

    // -- Creating Type Synonyms with Type Aliases
    // main01();

    // -- Example of what Type Synonyms Save You From
    // main02();

    // -- Above, more readable
    main03();

}


// -- main03() -- Below, more readable

type Thunk = Box< dyn Fn() + Send + 'static >;

fn main03() {
    let _f: Thunk = Box::new( || println!("hi") );

    fn takes_long_type( _f: Thunk ) {
        println!("here01");
    }

    fn returns_long_type() -> Thunk {
        println!("here02");
        Box::new( || () )
    }

    takes_long_type( _f );
    let _z = returns_long_type();
}



// // -- main02() -- Without Type Synonyms

// // -- TODO: read up on Send, and review 'static, and review the || clousure (IIRC)

// fn main02() {

//     let _f: Box< dyn Fn() + Send + 'static > = Box::new( || println!("hi") );
//     // let zz: () = f;  // yields: found struct `std::boxed::Box<(dyn std::ops::Fn() + std::marker::Send + 'static)>`

//     fn takes_long_type( _f: Box< dyn Fn() + Send + 'static > ) {
//         println!("here01");
//     }

//     fn returns_long_type() -> Box< dyn Fn() + Send + 'static > {
//         println!("here02");
//         Box::new( || () )
//     }

//     takes_long_type( _f );
//     let _z = returns_long_type();
// }



// // -- main01() -- Creating Type Synonyms with Type Aliases

// type Kilometers = i32;

// fn main01() {
//     let x: i32 = 5;
//     let y: Kilometers = 5;

//     println!( "x + y = `{}`", x + y  );
// }



