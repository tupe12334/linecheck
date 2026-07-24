# linecheck

[![Crates.io](https://img.shields.io/crates/v/linecheck.svg)](https://crates.io/crates/linecheck)
[![CI](https://github.com/tupe12334/linecheck/actions/workflows/ci.yml/badge.svg)](https://github.com/tupe12334/linecheck/actions/workflows/ci.yml)

Stop your AI agent from turning one file into a monolith. `linecheck` enforces per-file line limits so bloated files get caught before they pile up.

## Features

- Set per-file or per-glob line limits
- Configurable warn / error thresholds
- Inline ignore comments and config-level excludes
- Works as a CLI, in CI pipelines, and as a library
- Rust core, WASM bindings for npm, and a Go binding via WASI (pip bindings planned)

## Why linecheck?

Counting lines is easy; enforcing a limit is the actual problem. `cloc`, `tokei`, and `scc` report line counts but have no concept of a threshold — no exit code you can gate CI on. ESLint's `max-lines` gates CI, but only inside a JS/TS ESLint setup, and applies one limit repo-wide. A `wc -l` script in CI can fail a build, but every warn/error tier, per-glob override, and ignore rule has to be hand-rolled and re-maintained.

| | `linecheck` | `cloc` / `tokei` / `scc` | ESLint `max-lines` | `wc -l` script |
| --- | --- | --- | --- | --- |
| Per-file / per-glob thresholds | ✅ | ❌ | one rule per config | hand-rolled |
| Warn vs. error tiers | ✅ | ❌ | ❌ | hand-rolled |
| CI-friendly exit codes | ✅ | ❌ | ✅ (JS/TS only) | ✅ (hand-rolled) |
| Language-agnostic | ✅ | ✅ | ❌ | ✅ |
| `.gitignore`-style nested config | ✅ | ❌ | partial (`overrides`) | ❌ |
| `--json` / `--status` for dashboards | ✅ | partial | ❌ | ❌ |

Use `cloc`/`tokei`/`scc` for a one-off repo-wide line-count report. Use `linecheck` when you want that check enforced automatically, per language, per directory, every PR.

## Installation

**Rust / Cargo**
```bash
cargo install linecheck
```

**npm** — CLI, no Rust toolchain required (runs the real binary under Node's WASI runtime):
```bash
pnpm dlx linecheck .
# or: npx linecheck .
```

Or use it as a library — check in-memory content directly, no filesystem access required:
```bash
npm install linecheck
```
```js
const { check } = require("linecheck");
check("src/main.rs", fileContents); // { status, lines, warn_limit, error_limit, message }
```
See [`npm/linecheck/README.md`](npm/linecheck/README.md) for the full JS API.

**Go**, via [wazero](https://wazero.io) (no cgo): build `crates/wasm-wasi` for
`wasm32-wasip1` and load it from Go — see [`examples/go`](examples/go) for a full
working example.

> pip bindings are planned, built on the same core crate.

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

When `warn_message` / `error_message` are set in the matching rule, the message is appended:
```
src/main.rs: 450 lines (error threshold: 400) — Too large to review easily; split this file now
src/utils.rs: 220 lines (warn threshold: 200) — Getting long — consider splitting into submodules
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

Files are flagged when their line count **exceeds** the threshold — a file with exactly N lines is fine; N+1 triggers the flag.

Preset flags are overridden by any [`linecheck.yml`](linecheck.yml) in scope.

## Schema

A JSON Schema for [`linecheck.yml`](linecheck.yml) is published alongside the codebase at [`schema/linecheck.schema.json`](schema/linecheck.schema.json).

**VS Code / any YAML Language Server editor** — add a modeline at the top of your [`linecheck.yml`](linecheck.yml) for inline validation and autocompletion with no plugin configuration required:

```yaml
# yaml-language-server: $schema=https://raw.githubusercontent.com/tupe12334/linecheck/main/schema/linecheck.schema.json
```

Alternatively, configure it project-wide via `.vscode/settings.json`:

```json
{
  "yaml.schemas": {
    "https://raw.githubusercontent.com/tupe12334/linecheck/main/schema/linecheck.schema.json": "linecheck.yml"
  }
}
```

## Configuration

`linecheck` resolves configuration like `.gitignore` — a [`linecheck.yml`](linecheck.yml) applies to its directory and all subdirectories recursively. A nested config overrides the parent for everything inside it. If no config is found anywhere, it falls back to built-in defaults: **warn when a file exceeds 200 lines, error when it exceeds 400 lines**.


```
project/
├── linecheck.yml        ← applies to everything
└── src/
    ├── linecheck.yml    ← overrides for src/ and below
    └── generated/
        └── linecheck.yml  ← can relax limits for generated code
```

> **Not sure where to start?** 200 lines is a reasonable warn threshold for most source files — it's enough for a focused module but flags anything that's grown too broad.

Create a [`linecheck.yml`](linecheck.yml) at the root of your project to override the defaults:

```yaml
# yaml-language-server: $schema=https://raw.githubusercontent.com/tupe12334/linecheck/main/schema/linecheck.schema.json

rules:
  - pattern: "**/*.rs"
    warn: 200
    warn_message: "Getting long — consider splitting into submodules"
    error: 400
    error_message: "Too large to review easily; split this file now"
  - pattern: "**/*.ts"
    warn: 150
    warn_message: "Consider breaking this into smaller components"
    error: 300
    error_message: "File exceeds the hard limit — split before merging"

exclude:
  - "**/generated/**"
  - "**/vendor/**"
```

Rules are evaluated in order — the **first matching rule wins**. Put more specific patterns before broader ones.

The optional `warn_message` and `error_message` fields set hints printed alongside violations at each severity level. Both are independent — you can set one, the other, or both. Use them to explain the intent of the limit or point to a team convention.

CLI flags override config file values. Run `linecheck --help` for all options.

## Ignoring files

**Exclude globs** — add an `exclude` list to [`linecheck.yml`](linecheck.yml) (see above).

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

## Library usage

Add `linecheck` to your `Cargo.toml` and call `check_file` directly:

```toml
[dependencies]
linecheck = "0.3"
```

```rust
use linecheck::{check_file, CheckOptions, Status};
use std::path::Path;

let result = check_file(
    Path::new("src/main.rs"),
    None,                      // no config file — uses fallback limits
    &CheckOptions::default(),  // warn at 200, error at 400
).unwrap();

match result.status {
    Status::Ok   => println!("ok ({} lines)", result.lines),
    Status::Warn => println!("warn: {} lines exceeds {}", result.lines, result.warn_limit.unwrap()),
    Status::Error => println!("error: {} lines exceeds {}", result.lines, result.error_limit.unwrap()),
}
```

For walking a whole directory tree with config resolution, combine `collect_files` and `ConfigResolver`:

```rust
use linecheck::{collect_files, CheckOptions, ConfigResolver, check_file};
use std::path::PathBuf;

let files = collect_files(&[PathBuf::from("src/")], &[]);
let mut resolver = ConfigResolver::new(None, "linecheck.yml");
let opts = CheckOptions::default();

for path in &files {
    let cfg = resolver.resolve(path);
    let result = check_file(path, cfg.as_ref(), &opts).unwrap();
    println!("{}: {:?}", path.display(), result.status);
}
```

Full API docs at [docs.rs/linecheck](https://docs.rs/linecheck).

## CI examples

See the [`examples/`](examples/) folder for ready-to-use setups:

- `examples/ci/linecheck.sh` — generic CI shell setup (any CI provider)
- `examples/github/workflow.yml` — GitHub Actions workflow (copy to `.github/workflows/linecheck.yml`)

### GitHub Action

`linecheck` is also published as a composite action on the [GitHub Marketplace](https://github.com/marketplace/actions/linecheck), so you don't need to copy a workflow file by hand:

```yaml
- uses: tupe12334/linecheck@main
  with:
    paths: src/
    args: --strict
```

| Input | Default | Description |
| --- | --- | --- |
| `paths` | `.` | Space-separated paths to check |
| `args` | *(empty)* | Extra CLI flags (`--strict`, `--status`, `--json`, ...) |
| `version` | *(latest)* | Pin a specific `linecheck` version |

## Related projects

- [moadim](https://moadim.io/) — loop engineering: build, schedule & run agent loops.

## License

[MIT](LICENSE)
