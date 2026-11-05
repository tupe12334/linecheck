---
"linecheck": patch
---

Restore the declared minimum supported Rust version (1.85): a let-chain (stable only since 1.88) in exclude-pattern matching broke `cargo install linecheck` on Rust 1.85–1.87. CI now builds and tests on 1.85 so the `rust-version` promise can't regress.
