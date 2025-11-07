# Lesson 01 · Section 02 — Functions

[한국어(KO)](section.ko.md)

Goal
- Practice writing functions and returning values.

Guide
- Function syntax: `fn name(param: Type, ...) -> ReturnType { ... }`.
- The last expression in the function body (no semicolon) is returned implicitly. Use `return` early if needed.
- String building options:
  - `format!("Hello, {}!", name)` creates a `String`.
  - `to_string()` converts `&str` to `String`.
- Prefer borrowing (`&str`) for inputs you only read; return owned `String` when you create new data.

Quick Demo
```rust
fn add(a: i32, b: i32) -> i32 { a + b }
fn greet(name: &str) -> String { format!("Hello, {}!", name) }

fn main() {
    println!("{}", add(2, 3));
    println!("{}", greet("Rust"));
}
```

Run
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

Built‑ins Cheat Sheet (Strings)
- Create: `String::from("hi")`, `format!("{}-{}", a, b)` → `String`.
- Convert: `"hi".to_string()`, `some_string.as_str()`.
- Inspect: `s.len()`, `s.is_empty()`.

Tasks
1) Implement `add(a, b)` so it returns the sum of the two numbers.
2) Implement `greet(name)` to return `"Hello, {name}!"`.

How to Run
- Run tests for this section: `cargo test -p lesson01_section02_functions`
- Or grade all sections: `./scripts/grade.ps1` (Windows) or `bash ./scripts/grade.sh` (macOS/Linux)

Reference
- See `answer.rs` for one possible solution (not compiled by default).
