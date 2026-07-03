#!/usr/bin/env sh
# Generic CI shell setup for linecheck.
# Run from the root of your repository.
set -e

# Install linecheck (requires Rust/cargo)
if ! command -v linecheck >/dev/null 2>&1; then
  cargo install linecheck
fi

# Check all files against your linecheck.yml config.
# Exit code 1 if any file exceeds its error threshold.
linecheck .
