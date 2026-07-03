// Reads the version from package.json and writes it back to Cargo.toml,
// keeping the two in sync after `changeset version` bumps package.json.
import { readFileSync, writeFileSync } from "fs";

const pkg = JSON.parse(readFileSync("package.json", "utf8"));
const version = pkg.version;

if (!version) {
  console.error("No version found in package.json");
  process.exit(1);
}

let cargo = readFileSync("Cargo.toml", "utf8");
cargo = cargo.replace(/^version = ".*"/m, `version = "${version}"`);
writeFileSync("Cargo.toml", cargo);

console.log(`Synced version ${version} to Cargo.toml`);
