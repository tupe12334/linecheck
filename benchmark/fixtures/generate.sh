#!/usr/bin/env sh
# Generate synthetic source files for benchmarking.
# Usage: ./generate.sh [--files N] [--lines N]
set -e

FILES=500
LINES=300

while [ $# -gt 0 ]; do
  case "$1" in
    --files) FILES="$2"; shift 2 ;;
    --lines) LINES="$2"; shift 2 ;;
    *) echo "Unknown option: $1" >&2; exit 1 ;;
  esac
done

DIR="$(cd "$(dirname "$0")" && pwd)/src"
rm -rf "$DIR"
mkdir -p "$DIR"

i=1
while [ "$i" -le "$FILES" ]; do
  file="$DIR/file_$(printf '%04d' "$i").rs"
  {
    j=1
    while [ "$j" -le "$LINES" ]; do
      printf '// line %d of %d in file %d\n' "$j" "$LINES" "$i"
      j=$((j + 1))
    done
  } > "$file"
  i=$((i + 1))
done

echo "Generated $FILES files × $LINES lines in $DIR"
