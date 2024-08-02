//! # doc_example
//!
//! `doc_example` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
/// # Example
///
/// ```
/// let arg = 5;
/// let answer = doc_example::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}