#!/usr/bin/env bash
set -euo pipefail

# Reset all lectures/**/src/lib.rs back to a baseline snapshot or a git ref.
#
# Usage:
#   ./scripts/reset_libs.sh --init            # snapshot current files as baseline
#   ./scripts/reset_libs.sh                   # restore from baseline
#   ./scripts/reset_libs.sh --git-ref <ref>   # restore from a git ref (e.g., origin/main)

here="$(cd "$(dirname "$0")" && pwd)"
repo_root="$(cd "$here/.." && pwd)"
lectures_root="$repo_root/lectures"
baseline_root="$here/baseline"

init=false
git_ref=""
dry_run=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    -i|--init) init=true; shift ;;
    --git-ref) git_ref="${2:-}"; shift 2 ;;
    -n|--dry-run) dry_run=true; shift ;;
    *) echo "Unknown option: $1" >&2; exit 2 ;;
  esac
done

info() { printf '\033[36m%s\033[0m\n' "$*"; }
act()  { printf '\033[33m%s\033[0m\n' "$*"; }
warn() { printf '\033[33m%s\033[0m\n' "$*"; }
err()  { printf '\033[31m%s\033[0m\n' "$*"; }

mapfile -t files < <(find "$lectures_root" -type f -path '*/src/lib.rs' | sort)
if [[ ${#files[@]} -eq 0 ]]; then
  warn "No lib.rs files found under lectures/"
  exit 0
fi

if $init; then
  info "Initializing baseline snapshot from current files..."
  for f in "${files[@]}"; do
    rel="${f#"$repo_root/"}"
    dest="$baseline_root/$rel"
    mkdir -p "$(dirname "$dest")"
    if $dry_run; then
      act "Would snapshot: $rel -> scripts/baseline/$rel"
    else
      cp -f "$f" "$dest"
      act "Snapshotted: $rel"
    fi
  done
  info "Baseline initialized in scripts/baseline."
  exit 0
fi

used_git=false
if [[ -n "$git_ref" ]] && git -C "$repo_root" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  info "Restoring from git ref: $git_ref"
  for f in "${files[@]}"; do
    rel="${f#"$repo_root/"}"
    if $dry_run; then
      act "Would git-restore $rel from $git_ref"
    else
      if ! git -C "$repo_root" restore --worktree --source "$git_ref" -- "$rel" 2>/dev/null; then
        git -C "$repo_root" checkout "$git_ref" -- "$rel"
      fi
    fi
  done
  used_git=true
fi

if ! $used_git; then
  if [[ ! -d "$baseline_root" ]]; then
    err "No baseline found at scripts/baseline and no --git-ref provided."
    err "Run:  ./scripts/reset_libs.sh --init   to snapshot the current starter state."
    exit 1
  fi
  info "Restoring from local baseline snapshot..."
  for f in "${files[@]}"; do
    rel="${f#"$repo_root/"}"
    src="$baseline_root/$rel"
    if [[ ! -f "$src" ]]; then
      warn "Missing baseline for $rel; skipping"
      continue
    fi
    if $dry_run; then
      act "Would restore: $rel <- baseline"
    else
      cp -f "$src" "$f"
      act "Restored: $rel"
    fi
  done
fi

info "Done."

