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
- Restore the entire `lectures/` tree from the snapshot:
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
- The reset scripts now reset the entire `lectures/` tree (docs, tests, lib, etc.). They overwrite from the baseline snapshot or a git ref but do not remove extra files not present in the snapshot.

Grader UI (optional)
- A minimal desktop UI is available under `tools/grader_ui` (not part of the workspace to keep grading fast).
- Build and run it from repo root:
  - Windows: `cargo run --manifest-path tools/grader_ui/Cargo.toml`
  - macOS/Linux: `cargo run --manifest-path tools/grader_ui/Cargo.toml`
- Features:
  - Discover packages under `lectures/` using `cargo metadata`.
  - Grade Selected (runs `cargo test -p ...`), Grade All (workspace).
  - Snapshot/Restore baseline via `scripts/reset_libs.*`.
  - Restore from a git ref (e.g., `origin/main`).

Basic usage
- Click `Discover Packages` to list all `lectures/**` crates.
- Select some packages (or `Select All`).
- Click `Grade Selected` to run tests for those crates, or `Grade All (workspace)` to run the full workspace.
- To reset starter files:
  - `Snapshot Baseline` creates/updates a snapshot at `scripts/baseline/lectures/`.
  - `Restore Baseline` copies the snapshot back onto `lectures/`.
  - `Restore from Git Ref` resets `lectures/` from a ref (e.g., `origin/main`).

Notes
- The UI streams command output into the lower pane and prints an exit status when done.
- It runs your installed `cargo`, PowerShell (`.ps1`) on Windows, or `bash` (`.sh`) on macOS/Linux.
- If you see a workspace error when running the UI, ensure the root `Cargo.toml` contains `exclude = ["tools/grader_ui"]`.

Lesson Plan
- See `lesson_plan/overview.md` for the full course outline, authoring guidelines, and roadmap.
- Use `lesson_plan/section-template.md` when creating new sections.
