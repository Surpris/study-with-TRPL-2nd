/*
    title: Cargo と Crates.io について
    name : Surpris
    date : 2020/02/28
*/

//! # My crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Add 1 to the number given
///
/// # Example
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(five));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(5, add_one(4));
    }
}
