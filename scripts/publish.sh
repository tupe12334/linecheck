#!/bin/bash
set -eo pipefail
OUTPUT=$(cargo publish --token "$CARGO_REGISTRY_TOKEN" --allow-dirty 2>&1) || {
  echo "$OUTPUT"
  if echo "$OUTPUT" | grep -q 'already exists'; then
    echo "Version already published, skipping"
    exit 0
  fi
  exit 1
}
echo "$OUTPUT"
