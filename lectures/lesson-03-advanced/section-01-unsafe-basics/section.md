# Lesson 03 · Section 01 — Unsafe Basics (Raw Pointers)

[한국어(KO)](section.ko.md)

Summary
- Use `unsafe` in a controlled way to read memory via raw pointers and to build slices manually.

Learning Objectives
- Explain safety contracts and limit `unsafe` to small blocks.
- Convert `*const T` / `*mut T` into slices with `from_raw_parts(_mut)`.
- Recreate `split_at_mut` with raw pointers while avoiding aliasing.

Prerequisites
- Ownership & borrowing; slices; iterator basics.

Timebox
- 15–20 minutes.

Guide
- `unsafe` lets you do operations the compiler cannot check: deref raw pointers, call `extern` functions, access mutable statics, union fields.
- Keep `unsafe` blocks tiny; document preconditions (valid pointer, length bounds, no aliasing).
- Building a slice from raw parts:
  - `let xs = unsafe { std::slice::from_raw_parts(ptr, len) };`
  - For mutable: `from_raw_parts_mut(ptr, len)`. Pointers must be valid and not alias.

Quick Demo
```rust
use std::slice;

unsafe fn sum_via_ptr(ptr: *const i32, len: usize) -> i32 {
    let xs = slice::from_raw_parts(ptr, len);
    xs.iter().copied().sum()
}

fn main() {
    let v = vec![1,2,3,4];
    let total = unsafe { sum_via_ptr(v.as_ptr(), v.len()) };
    println!("sum={}", total);
}
```

Run
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

Starter Code
- Implement `sum_via_ptr`, `sum_slice_unsafe`, and `split_at_mut_via_ptr` in `src/lib.rs`.

Validation
- Run: `cargo test -p lesson03_section01_unsafe_basics`

Common Pitfalls
- Forgetting to assert `mid <= xs.len()` when splitting.
- Creating overlapping mutable slices — always use non-overlapping ranges.
