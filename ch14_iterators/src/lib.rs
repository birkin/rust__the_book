/* ---------------------------------------------
   START `Creating Our Own Iterators...` example */


 #[derive(PartialEq, Debug)]
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

    fn next( &mut self ) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some( self.count )
        } else {
            None
        }
    }
}


#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!( counter.next(), Some(1) );
    assert_eq!( counter.next(), Some(2) );
    assert_eq!( counter.next(), Some(3) );
    assert_eq!( counter.next(), Some(4) );
    assert_eq!( counter.next(), Some(5) );
    assert_eq!( counter.next(), None );

}

// fn main() {
//     let cntr = Counter::new();
//     println!("cntr, ``{:?}``", cntr );
// }
/* END `Creating Our Own Iterators...` example
   --------------------------------------------- */




/* ---------------------------------------------
   START shoe filter iterator example */


 #[derive(PartialEq, Debug)]
 struct Shoe {
    size: u32,
    style: String,
 }


 fn shoes_in_my_size( shoes: Vec<Shoe>, shoe_size: u32 ) -> Vec<Shoe> {
    shoes.into_iter().filter( |s| s.size == shoe_size ).collect()
 }


#[cfg(test)]
mod tests {
    use super::*;  // Shoe not found in scope without this.

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size( shoes, 10 );

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                }
            ]
        );

    }  // end fn filters_by_size()

}

// fn main() {}  // don't know why book includeds this; test runs fine without it.

/* END shoe filter iterator example
   --------------------------------------------- */


#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!( v1_iter.next(), Some(&1) );
    assert_eq!( v1_iter.next(), Some(&2) );
    assert_eq!( v1_iter.next(), Some(&3) );
    assert_eq!( v1_iter.next(), None );
}


#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!( total, 6 );
}


#[test]
fn iterator_start_then_sum() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();
    let foo = v1_iter.next();
    println!( "foo, ``{:?}``", foo );  // yields: foo, ``Some(1)``

    let total: i32 = v1_iter.sum();

    assert_eq!( total, 5 );
}

