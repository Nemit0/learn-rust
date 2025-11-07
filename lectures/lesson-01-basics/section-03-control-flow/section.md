# Lesson 01 · Section 03 — Control Flow

Summary
- Practice branching with `if`/`match` and simple loops.

Learning Objectives
- Implement conditional logic with correct ordering and guards.
- Use `for` loops to accumulate results.

Prerequisites
- Variables, functions.

Timebox
- Target: 10–15 minutes.

Guide
- `if` is an expression: `let label = if n % 2 == 0 { "even" } else { "odd" };`.
- `match` compares against patterns top‑to‑bottom; the first match wins. Order matters when patterns overlap.
- Use guards to refine branches: `match n { x if x % 15 == 0 => ..., _ => ... }`.
- Loops:
  - `for i in 1..=n { ... }` iterates inclusive.
  - `while cond { ... }` repeats until false.
- For FizzBuzz, check 15 before 3/5 to avoid earlier matches stealing the 15 case.

Quick Demo
```rust
fn fizz_buzz(n: i32) -> String {
    if n % 15 == 0 { "FizzBuzz".into() }
    else if n % 3 == 0 { "Fizz".into() }
    else if n % 5 == 0 { "Buzz".into() }
    else { n.to_string() }
}

fn main() {
    for n in 1..=6 { print!("{} ", fizz_buzz(n)); }
}
```

Run
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

Starter Code
- Implement `is_even`, `fizz_buzz`, and `sum_up_to` in `src/lib.rs`.
- Keep implementations simple and readable.

Tasks
1) `is_even(n)` returns `true` if `n` is divisible by 2.
2) `fizz_buzz(n)` follows standard rules; prefer checking 15 first.
3) `sum_up_to(n)` sums 1..=n (0 returns 0).

Hints
- For fizz_buzz, string literals like "Fizz" are fine; use `to_string()` where needed.

Common Pitfalls
- Checking 3 or 5 before 15 produces wrong output for 15.

Validation
- Run: `cargo test -p lesson01_section03_control_flow`
- Tests cover evenness, fizz/buzz, and simple sums.

Extensions (Optional)
- Re-implement `sum_up_to` using `Iterator::sum` for comparison.
