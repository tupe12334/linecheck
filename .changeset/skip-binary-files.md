---
"linecheck": patch
---

Skip binary files instead of counting their raw newline bytes as lines. Detected via the `content_inspector` crate; previously an unmatched binary file (e.g. an image) fell back to the default threshold and could false-positive error (#14).
