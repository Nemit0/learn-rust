// Reference solution (not compiled by default).

use std::collections::HashMap;

pub fn square_all(nums: &[i32]) -> Vec<i32> {
    nums.iter().map(|n| n * n).collect()
}

pub fn sum_of_evens(nums: &[i32]) -> i32 {
    nums.iter().copied().filter(|n| n % 2 == 0).sum()
}

pub fn word_counts(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let key = word.to_lowercase();
        *map.entry(key).or_insert(0) += 1;
    }
    map
}

