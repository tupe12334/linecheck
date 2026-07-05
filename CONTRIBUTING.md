# Contributing to linecheck

Thanks for your interest in contributing!

## Prerequisites

- Rust 1.85+ (`rustup update stable`)
- Node.js 20+ and pnpm 9 (only needed for changesets / releases)
- The `wasm32-unknown-unknown` target (only needed to build/test `crates/wasm`, the WASM bindings crate)

## Development workflow

```bash
# Run all tests
cargo test

# Check for lint issues
cargo clippy --all-targets -- -D warnings

# Build the CLI
cargo build
```

## Making changes

1. Fork the repo and create a branch.
2. Make your change and add or update tests as needed.
3. Run the full local check — all three must pass:
   ```bash
   cargo test
   cargo clippy --all-targets -- -D warnings
   RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps
   ```
4. Open a pull request; the CI workflow will verify your branch automatically.

## Releasing (maintainers only)

This project uses [changesets](https://github.com/changesets/changesets) for versioning.

```bash
# Describe your change
pnpm changeset

# Changesets Action on CI handles the actual publish to crates.io
```

## Code style

- No comments that restate what the code does — only explain *why* when non-obvious.
- Keep individual `.rs` files under the limits in `linecheck.yml` (the tool enforces itself).
