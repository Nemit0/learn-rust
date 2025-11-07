# Lesson 01 · Section 09 — Modules, Crates, Workspaces

[한국어(KO)](section.ko.md)

Summary
- Organize code in modules and re-export a clean public API.

Learning Objectives
- Declare modules with `pub mod` and `pub` functions.
- Re-export selected items with `pub use`.

Prerequisites
- Error handling; collections and iterators.

Timebox
- Target: 10–15 minutes.

Guide
- Modules organize code: `pub mod math { pub fn add(...) { ... } }`.
- Visibility is private by default; use `pub` to expose functions, types, and modules.
- Re‑export APIs from the crate root to shape a friendly public surface: `pub use math::add;`.
- Paths: refer to items with `crate::`, `super::`, or by module paths.
- Keep the crate root minimal by re‑exporting only what students need in tests.

Quick Demo
```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
    pub fn mul(a: i32, b: i32) -> i32 { a * b }
}

mod string_utils {
    pub fn shout(s: &str) -> String { format!("{}!", s.to_uppercase()) }
}

pub use math::{add, mul};
pub use string_utils::shout;

fn main() {
    println!("{} {}", add(2, 3), mul(4, 5));
    println!("{}", shout("rust"));
}
```

Run
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

Starter Code
- Implement `math::add`, `math::mul`, and `string_utils::shout` in `src/lib.rs`.
- The crate root re-exports `add`, `mul`, and `shout` for external use.

Validation
- Run: `cargo test -p lesson01_section09_modules_workspaces`

Extensions (Optional)
- Add a private helper function and call it from a public function.
