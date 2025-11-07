# Lesson 01 · Section 01 — Variables and Mutability

Goal
- Practice declaring variables, mutability, and using expressions.

Guide
- `let` binds an immutable variable; add `mut` for mutability: `let mut x = 5; x = 10;`.
- Shadowing creates a new binding with the same name: `let x = x + 1;` (type can change), whereas `mut` keeps the same binding/type.
- Expressions return values; statements do not. The last expression in a block (without a trailing semicolon) becomes the return value.
- Common integer types: `i32` (default for integers), `u32`, `i64`, etc.
- Return from a function by:
  - implicit tail expression: `x * 2`
  - or explicitly: `return x * 2;`

Quick Demo
```rust
fn main() {
    let mut x = 5;   // immutable by default; add `mut` to change
    x = 10;
    let y = x * 2;   // expressions return values
    println!("{x} {y}"); // 10 20
}
```

Run
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

Built‑ins Cheat Sheet (Basics)
- Integers: `i32::MIN`, `i32::MAX`, `i32::abs(-3)`, `5.saturating_add(10)`, `5.wrapping_mul(3)`.
- Booleans: `bool` supports `&&`, `||`, `!` (logical ops).
- Strings:
  - `String::from("hi")`, `s.len()`, `s.is_empty()`, `s.push_str("!")`, `s.to_uppercase()`.
  - `&str` methods: `"hi".len()`, `"hi rust".split_whitespace()`.
- Vectors: `Vec::<i32>::new()`, `vec![1,2,3]`, `v.len()`, `v.push(4)`, `v.iter()`.

Tasks
1) `make_ten()` should return `10`.
   - Try declaring a variable, make it mutable with `mut`, and reassign it.
   - Any correct approach that returns `10` will pass.

2) `double(x)` should return `2 * x`.

How to Run
- Run tests for this section: `cargo test -p lesson01_section01_variables`
- Or grade all sections: `./scripts/grade.ps1` (Windows) or `bash ./scripts/grade.sh` (macOS/Linux)

Reference
- See `answer.rs` for one possible solution (not compiled by default).
