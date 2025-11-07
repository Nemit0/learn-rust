// Reference solution (not compiled by default).

pub mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
    pub fn mul(a: i32, b: i32) -> i32 { a * b }
}

pub mod string_utils {
    pub fn shout(s: &str) -> String { format!("{}!", s.to_uppercase()) }
}

pub use math::{add, mul};
pub use string_utils::shout;

