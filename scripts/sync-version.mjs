// Reads the version from package.json and writes it back to every Cargo.toml,
// keeping them in sync after `changeset version` bumps package.json.
import { readFileSync, writeFileSync } from "fs";

const pkg = JSON.parse(readFileSync("package.json", "utf8"));
const version = pkg.version;

if (!version) {
  console.error("No version found in package.json");
  process.exit(1);
}

for (const path of ["Cargo.toml", "crates/wasm/Cargo.toml"]) {
  let cargo = readFileSync(path, "utf8");
  cargo = cargo.replace(/^version = ".*"/m, `version = "${version}"`);
  writeFileSync(path, cargo);
}

console.log(`Synced version ${version} to Cargo.toml files`);
