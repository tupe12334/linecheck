# Python monorepo example

Shows `linecheck` applying **different limits to different parts of a Python
monorepo** in one config — something pylint's `max-module-lines` (one limit
per pylintrc, no warn/error tiers) can't express.

```
python-monorepo/
├── linecheck.yml
├── services/
│   └── api/app/          ← service code, stricter limit (warn 150 / error 250)
└── libs/
    └── core/core/         ← shared library code, looser limit (warn 300 / error 500)
        ├── test_*.py      ← tests get their own limit, wherever they live
        └── migrations/    ← generated migrations, excluded entirely
```

## Run it

```bash
cd examples/python-monorepo
linecheck .
```

`linecheck.yml` here keeps service modules small (handlers should delegate to
`libs/`), gives shared libraries more room, lets `test_*.py` files grow larger
than the code they cover, and excludes generated `migrations/` — all from one
file, with no per-package `pyproject.toml` or linter plugin required. Rules
are matched in order, so the `test_*.py` rule wins over the directory rules.
