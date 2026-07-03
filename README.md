# linecheck

[![Crates.io](https://img.shields.io/crates/v/linecheck.svg)](https://crates.io/crates/linecheck)
[![CI](https://github.com/tupe12334/linecheck/actions/workflows/ci.yml/badge.svg)](https://github.com/tupe12334/linecheck/actions/workflows/ci.yml)

> **Alpha** — experimental, expect breaking changes.

Stop your AI agent from turning one file into a monolith. `linecheck` enforces per-file line limits so bloated files get caught before they pile up.

## Features

- Set per-file or per-glob line limits
- Configurable warn / error thresholds
- Inline ignore comments and config-level excludes
- Works as a CLI, in CI pipelines, and as a library
- Rust core (npm, pip, Go, and WASM bindings planned)

## Installation

**Rust / Cargo**
```bash
cargo install linecheck
```

> npm, pip, and Go bindings are coming soon.

## Usage

```bash
linecheck [OPTIONS] [PATHS]...
```

Check all files in the current directory using your config:
```bash
linecheck .
```

Override the line limit inline:
```bash
linecheck --max-lines 200 src/
```

Example output:
```
src/main.rs: 450 lines (error threshold: 400)
src/utils.rs: 220 lines (warn threshold: 200)
```

See which files are creeping toward their limit before they breach it:
```bash
linecheck --status src/
```
```
src/main.rs   450 / 400  [ERROR]
src/utils.rs  220 / 200  [WARN]
src/lib.rs    180 / 200  90%
src/config.rs  45 / 200  22%
```

For scripting and CI dashboards, use `--json`. Without `--status` it outputs only violations:
```bash
linecheck --json src/
```
```json
[
  {"file":"src/main.rs","lines":450,"limit":400,"percent":112,"status":"error"},
  {"file":"src/utils.rs","lines":220,"limit":200,"percent":110,"status":"warn"}
]
```

Add `--status` to include all files regardless of their status:
```bash
linecheck --status --json src/
```

Each object contains: `file` (path), `lines` (count), `limit` (threshold used), `percent` (lines × 100 / limit), `status` (`"ok"`, `"warn"`, or `"error"`).

## Presets

Use a preset flag to apply a built-in strictness level without writing a config file:

| Flag        | Warn  | Error     |
| ----------- | ----- | --------- |
| `--strict`  | 100   | 100       |
| `--default` | 200   | 400       |
| `--loose`   | 400   | 400       |
| `--free`    | —     | —         |

```bash
linecheck --strict src/
linecheck --loose .
```

Preset flags are overridden by any `linecheck.yml` in scope.

## Configuration

`linecheck` resolves configuration like `.gitignore` — a `linecheck.yml` applies to its directory and all subdirectories recursively. A nested config overrides the parent for everything inside it. If no config is found anywhere, it falls back to built-in defaults: **warn at 200 lines, error at 400 lines** for all files.

```
project/
├── linecheck.yml        ← applies to everything
└── src/
    ├── linecheck.yml    ← overrides for src/ and below
    └── generated/
        └── linecheck.yml  ← can relax limits for generated code
```

> **Not sure where to start?** 200 lines is a reasonable warn threshold for most source files — it's enough for a focused module but flags anything that's grown too broad.

Create a `linecheck.yml` at the root of your project to override the defaults:

```yaml
rules:
  - pattern: "**/*.rs"
    warn: 200
    error: 400
  - pattern: "**/*.ts"
    warn: 150
    error: 300

exclude:
  - "**/generated/**"
  - "**/vendor/**"
```

When multiple rules match the same file, the most specific pattern wins.

CLI flags override config file values. Run `linecheck --help` for all options.

## Ignoring files

**Exclude globs** — add an `exclude` list to `linecheck.yml` (see above).

**Inline ignore** — add this comment anywhere in the file to exempt it entirely:

```
# linecheck:ignore
```

The file will be skipped regardless of its line count. There is no partial ignore — it's all-or-nothing.

> **Note:** detection is a raw byte scan, so any file that contains the literal string `linecheck:ignore` anywhere (including in string literals or documentation) will be treated as ignored. If your source code references the marker as a string constant, write the colon as `\x3a` in the Rust/C literal so the raw sequence does not appear in the file.

## Exit codes

| Code | Meaning |
| ---- | ------- |
| `0`  | All files within limits (warnings are printed but non-blocking) |
| `1`  | One or more files exceed the error threshold |

## CI examples

See the [`examples/`](examples/) folder for ready-to-use configurations:

- `examples/ci/` — generic CI shell setup
- `examples/github/` — GitHub Actions workflow

## License

[MIT](LICENSE)
