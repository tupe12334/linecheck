---
"linecheck": patch
---

Publish prebuilt binaries: each release now creates a `vX.Y.Z` git tag and a GitHub release with prebuilt `linecheck` binaries for Linux (x86_64, aarch64), macOS (x86_64, aarch64), and Windows (x86_64), so non-Rust CI can install without a toolchain.
