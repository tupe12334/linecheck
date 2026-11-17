---
"linecheck": patch
---

Warn on stderr when a `linecheck.yml` fails to parse. Previously the CLI dropped a malformed config silently and fell back to default limits, so a typo could quietly disable every custom rule while CI kept passing.
