//! Section: Modules, Crates, Workspaces
//!
//! Organize code into modules and re-export selected APIs from the crate root.

pub mod math {
    /// Add two integers.
    pub fn add(a: i32, b: i32) -> i32 {
        unimplemented!("return a + b")
    }

    /// Multiply two integers.
    pub fn mul(a: i32, b: i32) -> i32 {
        unimplemented!("return a * b")
    }
}

pub mod string_utils {
    /// Uppercase the input and add an exclamation mark.
    pub fn shout(s: &str) -> String {
        unimplemented!("uppercase and add !")
    }
}

// Re-export a curated public surface from the crate root.
pub use math::{add, mul};
pub use string_utils::shout;

