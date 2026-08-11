---
"linecheck": patch
---

Fix stale `Cargo.lock`: the release version-sync script now re-locks workspace member versions, so `cargo build --locked` (used by the prebuilt-binary release workflow) no longer fails after a version bump.
