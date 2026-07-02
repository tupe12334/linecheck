# linecheck

> **Alpha** — experimental, expect breaking changes.

Stop your AI agent from turning one file into a monolith. `linecheck` enforces per-file line limits so bloated files get caught before they pile up.

## Features

- Set per-file or per-glob line limits
- Configurable warn / error thresholds
- Inline ignore comments and config-level excludes
- Works as a CLI, in CI pipelines, and as a library
- Rust core with bindings for npm, pip, Go, and WASM

## Installation

> Bindings are coming soon. The Rust CLI is the first target.

**Rust / Cargo**
```bash
cargo install linecheck
```

**npm**
```bash
npm install -g linecheck
```

**pip**
```bash
pip install linecheck
```

**Go**
```bash
go install github.com/tupe12334/linecheck@latest
```

## Usage

```bash
linecheck [OPTIONS] [FILES...]
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

## Configuration

Create a `linecheck.yml` at the root of your project:

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

CLI flags override config file values. Run `linecheck --help` for all options.

## Ignoring files

**Exclude globs** — add an `exclude` list to `linecheck.yml` (see above).

**Inline ignore** — add this comment anywhere in the file to exempt it entirely:

```
# linecheck:ignore
```

The file will be skipped regardless of its line count. There is no partial ignore — it's all-or-nothing.

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
