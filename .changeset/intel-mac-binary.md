---
"linecheck": patch
---

Ship the `x86_64-apple-darwin` (Intel Mac) prebuilt binary again: its release job was pinned to the retired `macos-13` runner and never ran, so v1.2.2 has no Intel Mac asset. It now cross-compiles on `macos-latest`.
