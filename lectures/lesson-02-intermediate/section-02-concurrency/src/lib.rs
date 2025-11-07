//! Section: Concurrency (Threads and Join)
//!
//! Implement a simple parallel sum by splitting work across threads.

use std::thread;

/// Compute the sum of `nums` using `n_threads` worker threads.
///
/// Requirements:
/// - Split `nums` into roughly equal chunks (contiguous ranges).
/// - Spawn up to `n_threads` threads, each summing its chunk.
/// - Join all threads and return the total.
/// - Edge cases: if `n_threads` is 0, behave as if it's 1.
pub fn parallel_sum(_nums: &[i32], _n_threads: usize) -> i32 {
    // TODO: implement chunked summation with thread::spawn
    unimplemented!("spawn threads, sum chunks, join, add results")
}

