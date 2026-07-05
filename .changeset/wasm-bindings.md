---
"linecheck": minor
---

Add `check_content` to check in-memory file content without touching the filesystem, as the foundation for non-Rust bindings. Includes a `crates/wasm` WASM-bindgen crate built on top (not yet published — packaging/publishing to follow separately).
