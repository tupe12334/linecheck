# linecheck

WASM build of [`linecheck`](https://github.com/tupe12334/linecheck) — enforce per-file line limits from Node.js.

```bash
npm install linecheck
```

```js
const { check } = require("linecheck");
const fs = require("fs");

const configYaml = fs.readFileSync("linecheck.yml", "utf8"); // optional
const result = check("src/main.rs", fs.readFileSync("src/main.rs", "utf8"), configYaml);

console.log(result); // { status: "ok" | "warn" | "error", lines, warn_limit, error_limit, message }
```

Omit the third argument (or pass `undefined`) to use the built-in 200/400 warn/error thresholds instead of a config file.

See the [main README](https://github.com/tupe12334/linecheck#readme) for the full rule/config format. A CLI (`pnpm dlx linecheck`) and Go bindings are planned as separate builds reusing the same Rust core.
