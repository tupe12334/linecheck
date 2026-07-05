#!/bin/bash
# Builds the WASM library bindings, then publishes npm/linecheck.
# Its package.json is committed and version-synced by scripts/sync-version.mjs.
set -eo pipefail

wasm-pack build crates/wasm --target nodejs --out-dir ../../npm/linecheck/dist --out-name linecheck --release

OUTPUT=$(npm publish npm/linecheck --access public 2>&1) || {
  echo "$OUTPUT"
  if echo "$OUTPUT" | grep -q 'cannot publish over'; then
    echo "Version already published, skipping"
    exit 0
  fi
  exit 1
}
echo "$OUTPUT"
