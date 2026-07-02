# linecheck

A fast, configurable tool that warns or errors when files exceed a set line count. Keep your files small by making the limit visible and enforced.

## Features

- Set per-file or per-glob line limits
- Configurable warn / error thresholds
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

Create a `linecheck.yml` (YAML) at the root of your project:

```yaml
rules:
  - pattern: "**/*.rs"
    warn: 200
    error: 400
  - pattern: "**/*.ts"
    warn: 150
    error: 300
```
