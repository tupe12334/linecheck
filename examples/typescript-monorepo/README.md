# TypeScript monorepo example

Shows `linecheck` applying **different limits to different parts of a JS/TS
monorepo** in one config — something a single repo-wide ESLint `max-lines`
rule can't do.

```
typescript-monorepo/
├── linecheck.yml
├── apps/
│   └── web/src/          ← app code, stricter limit (warn 120 / error 200)
├── packages/
│   ├── ui/src/            ← shared library code, default limit (warn 200 / error 400)
│   └── generated/         ← codegen output, excluded entirely
```

## Run it

```bash
cd examples/typescript-monorepo
linecheck .
```

`linecheck.yml` here applies a tighter threshold to `apps/**` (UI code tends
to sprawl fastest), a looser default to `packages/**`, and excludes
`packages/generated/**` so generated code never trips the check — all from
one file, with no per-package ESLint config or plugin required.
