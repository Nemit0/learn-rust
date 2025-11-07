# Lesson 01 · Section 07 — Collections & Iterators

Summary
- Use vectors, slices, iterator adapters, and `HashMap`.

Learning Objectives
- Transform and collect results with iterator chains.
- Filter and sum values from a slice.
- Use `HashMap::entry` to count occurrences.

Prerequisites
- Methods, traits, generics basics.

Timebox
- Target: 15–20 minutes.

Guide
- Slices `&[T]` are views into contiguous memory; iterate with `.iter()`.
- Build vectors from iterators using adapters and `collect::<Vec<_>>()`.
- Examples:
  - Map and collect: `nums.iter().map(|n| n * n).collect::<Vec<_>>()`.
  - Filter and sum: `nums.iter().copied().filter(|n| n % 2 == 0).sum()`.
- `HashMap<K, V>` counting pattern using the entry API:
  - `*map.entry(key).or_insert(0) += 1;`
- For case‑insensitive word counts: `to_lowercase()` then `split_whitespace()`.

Quick Demo
```rust
use std::collections::HashMap;

fn square_all(nums: &[i32]) -> Vec<i32> { nums.iter().map(|n| n * n).collect() }
fn sum_of_evens(nums: &[i32]) -> i32 { nums.iter().copied().filter(|n| n % 2 == 0).sum() }
fn word_counts(text: &str) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for w in text.to_lowercase().split_whitespace() { *m.entry(w.to_string()).or_insert(0) += 1; }
    m
}

fn main() {
    let v = [1, 2, 3, 4];
    println!("{:?}", square_all(&v));
    println!("{}", sum_of_evens(&v));
    println!("{:?}", word_counts("a A b a"));
}
```

Run
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

Starter Code
- Implement `square_all`, `sum_of_evens`, and `word_counts` in `src/lib.rs`.

Validation
- Run: `cargo test -p lesson01_section07_collections_iterators`

Extensions (Optional)
- Ignore punctuation by filtering non-alphanumeric characters.
