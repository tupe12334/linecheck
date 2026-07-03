---
"linecheck": patch
---

Fix several issues discovered during public release audit:

- **Self-ignoring bug**: `src/lines.rs` and `tests/integration.rs` contained
  the ignore marker as raw string literals, causing `linecheck` to silently skip
  its own core files. The marker now uses `\x3a` for `:` in all source files.
- **Rule-matching docs corrected**: README claimed "most specific pattern wins"
  but the code uses first-match semantics. Docs now accurately say "first
  matching rule wins". A test locks in this behaviour.
- **`display` module made private**: output-formatting functions (`print_violations`,
  `print_status`, `print_json`) were exported from the library crate despite being
  CLI implementation detail. Moved to the binary crate; public library surface is
  now `checker`, `config`, `files`, `lines`, `preset`.
- **`unwrap()` removed from library code**: `count_newlines` used `.unwrap()` on
  `data.last()` guarded by an `is_empty()` check. Replaced with `data.last() != Some(&b'\n')`.
- **Missing `--config` error**: passing a non-existent explicit `--config` path
  silently fell back to defaults. Now exits 1 with a clear error message.
- **Inaccurate README**: fixed `--status` output format (removed colons), Presets
  table (separate Warn/Error columns), usage line (`[FILES...]` → `[PATHS]...`),
  and documented both `--json` modes.
- **Crates.io package cleanup**: `.changeset/`, `.claude/`, `.githooks/`,
  `package.json`, `pnpm-lock.yaml`, `scripts/` excluded from published crate.
- **Missing crates.io metadata**: added `readme`, `documentation`, `rust-version`,
  `repository`, `homepage`, `keywords`, `categories`.
- **CI improvements**: added `cargo doc` with `RUSTDOCFLAGS="-D missing_docs"` to
  catch undocumented public API additions.
- **CHANGELOG typo**: `linecheck-ignore` → `` `linecheck:ignore` ``.
- **Self-compliance**: project now passes its own `linecheck .` check.
