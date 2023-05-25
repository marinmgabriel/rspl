//! # My Crate
//!
//! `my_crate` is a collectoin  of utilities to make
//! performing certain calculation more convenient.
//! It's also very easy to use.

/// Adds one to the number given.
/// # Example
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
