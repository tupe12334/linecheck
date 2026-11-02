---
"linecheck": patch
---

Enforce `cargo fmt --all --check` in CI. Formatting was only verified by the opt-in `.githooks/pre-push` hook, so unformatted code could merge from any contributor who had not run `git config core.hooksPath .githooks`.
