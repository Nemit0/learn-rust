//! Section: Error Handling
//!
//! Use `Result`, the `?` operator, and define simple error types.

use std::num::ParseIntError;

/// Parse two integers and return their sum.
pub fn parse_and_add(a: &str, b: &str) -> Result<i32, ParseIntError> {
    // Hint: use `a.parse::<i32>()?` etc.
    unimplemented!("parse both strings and add the values")
}

/// Return `Ok(n as u32)` if `n >= 0`, else an error message.
pub fn to_positive(n: i32) -> Result<u32, String> {
    // Hint: map negative to Err with a message
    unimplemented!("convert or return Err")
}

/// Try to get the first element of the slice as `i32`.
/// Return an error message if the slice is empty.
pub fn first_or_err(xs: &[i32]) -> Result<i32, String> {
    // Hint: use `xs.first().copied()`
    unimplemented!("wrap Option in Result with a message")
}

