fn main() {

    // -- Creating Type Synonyms with Type Aliases
    main01();

}


// -- main01() -- Creating Type Synonyms with Type Aliases

fn main01() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!( "x + y = `{}`", x + y  );
}
