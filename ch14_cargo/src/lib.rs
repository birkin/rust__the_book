//! # My Crate

//! `ch14_cargo` is a collection of utilities to make performaing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = ch14_cargo::add_one( arg );
///
/// assert_eq!( 6, answer );
/// ```
pub fn add_one( x: i32 ) -> i32 {
    x + 1
}
