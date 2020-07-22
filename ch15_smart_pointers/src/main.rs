fn main() {
    let b = Box::new(5);  // type check yields: found struct `std::boxed::Box`
    println!( "b, ``{:?}``", b );  // yields: b, ``5``
}
