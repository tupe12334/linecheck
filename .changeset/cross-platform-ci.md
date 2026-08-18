---
"linecheck": patch
---

Fix exclude patterns being ignored on Windows: paths are now normalized to `/` before glob matching. CI also builds and tests on macOS and Windows, matching the platforms release binaries are published for.
