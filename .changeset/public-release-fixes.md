---
"linecheck": patch
---

Fix several issues discovered during public release audit:

- **Self-ignoring bug**: `src/lines.rs` contained the ignore marker as a raw
  string literal, causing `linecheck` to silently skip its own core file when
  checking the project. The marker constant now uses `\x3a` for `:` so the
  source contains no raw occurrence of the sequence.
- **Inaccurate `--status` output format in README**: example showed colons
  after filenames (`src/main.rs:`) but actual output uses space-aligned columns.
- **Wrong Presets table**: `--default` was listed as "200 lines" but is
  200 warn / 400 error — two thresholds. Table now has separate Warn/Error columns.
- **CHANGELOG used wrong ignore syntax**: `linecheck-ignore` → `linecheck:ignore`.
- **Crates.io package contained JS tooling files**: `.changeset/`, `.claude/`,
  `.githooks/`, `package.json`, `pnpm-lock.yaml`, `scripts/` are now excluded.
- **Missing MSRV**: `rust-version = "1.85"` added (required by Rust 2024 edition).
- **Clippy warnings**: `map_or(false, …)` replaced with `is_some_and(…)`.
- **Self-compliance**: project now passes its own `linecheck .` check.
- **CI**: added workflow to run tests and clippy on every push and pull request.
