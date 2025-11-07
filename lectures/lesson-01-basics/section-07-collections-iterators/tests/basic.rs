use lesson01_section07_collections_iterators::*;
use std::collections::HashMap;

#[test]
fn squares_numbers() {
    assert_eq!(square_all(&[1, 2, 3]), vec![1, 4, 9]);
    assert_eq!(square_all(&[]), Vec::<i32>::new());
}

#[test]
fn sums_evens_only() {
    assert_eq!(sum_of_evens(&[1, 2, 3, 4, 5, 6]), 12);
    assert_eq!(sum_of_evens(&[]), 0);
}

#[test]
fn counts_words_case_insensitive() {
    let text = "Rust rust RUST rustacean";
    let counts = word_counts(text);
    assert_eq!(counts.get("rust"), Some(&3));
    assert_eq!(counts.get("rustacean"), Some(&1));
    assert_eq!(counts.get("missing"), None);
}

