# Lesson 01 · Section 04 — Ownership & Borrowing

[한국어(KO)](section.ko.md)

Summary
- Borrow data immutably and mutably; return references with lifetimes.

Learning Objectives
- Modify data via `&mut` without taking ownership.
- Read data via `&str` and return options.
- Use an explicit lifetime when returning a borrowed value from inputs.

Prerequisites
- Variables, functions, basic control flow.

Timebox
- Target: 15–20 minutes.

Guide
- Ownership: values like `String` own their heap data; moving transfers ownership. Borrow with references to access without taking ownership.
- Immutable borrow: `&T` lets you read; Mutable borrow: `&mut T` lets you modify but is exclusive while active.
 - `String` vs `&str`:
  - `String` is an owned, growable buffer. `push_str` appends in place.
  - `&str` is an immutable view (borrow) into UTF‑8 data.
- Lifetimes: returning a reference tied to inputs often needs an explicit lifetime when the compiler cannot infer a single source. Example signature:
  - `fn longer<'a>(a: &'a str, b: &'a str) -> &'a str` — the result lives as long as both inputs; you return one of them.
- Iterating characters: `s.chars().next()` returns `Option<char>`; `None` when the string is empty.

Option Refresher (Some/None)
- `Option<T>` is used when a value may or may not exist.
- `Some(value)` wraps an existing value; `None` means “no value”.
- `s.chars().next()` already returns `Option<char>`, so you can just `return s.chars().next()`.
- If you prefer an explicit match: `match s.chars().next() { Some(c) => Some(c), None => None }`.

Useful Built‑ins
- `String`: `push_str`, `push`, `len`, `is_empty`, `clear`.
- `&str`: `len`, `is_empty`, `starts_with`, `ends_with`, `contains`, `split_whitespace`.

Quick Demo
```rust
fn append_in_place(s: &mut String, suffix: &str) { s.push_str(suffix); }
fn first_char(s: &str) -> Option<char> { s.chars().next() }
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str { if a.len() >= b.len() { a } else { b } }

fn main() {
    let mut s = String::from("hi");
    append_in_place(&mut s, "!");
    println!("{} {:?}", s, first_char(&s));
    println!("{}", longer("abc", "rust"));
}
```

Run
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

Starter Code
- Implement `append_in_place`, `first_char`, and `longer` in `src/lib.rs`.

Tasks
1) `append_in_place(s, suffix)` appends `suffix` to `s`.
2) `first_char(s)` returns the first `char` or `None`.
3) `longer(a, b)` returns the longer `&str` (by reference).

Hints
- `String::push_str` appends onto a `String`.
- `s.chars().next()` yields an `Option<char>`.

Common Pitfalls
- Attempting to return a new owned `String` from `longer` rather than a borrow.
- Forgetting to annotate a lifetime for the `longer` return type.

Validation
- Run: `cargo test -p lesson01_section04_ownership_borrowing`

Extensions (Optional)
- Add `shorter(a, b)` with the inverse logic.
