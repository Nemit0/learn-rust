// Reference solution (not compiled by default).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 { self.width * self.height }
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub trait Summarize {
    fn summary(&self) -> String;
}

#[derive(Debug)]
pub struct Person { pub name: String }

impl Summarize for Person {
    fn summary(&self) -> String { format!("Person: {}", self.name) }
}

pub fn max_of<T: Ord + Copy>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

