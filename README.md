# linecheck

> **Alpha** — experimental, expect breaking changes.

A fast, configurable tool that warns or errors when files exceed a set line count. Keep your files small by making the limit visible and enforced.

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

**Inline ignore** — add a comment at the top of a file to exempt it entirely:

```rust
// linecheck:ignore
```

```python
# linecheck:ignore
```

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
