# Lesson XX · Section YY — Title

Summary
- 1–3 sentences describing the problem and why it matters.

Learning Objectives
- Objective 1 (behavior-oriented)
- Objective 2 (API/idiom-oriented)
- Objective 3 (ownership/borrowing/error-handling as applicable)

Prerequisites
- Prior section(s) or concepts needed.

Timebox
- Target: 10–20 minutes.

Reading
- Link to Rust Book chapter(s) and brief notes.

Quick Demo
- A tiny, self-contained Rust program (<= ~15 lines) that illustrates the core idea of the section.
- Must compile with `rustc` (no dependencies) and run on Windows/macOS/Linux.
- Run locally:
  - Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
  - macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

Starter Code
- Describe the incomplete functions and constraints (e.g., “no loops,” “use `match`”).
- Mention any provided helpers and how to run tests.

Tasks
1) Task one with acceptance criteria.
2) Task two with acceptance criteria.

Hints
- Short nudge(s) for common sticking points.

Common Pitfalls
- Typical mistakes, especially around borrowing, lifetimes, or mutability.

Validation
- Run: `cargo test -p lessonXX_sectionYY_title`
- Expected behavior: list main assertions the tests enforce.

Extensions (Optional)
- One or two small follow-ups for deeper exploration.

Author Notes (Remove in student version)
- Keep `src/lib.rs` compiling using `unimplemented!()` where necessary.
- Use clear, deterministic tests; avoid I/O and network unless the section is about them.
- If needed, add `#[allow(unused)]` to keep the starter compiling quietly.
