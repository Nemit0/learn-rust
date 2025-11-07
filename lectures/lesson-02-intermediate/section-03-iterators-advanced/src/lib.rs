//! Section: Iterators (Custom Iterator + Adapters)
//!
//! Implement a simple Fibonacci iterator and use adapters to collect values.

#[derive(Debug, Clone)]
pub struct Fibonacci {
    curr: u64,
    next: u64,
    remaining: usize,
}

impl Fibonacci {
    /// Create a Fibonacci iterator that yields `n` values starting from 0, 1.
    pub fn new(_n: usize) -> Self {
        // TODO: initialize fields so that the first call to next() returns 0,
        // then 1, 1, 2, 3, ... up to n items.
        unimplemented!("initialize Fibonacci state")
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: yield the current value and update internal state.
        unimplemented!("decrement remaining and advance the sequence")
    }
}

/// Return the first `n` Fibonacci numbers as a Vec using your iterator.
pub fn fib_take(_n: usize) -> Vec<u64> {
    // TODO: use Fibonacci::new(n) and collect
    unimplemented!("construct the iterator and collect")
}

