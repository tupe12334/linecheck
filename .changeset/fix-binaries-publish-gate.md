---
"linecheck": patch
---

Fix the prebuilt-binaries job never triggering: gate it on a missing GitHub release for the current version instead of changesets' `published` output, which stays false under this repo's custom cargo publish script.
