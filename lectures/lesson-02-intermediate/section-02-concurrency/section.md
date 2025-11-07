# Lesson 02 · Section 02 — Concurrency (Threads and Join)

[한국어(KO)](section.ko.md)

Summary
- Practice spawning threads, joining them, and aggregating results safely.

Learning Objectives
- Split work into chunks and spawn worker threads.
- Use `JoinHandle` to collect results deterministically.
- Handle edge cases like `n_threads == 0` gracefully.

Prerequisites
- Basics: ownership/borrowing, iterators; no prior concurrency required.

Timebox
- 15–20 minutes.

Guide
- Spawn a worker with `thread::spawn(move || { /* compute */ })`; `move` transfers captured values into the thread closure.
- Always join spawned threads with `handle.join().expect("thread panicked")` to retrieve results.
- Partition work deterministically using `chunks(chunk_size)`; compute `chunk_size` with ceiling division to cover all items.
- Keep inputs borrowed; pass owned copies into threads if needed (e.g., `chunk.to_vec()`) to avoid lifetime issues.

Quick Demo
```rust
use std::thread;

fn main() {
    let data: Vec<i32> = (1..=10_000).collect();
    let mid = data.len()/2;
    let left = data[..mid].to_vec();
    let right = data[mid..].to_vec();
    let h1 = thread::spawn(move || left.iter().copied().sum::<i32>());
    let h2 = thread::spawn(move || right.iter().copied().sum::<i32>());
    let total = h1.join().unwrap() + h2.join().unwrap();
    println!("sum={}", total);
}
```

Run
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

Starter Code
- Implement `parallel_sum(nums, n_threads)` using `thread::spawn` and joins.
- Keep each thread summing a contiguous chunk; the final sum must match sequential.
- Run tests: `cargo test -p lesson02_section02_concurrency`.

Tasks
1) Split input into at most `n_threads` slices; spawn threads to sum them.
2) Join all threads and add their partial sums into the final result.
3) Treat `n_threads == 0` as `1`.

Hints
- `chunks` or manual slice math help partition the input.
- `move` closures capture slices by value; it’s fine since threads end before join returns.

Common Pitfalls
- Forgetting to join threads (leaks work) or moving `nums` instead of borrowing.
- Spawning more threads than necessary.

Validation
- Run: `cargo test -p lesson02_section02_concurrency`.
- The tests compare against a deterministic sequential sum across settings.

Extensions (Optional)
- Add parallel map-reduce or parameterize the operation.
