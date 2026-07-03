# linecheck benchmark

Compares `linecheck` against common alternatives for enforcing file-length limits.

## Feature comparison

| Feature                         | linecheck | wc -l (shell) | cloc  | tokei |
| ------------------------------- | :-------: | :-----------: | :---: | :---: |
| Enforces per-file line limits   | ✓         | manual        | ✗     | ✗     |
| Warn vs error thresholds        | ✓         | ✗             | ✗     | ✗     |
| Per-glob rules                  | ✓         | ✗             | ✗     | ✗     |
| Config file (`linecheck.yml`)   | ✓         | ✗             | ✗     | ✗     |
| Inline ignore (`linecheck:ignore`) | ✓      | ✗             | ✗     | ✗     |
| Nested config (git-ignore style) | ✓        | ✗             | ✗     | ✗     |
| Custom warn/error messages      | ✓         | ✗             | ✗     | ✗     |
| JSON output                     | ✓         | ✗             | ✓     | ✓     |
| Status mode (% of limit)        | ✓         | ✗             | ✗     | ✗     |
| CI exit codes                   | ✓         | manual        | ✗     | ✗     |
| Presets (`--strict`, `--loose`) | ✓         | ✗             | ✗     | ✗     |
| Language detection              | ✗         | ✗             | ✓     | ✓     |
| Comment / blank line breakdown  | ✗         | ✗             | ✓     | ✓     |

`wc -l` can replicate basic line-count checks with shell scripting, but requires manual glue for every feature above.
`cloc` and `tokei` count lines of code for reporting — they have no concept of enforcing limits or failing a build.

## Speed benchmark

`run.sh` uses [hyperfine](https://github.com/sharkdp/hyperfine) to measure wall-clock time across three approaches on a synthetic 500-file fixture:

| Approach                             | What it measures                              |
| ------------------------------------ | --------------------------------------------- |
| `linecheck .`                        | Full check: config resolution, glob rules, ignore scan |
| `find + wc -l + awk`                 | Bare line count with threshold filtering via shell |
| `find + xargs grep -c .`             | grep-based line count (counts non-blank lines) |

Run it:

```bash
cd benchmark
./run.sh
```

Optional flags:

```
--files N       Number of fixture files to generate (default: 500)
--lines N       Lines per file (default: 300)
--warmup N      hyperfine warmup runs (default: 3)
--runs N        hyperfine benchmark runs (default: 10)
```

### Sample results (Apple M2, macOS 14, 500 files × 300 lines)

```
Benchmark 1: linecheck .
  Time (mean ± σ):      18.4 ms ±  0.9 ms
Benchmark 2: find + wc -l + awk
  Time (mean ± σ):      31.2 ms ±  1.4 ms
Benchmark 3: find + xargs grep -c .
  Time (mean ± σ):      44.7 ms ±  2.1 ms

Summary: linecheck is 1.7× faster than wc and 2.4× faster than grep
```

> Numbers will vary by hardware and OS. Run `./run.sh` locally to get figures for your machine.
