# Lesson 02 · Section 03 — Iterators (Custom + Adapters)

[한국어(KO)](section.ko.md)

Summary
- Build a custom `Iterator` and compose it with adapters to collect results.

Learning Objectives
- Implement `Iterator` for a stateful type.
- Use `.collect()`, `.take()`, and other adapters fluently.
- Write tests that validate both structure and content.

Prerequisites
- Basics: enums/structs, methods, iterators and closures.

Timebox
- 10–15 minutes.

Guide
- Implementing `Iterator` requires a `next(&mut self) -> Option<Item>` method that returns `Some(item)` or `None` when finished.
- Keep iterator state minimal and update it atomically each `next()` call.
- Compose with adapters like `.map`, `.take`, `.collect::<Vec<_>>()` to transform and gather results.

Quick Demo
```rust
#[derive(Clone)]
struct Fib { curr: u64, next: u64, left: usize }
impl Fib { fn new(n: usize) -> Self { Self { curr: 0, next: 1, left: n } } }
impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.left == 0 { return None; }
        let out = self.curr;
        let new_next = self.curr + self.next;
        self.curr = self.next; self.next = new_next; self.left -= 1; Some(out)
    }
}
fn main() { println!("{:?}", Fib::new(7).collect::<Vec<_>>()); }
```

Run
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

Starter Code
- Implement `Fibonacci::new(n)` and `Iterator for Fibonacci`.
- Implement `fib_take(n)` using your iterator.
- Run: `cargo test -p lesson02_section03_iterators_advanced`.

Tasks
1) Yield exactly `n` Fibonacci numbers starting from 0, 1.
2) Ensure `.collect::<Vec<_>>()` returns the expected prefix.

Hints
- Keep a `remaining` counter and decrement on each successful `next()`.
- Update `(curr, next)` as `(next, curr + next)`.

Common Pitfalls
- Off-by-one errors in `remaining`.
- Overflow isn’t tested here, but use `u64` to keep numbers large.

Validation
- Run: `cargo test -p lesson02_section03_iterators_advanced`.
- Tests assert length and known prefixes.

Extensions (Optional)
- Add a method to skip to a specific index (e.g., via `nth`).
