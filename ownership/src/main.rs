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



    /* Doesn't work, because "...We _also_ cannot have a mutable reference
       while we have an immutable one. Users of an immutable reference
       don’t expect the values to suddenly change out from under them!" */
    // let mut s = String::from("hello");

    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;  // no go

    // println!("{:?}, {}, and {}", r1, r2, r3 );


    /* Works, because in the function, `s` goes out-of-scope when the
       function-call is over, but that's ok because it has been moved
       here, to `something`. */
    // let something = no_dangle();
    // println!("something, `{:?}`", something);


    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("word, `{:?}`", word);

    println!("s before clear, `{:?}`", s);
    s.clear(); // this empties the String, making it equal to ""
    println!("s after clear, `{:?}`", s);



    At: <https://doc.rust-lang.org/stable/book/ch04-03-slices.html>
    ```With all this information in mind, let’s rewrite first_word```

}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


// fn no_dangle() -> String {
//     let s = String::from("hello");
//     s
// }



/* This implementation of dangle() doesn't work, because it returns
   a reference to something that goes out-of-scope upon the return! */
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }



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
