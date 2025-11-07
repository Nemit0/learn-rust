# Lesson 02 · Section 01 — Smart Pointers (Box, Rc)

Summary
- Practice building a tiny recursive list using `Box` and share data using `Rc`.

Learning Objectives
- Construct recursive data with `Box<T>` and pattern match on enums.
- Use `Rc<T>` for shared ownership and query the strong reference count.
- Recursively traverse structures and collect data.

Prerequisites
- Lesson 01: enums, pattern matching, basic ownership and borrowing.

Timebox
- 15 minutes.

Guide
- `Box<T>` stores data on the heap and gives you ownership of it; use it to build recursive types like linked lists.
- Typical cons node construction: `List::Cons(value, Box::new(tail))`.
- Pattern‑match recursively to traverse: match on `Cons(h, t)` and call into the tail.
- `Rc<T>` enables shared ownership in single‑threaded code; clone with `Rc::clone(&rc)`.
- Inspect reference counts via `Rc::strong_count(&rc)` for learning/debugging.

Quick Demo
```rust
use std::rc::Rc;

enum List { Cons(i32, Box<List>), Nil }
use List::*;

fn len(lst: &List) -> usize { match lst { Nil => 0, Cons(_, t) => 1 + len(t) } }

fn main() {
    let lst = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("len={}", len(&lst));

    let rc = Rc::new(42);
    let a = Rc::clone(&rc);
    println!("count={}", Rc::strong_count(&rc)); // 2
    drop(a);
    println!("count={}", Rc::strong_count(&rc)); // 1
}
```

Run
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

Starter Code
- Implement `List::cons`, `List::len`, and `List::to_vec`.
- Implement `make_rc`, `clone_rc`, and `strong_count` using `Rc<T>`.
- Run tests with: `cargo test -p lesson02_section01_smart_pointers`.

Tasks
1) Build a `List` using `Box` and implement `len()` and `to_vec()` recursively.
2) Create and clone `Rc<i32>` values and verify the strong count.

Hints
- For `cons`, return `List::Cons(head, Box::new(tail))`.
- `Rc::clone(&rc)` increments the strong count.

Common Pitfalls
- Forgetting to box the tail when constructing a cons cell.
- Returning borrowed data from `to_vec()`; collect into an owned `Vec<i32>` instead.

Validation
- Run: `cargo test -p lesson02_section01_smart_pointers`.
- The tests assert correct list length, order, and `Rc` strong counts.

Extensions (Optional)
- Add a `push_back` or `rev` function for the list.
