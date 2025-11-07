# Rust Course Lesson Plan

Audience and Goals
- Audience: Beginners with basic programming experience; pathways to intermediate/advanced.
- Goals: Confidently write, test, and reason about Rust programs; understand ownership/borrowing, error handling, traits/generics, and essential tooling.
- Format: Small, focused sections that compile and run tests quickly; each section has a short brief and code to complete.

Tiered Structure (Overview)
- Basics (Core): Fundamentals that every learner completes. Targets day‑to‑day Rust with ownership, types, error handling, and modules.
- Intermediate (Practice): Broader tooling and patterns used in real projects: pointers, concurrency, I/O, and a macro survey.
- Advanced (Optional): Deeper or specialized topics like unsafe/FFI and advanced macro patterns.
- Capstone (Optional): Integrates multiple lessons into a small, realistic project with strong tests and docs.

Progression by Tier
- Basics
  - 0. Setup & Tooling
    - Install toolchain with `rustup`; `cargo new`, `run`, `test`, `doc`.
    - Rust editions, `rustfmt`, `clippy`, doc comments.
    - Exercise: Create a simple CLI that prints a message.
  - 1. Rust Basics
    - Variables, mutability, shadowing; scalar types; expressions vs statements.
    - Functions, parameters, return values; control flow (`if`, `match`, loops`).
    - Exercises: variables, functions, branching and loops.
  - 2. Ownership & Borrowing
    - Stack/heap model; moves vs copies; references; borrow rules.
    - Slices and string slices; lifetimes (intro-level, elision rules).
    - Exercises: pass-by-reference, fix borrow checker issues, slice APIs.
  - 3. Data Types & Pattern Matching
    - Structs (tuple/unit/regular), enums; `Option`, `Result`.
    - Exercises: implement data types; match expressions; option/result use.
  - 4. Methods, Traits, Generics
    - `impl` blocks; trait definitions/impls; trait bounds; common derives.
    - Exercises: implement a trait; generic functions; trait bounds and `where`.
  - 5. Collections & Iterators
    - `Vec`, `String`, `HashMap`; ownership in collections; slicing.
    - Iterators, adapters, consumers; closures; `IntoIterator`/`FromIterator`.
    - Exercises: iterator pipelines; map/filter; ownership-safe collection code.
  - 6. Error Handling
    - `Result`, the `?` operator; defining error types.
    - Optional: `thiserror`/`anyhow` for ergonomics.
    - Exercises: propagate errors; custom error enums; fallible parsing.
  - 7. Modules, Crates, Workspaces
    - Module system, visibility, paths, `use`; crates; features (intro).
    - Workspaces for multi-crate projects; testing in workspaces.
    - Exercises: refactor into modules; workspace tests; feature-gated code (optional).

- Intermediate
  - 8. Smart Pointers & Interior Mutability
    - `Box`, `Rc`, `Arc`; `Cell`, `RefCell`; `Drop`; ownership patterns.
    - Exercises: shared ownership; interior mutability with borrow checks.
  - 9. Concurrency
    - Threads, `Send`/`Sync`; channels; simple parallelism.
    - Exercises: threaded counter; channel-based worker.
  - 10. Async (Optional in core track)
    - Async/await basics; executors; tasks; I/O at a high level.
    - Exercises: sketch an async pipeline; optional HTTP example.
  - 11. I/O & Serialization
    - Filesystem APIs; reading/writing files; buffering; serde for JSON.
    - Exercises: parse JSON, compute aggregates, write results.
  - 12. Macros & Attributes (Survey)
    - Derive macros; common attributes; intro to declarative macros.
    - Exercises: write a small `macro_rules!` or use custom derives.

- Advanced (Optional)
  - 13. Unsafe & FFI
    - When/why unsafe; invariants; minimal FFI to C.
    - Exercises: safe wrapper sketch; small FFI call (optional).
  - 14. Advanced Macros (Optional)
    - Declarative macro patterns; proc-macro overview; derive internals at a high level.
    - Exercises: small macro refinements; reasoning about hygiene.

- Capstone (Optional)
  - Build a small but real tool/service using multiple lessons; emphasize testing, docs, and error handling.

Authoring Guidelines (Per Section)
- Naming
  - Folder: `lectures/lesson-XX-<topic>/section-YY-<topic>/`.
  - Crate name: `lessonXX_sectionYY_<topic>` (lowercase, underscores).
  - Update root `Cargo.toml` workspace members when adding new sections.
- Files
  - `section.md`: concise brief with goals, constraints, hints, examples, and how to run tests.
  - `src/lib.rs`: starter code with minimal TODOs; keep compiling using `unimplemented!()` where helpful.
  - `tests/*.rs`: definitive grading logic; deterministic, fast, focused on the learning goals.
  - `answer.rs`: reference solution; not compiled.
- Learning Objectives
  - 2–4 clear objectives per section; 10–20 min target duration.
  - Explicit prerequisites and key terms.
- Starter Code
  - Compile-clean or narrowly failing via `unimplemented!()`; avoid noisy warnings (use `#[allow(unused)]` if necessary).
  - Keep function signatures stable; tests target behavior, not internal structure.
- Tests
  - Deterministic, <1s runtime per section; no network; no randomness (or seed it).
  - Cover happy path and 1–2 common mistakes; prefer simple asserts over heavy frameworks.
  - Don’t rely on OS-specific behavior; the Windows grader runs `cargo test -p <pkg>`.
- Hints & Feedback
  - Provide brief hints in `section.md`; point to relevant Rust Book chapters.
  - Consider a “common errors/pitfalls” block with borrow-checker tips.

Quality Bar & Review Checklist
- Does `cargo test -p <pkg>` pass quickly on a fresh checkout?
- Are objectives aligned with tests? Are tests readable and maintainable?
- Is the difficulty appropriate and incremental from prior sections?
- Does `section.md` provide just enough guidance and clear success criteria?
- Are names consistent with the naming scheme and workspace members updated?

Mapping to External Resources (Optional)
- Primary: The Rust Book; Rust by Example; the Rustlings repo.
- Encourage linking specific chapters in each `section.md` for deeper reading.

Current Sample Mapping
- Lesson 01 (Basics)
  - Section 01 — Variables and Mutability → `lectures/lesson-01-basics/section-01-variables`
  - Section 02 — Functions → `lectures/lesson-01-basics/section-02-functions`
  
- Lesson 02 (Intermediate)
  - Section 01 — Smart Pointers (Box, Rc) → `lectures/lesson-02-intermediate/section-01-smart-pointers`
  - Section 02 — Concurrency (Threads and Join) → `lectures/lesson-02-intermediate/section-02-concurrency`
  - Section 03 — Iterators (Custom + Adapters) → `lectures/lesson-02-intermediate/section-03-iterators-advanced`

- Lesson 03 (Advanced)
  - Section 01 — Unsafe Basics (Raw Pointers) → `lectures/lesson-03-advanced/section-01-unsafe-basics`
  - Section 02 — Advanced Declarative Macros → `lectures/lesson-03-advanced/section-02-advanced-macros`

Roadmap Suggestions
- Fill out the Basics tier first (through modules/workspaces) to ensure a strong foundation.
- Introduce concurrency only after strong ownership fundamentals.
- Treat Async, Advanced Macros, and Unsafe/FFI as opt-in, time-permitting modules.

Appendix: Grading Integration
- Grader expects passing `cargo test` per crate. Avoid long-running tests.
- Keep solutions simple and canonical; avoid non-deterministic or flaky behavior.
- For Windows, `scripts/grade.ps1` auto-discovers crates under `lectures/`.
