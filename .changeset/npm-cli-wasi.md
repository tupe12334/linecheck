---
"linecheck": minor
---

Publish `crates/wasm` to npm as `linecheck`. `pnpm dlx linecheck` / `npx linecheck` run the real CLI binary (built for `wasm32-wasip1`) under Node's WASI runtime, and `require("linecheck").check(...)` checks in-memory file content without a filesystem.
