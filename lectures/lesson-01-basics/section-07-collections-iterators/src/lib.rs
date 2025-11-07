//! Section: Collections & Iterators
//!
//! Practice using `Vec`, slices, and iterator adapters.

/// Return a new Vec with each input squared.
pub fn square_all(nums: &[i32]) -> Vec<i32> {
    // Hint: use `iter().map(|n| n * n).collect()`
    unimplemented!("map over nums and collect")
}

/// Sum only the even numbers from the slice.
pub fn sum_of_evens(nums: &[i32]) -> i32 {
    // Hint: filter then sum
    unimplemented!("filter even numbers and sum")
}

use std::collections::HashMap;

/// Count case-insensitive word frequencies (split on ASCII whitespace).
pub fn word_counts(text: &str) -> HashMap<String, usize> {
    // Hint: to_lowercase + split_whitespace + entry API
    unimplemented!("accumulate counts into a HashMap")
}

