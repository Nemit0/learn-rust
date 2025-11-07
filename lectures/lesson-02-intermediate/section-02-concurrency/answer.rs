// Reference solution (not compiled by default).

use std::thread;

pub fn parallel_sum(nums: &[i32], n_threads: usize) -> i32 {
    let n_threads = n_threads.max(1);
    if nums.is_empty() {
        return 0;
    }
    let chunk_size = (nums.len() + n_threads - 1) / n_threads; // ceil div
    let mut handles = Vec::new();
    for chunk in nums.chunks(chunk_size) {
        // Move owned Vec copy of the chunk to avoid lifetime headaches; but to keep
        // memory lean, clone into a Vec only if necessary. Alternatively, copy the
        // slice into a Vec and sum inside the thread.
        let v = chunk.to_vec();
        let handle = thread::spawn(move || v.iter().copied().sum::<i32>());
        handles.push(handle);
    }
    handles
        .into_iter()
        .map(|h| h.join().expect("thread panicked"))
        .sum()
}

