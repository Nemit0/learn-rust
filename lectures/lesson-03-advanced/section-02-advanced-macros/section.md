# Lesson 03 · Section 02 — Advanced Declarative Macros

Summary
- Write data‑producing macros with repetition and trailing comma support.

Learning Objectives
- Expand to owned data structures using input expressions.
- Use macro repetition `$( ... ),*` and optional trailing commas `$(,)?`.
- Keep macros hygienic and minimal.

Prerequisites
- Basic macro_rules syntax; collections & strings.

Timebox
- 15 minutes.

Guide
- Turn comma‑separated inputs into a `Vec<String>` by mapping each expr with `.to_string()`.
- Build a `HashMap<String, i32>` from `key => value` pairs by allocating a `mut` map and inserting pairs.
- Support an optional trailing comma in inputs to match idiomatic Rust macros.

Quick Demo
```rust
macro_rules! vec_of_strings { ($($x:expr),* $(,)?) => { vec![$($x.to_string()),*] }; }
macro_rules! make_map { ($($k:expr => $v:expr),* $(,)?) => {{
    let mut m = std::collections::HashMap::new();
    $( m.insert($k.to_string(), $v); )*
    m
}} }

fn main() {
    let v = vec_of_strings!("a", 1, String::from("x"));
    let m = make_map!("a" => 1, "b" => 2,);
    println!("{:?} {:?}", v, m.get("a"));
}
```

Run
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

Starter Code
- Implement `vec_of_strings!` and `make_map!` in `src/lib.rs`.

Validation
- Run: `cargo test -p lesson03_section02_advanced_macros`

Common Pitfalls
- Forgetting `$(,)?` to accept a trailing comma.
- Using `$x:ident` instead of `$x:expr` and rejecting complex expressions.

