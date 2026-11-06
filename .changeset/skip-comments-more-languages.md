---
"linecheck": patch
---

Widen `--skip-comments` language coverage: `.php`, `.zig`, `.groovy`, `.gradle`, `.sol` and `.proto` join the `//` family; `.nix`, `.tf`, `.tfvars`, `.hcl`, `.jl`, `.cmake`, `.graphql`, `.gql`, `.ps1`, `.fish`, `.mk` and `.dockerfile` join the `#` family; and `--`, `%` and `;` comment styles are now recognized for Lua, SQL, Haskell, Elm, Ada, Erlang, LaTeX, Lisp, Clojure, Scheme and INI. Extensions whose comment syntax is ambiguous (`.m`) stay unrecognized. Unaffected without `--skip-comments`.
