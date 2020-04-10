fn main() {

    // let mut s = String::from("hello");
    // s.push_str(", world!");  // appends
    // println!("{:?}", s);



    // let x = 5;
    // let y = x;
    // println!("x, {:?}", x);
    // println!("y, {:?}", y);  // all ok, but only because "... types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make..."



    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("s1, {:?}", s1);  // error: `value borrowed here after move`
    // println!("s2, {:?}", s2);



    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1, {:?}", s1); // error: `value borrowed here after move`
    // println!("s2, {:?}", s2);



    // let s = String::from("hello");
    // takes_ownership(s);
    // // println!("s, {:?}", s);  // no, can't be called, because `s` value has moved into the function
    // let x = 5;
    // makes_copy(x);
    // println!("x, {:?}", x);  // basic i32 type has a Copy trait, so x is still accessible



    // let s1 = gives_ownership();
    // println!("s1, ```{:?}```", s1);
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);
    // println!("s3, ```{:?}```", s3);



    // let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    // println!("The length of `{:?}` is `{:?}`", s2, len);



    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of `{:?}` is `{}`", s1, len);


    /* Doesn't work because "Just as variables are immutable by default,
       so are references. We’re not allowed to modify something we have
       a reference to." */
    // let s = String::from("hello");
    // change( &s );
    // println!("s is now, `{:?}`", s);


    /* Works after making s mutable. */
    // let mut s = String::from("hello");
    // change( &mut s );
    // println!("s is now, `{:?}`", s);


    /* Doesn't work because "...mutable references have one big restriction:
       you can have only one mutable reference to a particular piece of data
       in a particular scope... ...The benefit of having this restriction
       is that Rust can prevent data races at compile time." */
    // let mut s = String::from( "hello" );

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{:?}, {}", r1, r2 );


    /* Works, because r1 goes out of scope,
       but of course then we can't print it at the end. */
    // let mut s = String::from( "hello" );
    // {
    //     let r1 = &mut s;
    // }  // r1 goes out of scope here, so ok to define r2 in same way.
    // let r2 = &mut s;



    At: <https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html#mutable-references>

    "A similar rule exists..."
}

// fn change( some_string: &mut String ) {
//     some_string.push_str( ", world" );
//     println!("some_string is now, `{:?}`", some_string);
// }



// fn change( some_string: &String ) {
//     some_string.push_str( ", world" );
//     println!("some_string is now, `{:?}`", some_string);
// }



// fn calculate_length(s: &String) -> usize {
//     s.len()
// }



// fn calculate_length( s: String ) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }



// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }



// fn takes_ownership( some_string: String ) {
//     println!("some_string, {:?}", some_string);
// }

// fn makes_copy( some_integer: i32 ) {
//     println!("some_integer, {:?}", some_integer );
// }
