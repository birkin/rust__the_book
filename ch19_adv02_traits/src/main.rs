fn main() {

    // -- Specifying Placeholder Types in Trait Definitions with Associated Types
    main01();

}



// -- main 01() -- Specifying Placeholder Types in Trait Definitions with Associated Types
/*
 Revisiting Listing 13-15.
 Implementing Iterator uses the associated-type Item.
 */

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main01() {
    let mut counter = Counter::new();

    println!( "counter, {:?}", counter );
    assert_eq!( counter.next(), Some(1) );
    println!( "counter, {:?}", counter );
    assert_eq!( counter.next(), Some(2) );
    println!( "counter, {:?}", counter );
}
