use ch11_adder;

mod common;


#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!( 4, ch11_adder::add_two(2) );
}
