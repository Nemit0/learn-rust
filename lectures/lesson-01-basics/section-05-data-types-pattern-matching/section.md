# Lesson 01 · Section 05 — Data Types & Pattern Matching

[한국어(KO)](section.ko.md)

Summary
- Create simple enums and tuple structs; branch with `match` and guards.

Learning Objectives
- Map integer inputs to enum variants safely.
- Use match guards and struct patterns.
- Model fallible operations with `Option`.

Prerequisites
- Ownership & borrowing basics.

Timebox
- Target: 15–20 minutes.

Guide
- Enums group related variants under one type. Example:
  - `enum Weekday { Mon, Tue, Wed, Thu, Fri, Sat, Sun }`
- `Option<T>` models presence/absence: `Some(T)` or `None`.
- Tuple structs are unnamed fields with a struct type: `struct Point(i32, i32);`
- `match` allows destructuring and guards:
  - `match p { Point(x, y) if x == 0 || y == 0 => "axis", ... }`
- Returning `Option` from `safe_div(a, b)` communicates failure without panics: return `None` when `b == 0`, otherwise `Some(a / b)`.

Quick Demo
```rust
#[derive(Debug)]
enum Weekday { Mon, Tue, Wed, Thu, Fri, Sat, Sun }

fn weekday_from(n: u8) -> Option<Weekday> {
    match n {
        1 => Some(Weekday::Mon), 2 => Some(Weekday::Tue), 3 => Some(Weekday::Wed),
        4 => Some(Weekday::Thu), 5 => Some(Weekday::Fri), 6 => Some(Weekday::Sat),
        7 => Some(Weekday::Sun), _ => None
    }
}

#[derive(Debug)]
struct Point(i32, i32);

fn quadrant(p: Point) -> &'static str {
    let Point(x, y) = p;
    if x == 0 || y == 0 { "axis" }
    else if x > 0 && y > 0 { "Q1" }
    else if x < 0 && y > 0 { "Q2" }
    else if x < 0 && y < 0 { "Q3" } else { "Q4" }
}

fn safe_div(a: i32, b: i32) -> Option<i32> { if b == 0 { None } else { Some(a / b) } }

fn main() {
    println!("{:?}", weekday_from(5));
    println!("{}", quadrant(Point(-2, 3)));
    println!("{:?}", safe_div(10, 2));
}
```

Run
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

Starter Code
- Implement `weekday_from`, `quadrant`, and `safe_div` in `src/lib.rs`.

Tasks
1) Convert 1..=7 to `Weekday`; others => `None`.
2) Classify a `Point` into quadrants or axis using `match` with guards.
3) Implement `safe_div(a, b)` that returns `None` on divide-by-zero.

Validation
- Run: `cargo test -p lesson01_section05_data_types_pattern_matching`

Extensions (Optional)
- Add a `display` function for `Weekday` using `match`.
