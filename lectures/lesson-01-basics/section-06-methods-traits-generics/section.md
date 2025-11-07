# Lesson 01 · Section 06 — Methods, Traits, Generics

[한국어(KO)](section.ko.md)

Summary
- Implement methods, define/implement a simple trait, and write a generic function.

Learning Objectives
- Use `impl` blocks for methods and borrowing with `&self`.
- Implement and use a trait with a String-returning method.
- Write a generic function constrained by `Ord + Copy`.

Prerequisites
- Structs/enums basics, pattern matching.

Timebox
- Target: 15–20 minutes.

Guide
- Methods live in `impl Type { ... }` blocks and take `&self` or `&mut self` to borrow the receiver.
- Example area method: `fn area(&self) -> u32 { self.width * self.height }`.
- Traits define shared behavior; implement them with `impl Trait for Type { ... }` and return owned `String` where needed.
- Generics add type parameters with trait bounds to restrict capabilities:
  - `fn max_of<T: Ord + Copy>(a: T, b: T) -> T { if a >= b { a } else { b } }`.

Quick Demo
```rust
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }
impl Rectangle {
    fn area(&self) -> u32 { self.width * self.height }
    fn can_hold(&self, other: &Rectangle) -> bool { self.width > other.width && self.height > other.height }
}

trait Summarize { fn summary(&self) -> String; }
struct Person { name: String }
impl Summarize for Person { fn summary(&self) -> String { format!("{} says hi", self.name) } }

fn max_of<T: Ord + Copy>(a: T, b: T) -> T { if a >= b { a } else { b } }

fn main() {
    let r = Rectangle { width: 3, height: 2 };
    println!("{} {}", r.area(), r.can_hold(&Rectangle { width: 2, height: 1 }));
    let p = Person { name: "Ada".into() };
    println!("{}", p.summary());
    println!("{}", max_of(3, 5));
}
```

Run
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

Starter Code
- Implement `Rectangle::area`, `Rectangle::can_hold`, `Person::summary`, and `max_of` in `src/lib.rs`.

Validation
- Run: `cargo test -p lesson01_section06_methods_traits_generics`

Extensions (Optional)
- Give `Summarize` a default method and override it for `Person`.
