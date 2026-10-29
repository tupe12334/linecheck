---
"linecheck": minor
---

Directory walks now honor `.gitignore`/`.ignore` files (via the `ignore` crate, ripgrep's walker), so build output like `target/` or `node_modules/` no longer has to be listed under `exclude`.
