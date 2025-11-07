Learn Rust — Lectures and Grader

[한국어(KO)](README.ko.md)

Overview
- A Cargo workspace of small, focused Rust exercises organized as lectures and sections.
- Each section contains:
  - `section.md` — instructions for the exercise.
  - `src/lib.rs` — incomplete code to fill in.
  - `answer.rs` — a reference solution (not compiled).
  - `tests/` — tests used to grade your solution.

Prerequisites
- Rust toolchain (rustup): `rustc --version` should work.
- Cargo (comes with rustup).

Project Structure
- `lectures/lesson-01-basics/section-01-variables`
- `lectures/lesson-01-basics/section-02-functions`
- `scripts/grade.ps1` — PowerShell grader for Windows.
- `scripts/grade.sh` — Minimal grader for macOS/Linux.

Workflow
1) Pick a section under `lectures/...` and read its `section.md`.
2) Edit `src/lib.rs` to complete the exercise.
3) Run tests for that section, e.g.:
   - `cargo test -p lesson01_section01_variables`
   - `cargo test -p lesson01_section02_functions`
4) Grade all sections together:
   - Windows: `./scripts/grade.ps1`
   - macOS/Linux: `bash ./scripts/grade.sh`

Resetting to Starter State
- Snapshot current starter files (do this once):
  - Windows: `./scripts/reset_libs.ps1 -Init`
  - macOS/Linux: `bash ./scripts/reset_libs.sh --init`
- Restore all `lectures/**/src/lib.rs` from the snapshot:
  - Windows: `./scripts/reset_libs.ps1`
  - macOS/Linux: `bash ./scripts/reset_libs.sh`
- If your project is in git and you want to restore from a branch or commit instead of the snapshot:
  - Windows: `./scripts/reset_libs.ps1 -GitRef origin/main`
  - macOS/Linux: `bash ./scripts/reset_libs.sh --git-ref origin/main`

Adding New Sections
- Create a new crate under `lectures/<lesson-name>/<section-name>/` with its own `Cargo.toml`, `src/lib.rs`, `tests/` and `section.md`.
- Add the new path to the `[workspace].members` array in the root `Cargo.toml`.
- Optional: the Windows grader auto-discovers packages under `lectures/` via `cargo metadata`; no changes needed there.

Notes
- `answer.rs` is not compiled; it’s just for reference.
- Keep exercises small and focused; tests should express the learning goal.
- The reset scripts only touch `src/lib.rs` files under `lectures/`.

Lesson Plan
- See `lesson_plan/overview.md` for the full course outline, authoring guidelines, and roadmap.
- Use `lesson_plan/section-template.md` when creating new sections.
