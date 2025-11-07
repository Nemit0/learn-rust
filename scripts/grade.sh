#!/usr/bin/env bash
set -euo pipefail

# Minimal grader for macOS/Linux: runs all workspace tests.
# For per-section runs, use: cargo test -p <package_name>

cargo test --workspace

