---
"linecheck": minor
---

Add WASM bindings published to npm as `linecheck`, so JS/Node projects can check in-memory file content without a filesystem. Adds `check_content` to the core Rust library (bindings entry point with no fs access) as the foundation for planned CLI and Go bindings.
