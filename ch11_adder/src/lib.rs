// -- Guess

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1; got ``{}``.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100; got ``{}``.",
                value
            );
        }

        Guess { value }
    }
}


// -- Rectangle

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold( &self, other: &Rectangle ) -> bool {
        self.width > other.width && self.height > other.height
    }
}


// -- pub functions

pub fn add_two(a: i32) -> i32 {
    a + 2
}


pub fn greeting( name: &str ) -> String {
    format!( "Hello {}!", name )
}


fn prints_and_returns_10( a: i32 ) -> i32 {
    println!( "I got the value, ``{:?}``", a );
    10
}


// -- tests

#[cfg(test)]
mod tests {
    use super::*;

    // -- Guess {}

    #[test]
    #[should_panic( expected = "Guess value must be less than or equal to 100" )]
    fn greater_than_100() {
        Guess::new( 200 );
    }

    #[test]
    fn should_be_ok() {
        let g = Guess::new( 50 );
        // println!("g, ``{:?}``", g);
        assert_eq!( 50, g.value );
    }


    // -- greeting()

    #[test]
    fn should_contain_name() {
        let result = greeting( "Carol" );
        // assert!( result.contains("Carol") );
        assert!(
            result.contains( "Carol" ),
            "Greeting did not contain name; value was ``{}``", result
        );
    }


    // -- add_two()

    #[test]
    fn add_two_and_two() {
        assert_eq!( 4, add_two(2) );
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!( 5, add_two(3) );
    }

    #[test]
    #[ignore]
    fn one_hundred() {
        assert_eq!( 102, add_two(100) );
    }


    // -- Rectangle {}

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        let check = larger.can_hold( &smaller );
        println!("check, ``{:?}``", check);
        // assert!( larger.can_hold(&smaller) );
        assert_eq!( true, check );
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!( !smaller.can_hold(&larger) );
    }


    // -- misc

    #[test]
    fn it_works_a() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works_b() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok( () )
        } else {
            Err( String::from("two plus two should equal four") )
        }
    }

    // -- showing function output

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10( 4 );
        assert_eq!( 10, value );
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10( 8 );
        assert_ne!( 5, value );
    }
}
