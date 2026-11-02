---
"linecheck": patch
---

Enable the `clippy::match_wildcard_for_single_variants` lint and match `Status::Ok` explicitly in the table renderer, so a future `Status` variant becomes a compile error instead of falling silently into a wildcard arm.
