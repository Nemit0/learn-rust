//! Section: Methods, Traits, Generics
//!
//! Implement methods on a struct, a simple trait, and a generic function.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// Return the area (width * height).
    pub fn area(&self) -> u32 {
        unimplemented!("compute width * height")
    }

    /// Return true if `self` can contain `other` strictly (both dims larger).
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        unimplemented!("compare both dimensions strictly")
    }
}

pub trait Summarize {
    fn summary(&self) -> String;
}

#[derive(Debug)]
pub struct Person {
    pub name: String,
}

impl Summarize for Person {
    fn summary(&self) -> String {
        unimplemented!("return a string summary that includes the name")
    }
}

/// Return the maximum of `a` and `b` using `Ord`.
pub fn max_of<T: Ord + Copy>(a: T, b: T) -> T {
    unimplemented!("return a if a >= b else b")
}
