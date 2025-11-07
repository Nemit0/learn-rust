//! Section: Ownership & Borrowing
//!
//! Practice references, mutability, and a simple lifetime.

/// Append `suffix` onto `s` in-place.
pub fn append_in_place(s: &mut String, suffix: &str) {
    // Hint: use `push_str`
    unimplemented!("modify the borrowed String")
}

/// Return the first character of `s` if present.
pub fn first_char(s: &str) -> Option<char> {
    // Hint: iterate over `s.chars()`
    unimplemented!("return s.chars().next()")
}

/// Return a reference to the longer of `a` and `b`.
/// This requires an explicit lifetime parameter because the result
/// borrows from one of the inputs.
pub fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    // Hint: compare lengths and return one of the inputs
    unimplemented!("return a or b by reference")
}

