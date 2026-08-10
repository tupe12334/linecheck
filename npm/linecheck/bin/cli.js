#!/usr/bin/env node
// Runs the real `linecheck` CLI (compiled to wasm32-wasip1) under Node's
// built-in WASI runtime, so `npx linecheck` / `pnpm dlx linecheck` behave
// identically to the native binary — same args, output, and exit code.
"use strict";
const { readFileSync } = require("fs");
const { join } = require("path");
const { WASI } = require("node:wasi");

const wasi = new WASI({
  version: "preview1",
  args: ["linecheck", ...process.argv.slice(2)],
  env: process.env,
  preopens: { ".": process.cwd() },
});

const wasmBuffer = readFileSync(join(__dirname, "..", "linecheck-cli.wasm"));
WebAssembly.instantiate(wasmBuffer, wasi.getImportObject()).then(({ instance }) => {
  process.exit(wasi.start(instance));
});
