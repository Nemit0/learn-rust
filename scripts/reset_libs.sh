#!/usr/bin/env bash
set -euo pipefail

# Reset the entire lectures/ tree back to a baseline snapshot or a git ref.
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

if [[ ! -d "$lectures_root" ]]; then
  err "Lectures folder not found at '$lectures_root'"
  exit 1
fi

if $init; then
  info "Initializing baseline snapshot of entire 'lectures/' tree..."
  dest_lectures="$baseline_root/lectures"
  mkdir -p "$dest_lectures"
  if $dry_run; then
    act "Would snapshot: lectures/ -> scripts/baseline/lectures/"
  else
    rm -rf "$dest_lectures"
    mkdir -p "$dest_lectures"
    cp -R "$lectures_root/." "$dest_lectures"
    act "Snapshotted: lectures/"
  fi
  info "Baseline initialized in scripts/baseline/lectures."
  exit 0
fi

used_git=false
if [[ -n "$git_ref" ]] && git -C "$repo_root" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  info "Restoring entire 'lectures/' from git ref: $git_ref"
  if $dry_run; then
    act "Would git-restore lectures/ from $git_ref"
  else
    if ! git -C "$repo_root" restore --worktree --source "$git_ref" -- lectures 2>/dev/null; then
      git -C "$repo_root" checkout "$git_ref" -- lectures
    fi
  fi
  used_git=true
fi

if ! $used_git; then
  baseline_lectures="$baseline_root/lectures"
  if [[ ! -d "$baseline_lectures" ]]; then
    err "No baseline found at scripts/baseline/lectures and no --git-ref provided."
    err "Run:  ./scripts/reset_libs.sh --init   to snapshot the current starter state."
    exit 1
  fi
  info "Restoring entire 'lectures/' from local baseline snapshot..."
  if $dry_run; then
    act "Would restore: lectures/ <- baseline"
  else
    cp -R "$baseline_lectures/." "$lectures_root"
    act "Restored: lectures/"
  fi
fi

info "Done."
