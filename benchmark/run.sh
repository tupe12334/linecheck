#!/usr/bin/env sh
# Benchmark linecheck against shell-based alternatives.
# Requires: linecheck, hyperfine (https://github.com/sharkdp/hyperfine)
set -e

FILES=500
LINES=300
WARMUP=3
RUNS=10
LIMIT=200

while [ $# -gt 0 ]; do
  case "$1" in
    --files)   FILES="$2";  shift 2 ;;
    --lines)   LINES="$2";  shift 2 ;;
    --warmup)  WARMUP="$2"; shift 2 ;;
    --runs)    RUNS="$2";   shift 2 ;;
    *) echo "Unknown option: $1" >&2; exit 1 ;;
  esac
done

BENCH_DIR="$(cd "$(dirname "$0")" && pwd)"
FIXTURE_DIR="$BENCH_DIR/fixtures/src"

# ── dependency checks ──────────────────────────────────────────────────────────

if ! command -v linecheck >/dev/null 2>&1; then
  echo "linecheck not found — install with: cargo install linecheck" >&2
  exit 1
fi

if ! command -v hyperfine >/dev/null 2>&1; then
  echo "hyperfine not found — install from https://github.com/sharkdp/hyperfine" >&2
  echo "  macOS:  brew install hyperfine" >&2
  echo "  cargo:  cargo install hyperfine" >&2
  exit 1
fi

# ── fixture generation ─────────────────────────────────────────────────────────

if [ ! -d "$FIXTURE_DIR" ]; then
  echo "Generating fixtures ($FILES files × $LINES lines)…"
  sh "$BENCH_DIR/fixtures/generate.sh" --files "$FILES" --lines "$LINES"
else
  ACTUAL=$(find "$FIXTURE_DIR" -name '*.rs' | wc -l | tr -d ' ')
  if [ "$ACTUAL" -ne "$FILES" ]; then
    echo "Regenerating fixtures ($FILES files × $LINES lines)…"
    sh "$BENCH_DIR/fixtures/generate.sh" --files "$FILES" --lines "$LINES"
  fi
fi

# Write a minimal linecheck config scoped to the fixture directory so the
# benchmark does not inherit the repo's own strict rules.
FIXTURE_CFG="$BENCH_DIR/fixtures/linecheck.yml"
cat > "$FIXTURE_CFG" <<EOF
rules:
  - pattern: "**/*.rs"
    warn: $LIMIT
    error: $((LIMIT * 2))
EOF

echo ""
echo "Fixture : $FIXTURE_DIR"
echo "Files   : $FILES × $LINES lines"
echo "Limit   : warn=$LIMIT, error=$((LIMIT * 2))"
echo ""

# ── benchmarks ─────────────────────────────────────────────────────────────────

hyperfine \
  --warmup "$WARMUP" \
  --runs   "$RUNS" \
  --export-markdown "$BENCH_DIR/results.md" \
  "linecheck --config '$FIXTURE_CFG' '$FIXTURE_DIR'" \
  "find '$FIXTURE_DIR' -name '*.rs' | xargs wc -l | awk '\$1 > $LIMIT {print \$2, \$1}'" \
  "find '$FIXTURE_DIR' -name '*.rs' -exec grep -c . {} + 2>/dev/null | awk -F: '\$2 > $LIMIT {print \$1, \$2}'"

echo ""
echo "Results written to benchmark/results.md"
