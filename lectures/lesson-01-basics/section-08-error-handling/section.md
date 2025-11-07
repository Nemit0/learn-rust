# Lesson 01 · Section 08 — Error Handling

Summary
- Return and propagate errors using `Result` and the `?` operator.

Learning Objectives
- Convert from `Option`/`parse` errors into `Result`.
- Use `?` to propagate errors.
- Return custom error messages where appropriate.

Prerequisites
- Collections & iterators basics.

Timebox
- Target: 10–15 minutes.

Guide
- `Result<T, E>` represents success (`Ok(T)`) or failure (`Err(E)`).
- The `?` operator returns early on `Err`, propagating it to the caller.
- Parsing strings: `let n: i32 = a.parse()?;` returns `Result<i32, ParseIntError>`.
- Converting `Option` to `Result`: `xs.first().copied().ok_or("empty slice".to_string())`.
- To return an error message for negatives, use a `String`: `Err(format!("{} is negative", n))`.

Quick Demo
```rust
use std::num::ParseIntError;

fn parse_and_add(a: &str, b: &str) -> Result<i32, ParseIntError> {
    Ok(a.parse::<i32>()? + b.parse::<i32>()?)
}

fn to_positive(n: i32) -> Result<u32, String> { if n >= 0 { Ok(n as u32) } else { Err(format!("{} is negative", n)) } }
fn first_or_err(xs: &[i32]) -> Result<i32, String> { xs.first().copied().ok_or("empty slice".to_string()) }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", parse_and_add("2", "40")?);
    println!("{:?}", to_positive(-3));
    println!("{}", first_or_err(&[7, 8])?);
    Ok(())
}
```

Run
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

Starter Code
- Implement `parse_and_add`, `to_positive`, and `first_or_err` in `src/lib.rs`.

Validation
- Run: `cargo test -p lesson01_section08_error_handling`

Extensions (Optional)
- Create a small error enum for structured errors (e.g., `Empty` vs `Negative`).
