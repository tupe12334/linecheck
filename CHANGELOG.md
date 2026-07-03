# linecheck

## 1.0.0

### Major Changes

- Release 1.0.0 — stable public API and production-ready CLI.

  This release marks `linecheck` as stable and production-ready. It includes all fixes and improvements accumulated since the initial public release, and commits to a stable library and CLI interface going forward.

  ### Breaking changes

  - `display` module is no longer part of the public library API (moved to the binary crate).

  ### New features

  - **Library API**: `linecheck` is a Rust library crate — `checker`, `config`, `files`, `lines`, and `preset` modules are public with stable interfaces.
  - **Presets**: `--strict`, `--default`, `--loose`, and `--free` flags for built-in strictness levels without a config file.
  - **JSON output**: `--json` flag emits structured results for tool integration.
  - **Hierarchical config**: without `--config`, linecheck walks up the directory tree to find the nearest `linecheck.yml`.

  ### Bug fixes

  - Self-ignoring bug: `linecheck:ignore` in string literals no longer silently skips core files.
  - `--config` with a missing file now exits 1 with a clear error message.
  - `./linecheck.yml` correctly treated as the default config name, not an explicit path.
  - Division by zero eliminated when `warn: 0` or `error: 0` in `--status`/`--json` mode.
  - JSON output now correctly escapes all control characters.
  - Invalid glob patterns emit a warning to stderr instead of being silently skipped.
  - Nonexistent paths passed as arguments now warn to stderr.
  - Config resolver correctly handles root-level hierarchical lookup.

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
