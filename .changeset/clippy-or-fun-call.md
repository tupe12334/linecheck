---
"linecheck": patch
---

Enable the `clippy::or_fun_call` lint at `deny` and replace eager `Option::or` calls in `main` with lazy `or_else`, so fallback values are only computed when they are actually needed.
