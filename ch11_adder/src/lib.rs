// -- Guess

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


// -- tests

#[cfg(test)]
mod tests {
    use super::*;

    // -- Guess {}

    #[test]
    #[should_panic( expected = "Guess value must be less than or equal to 100" )]
    fn greater_then_100() {
        Guess::new( 200 );
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
    fn should_add_two() {
        assert_eq!(4, add_two(2));
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
        assert!( larger.can_hold(&smaller) );
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
    fn it_works_A() {
        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn it_works_B() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok( () )
        } else {
            Err( String::from("two plus two should equal four") )
        }
    }
}
