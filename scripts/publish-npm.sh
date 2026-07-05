#!/bin/bash
# Builds the WASM library bindings + WASI CLI, then publishes npm/linecheck.
# Its package.json is committed and version-synced by scripts/sync-version.mjs.
set -eo pipefail

# Library bindings (require("linecheck").check(...)) — no filesystem access.
wasm-pack build crates/wasm --target nodejs --out-dir ../../npm/linecheck/dist --out-name linecheck --release

# CLI (pnpm dlx / npx linecheck ...) — the real src/main.rs binary, compiled to
# WASI so it keeps full filesystem access (directory walks, config resolution),
# run under Node's built-in WASI runtime via npm/linecheck/bin/cli.js.
cargo build --release --target wasm32-wasip1 --bin linecheck
cp target/wasm32-wasip1/release/linecheck.wasm npm/linecheck/linecheck-cli.wasm

OUTPUT=$(npm publish npm/linecheck --access public 2>&1) || {
  echo "$OUTPUT"
  if echo "$OUTPUT" | grep -q 'cannot publish over'; then
    echo "Version already published, skipping"
    exit 0
  fi
  exit 1
}
echo "$OUTPUT"
