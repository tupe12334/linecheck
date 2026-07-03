# linecheck

## 0.3.0

### Minor Changes

- Add library API, built-in presets, JSON output, and hierarchical config resolution.

  - **Library API**: `linecheck` is now also a Rust library crate — `checker`, `config`, `files`, `display`, and `preset` modules are all public.
  - **Presets**: `--strict` (100 lines), `--default` (200 warn / 400 error), `--loose` (400 lines), and `--free` (unlimited) flags let you pick a built-in strictness level without a config file.
  - **JSON output**: `--json` flag emits structured results for integration with other tools.
  - **Hierarchical config**: without an explicit `--config`, linecheck walks up the directory tree to find the nearest `linecheck.yml`, enabling per-subdirectory overrides.

## 0.2.0

### Minor Changes

- 455ba73: Initial release of `linecheck` — a fast, configurable CLI that warns or errors when files exceed a set line count. Supports glob patterns, per-file overrides via inline `# linecheck:ignore` comments, and YAML configuration.
