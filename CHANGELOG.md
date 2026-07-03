# linecheck

## 0.3.1

### Patch Changes

- 55d8a04: Fix numerous issues discovered during public release audit:

  - **Self-ignoring bug**: `src/lines.rs` and `tests/integration.rs` contained the ignore
    marker as raw string literals, causing `linecheck` to silently skip its own core files.
  - **Rule-matching docs corrected**: README claimed "most specific pattern wins" but the
    code uses first-match semantics. Docs corrected; a test locks in the behaviour.
  - **`display` module made private**: CLI output functions moved from library to binary;
    public library surface is now `checker`, `config`, `files`, `lines`, `preset`.
  - **`unwrap()` removed from library**: `count_newlines` replaced `.unwrap()` with a safe
    `data.last() != Some(&b'\n')` comparison.
  - **`--config` missing file now errors**: explicit `--config nonexistent.yml` silently
    fell back to defaults; now exits 1 with a clear message.
  - **`./linecheck.yml` treated as default**: `--config ./linecheck.yml` was treated as an
    explicit path (erroring when absent) rather than as the default name.
  - **Division by zero in `--status`/`--json`**: `warn: 0` or `error: 0` in config caused
    a panic when computing the percent field.
  - **Incomplete JSON escaping**: `--json` output didn't escape `\n`, `\r`, `\t`, or
    control characters, producing invalid JSON for paths containing those bytes.
  - **`--json` bracket formatting**: closing `]` appeared on same line as last item,
    contradicting the README example.
  - **Invalid pattern warning**: glob patterns that fail to parse were silently skipped in
    both `load_config` and `ConfigResolver`; now emits a warning to stderr.
  - **Nonexistent path silently ignored**: `linecheck /typo` exited 0 with no output;
    now prints `Warning: path not found:` to stderr.
  - **Missing derives on public types**: `FileResult` and `CheckOptions` had no derives;
    added `Debug`/`Clone` to both, `Copy`/`PartialEq`/`Eq` to `FileResult`.
    `Config` and `Rule` were also missing `PartialEq`; added to match the rest of the API.
  - **Invalid exclude patterns silently dropped**: `warn_invalid_patterns` only checked
    rule globs, not the `exclude` list; extended to warn on both.
  - **Crates.io metadata**: added `readme`, `documentation`, `rust-version`, `repository`,
    `homepage`, `keywords`, `categories`.
  - **CI**: added `cargo doc` with `RUSTDOCFLAGS="-D missing_docs"` step.
  - **Self-compliance**: project passes its own `linecheck .` check.

## 0.3.0

### Minor Changes

- Add library API, built-in presets, JSON output, and hierarchical config resolution.

  - **Library API**: `linecheck` is now also a Rust library crate — `checker`, `config`, `files`, `lines`, and `preset` modules are public.
  - **Presets**: `--strict` (100 lines), `--default` (200 warn / 400 error), `--loose` (400 lines), and `--free` (unlimited) flags let you pick a built-in strictness level without a config file.
  - **JSON output**: `--json` flag emits structured results for integration with other tools.
  - **Hierarchical config**: without an explicit `--config`, linecheck walks up the directory tree to find the nearest `linecheck.yml`, enabling per-subdirectory overrides.

## 0.2.0

### Minor Changes

- 455ba73: Initial release of `linecheck` — a fast, configurable CLI that warns or errors when files exceed a set line count. Supports glob patterns, per-file overrides via inline `# linecheck:ignore` comments, and YAML configuration.
