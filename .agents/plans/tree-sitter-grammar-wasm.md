# Tree-sitter grammar Wasm remediation plan

## Summary

- Wasm build fails on `wasm32-unknown-unknown` because grammar crates still expect host libc headers; `clang --target=wasm32-unknown-unknown` cannot find `stdlib.h` when compiling each `parser.c`/`scanner.c`.
- Upstream tree-sitter PR #4820 adds a mini Wasm sysroot and build.rs metadata via the `tree-sitter-language` crate, but the grammar versions in this repo (go/java/php/ruby/c-sharp/python/etc.) and `tree-sitter` 0.26.3 do not yet propagate those headers during wasm builds.
- Fix requires pinning grammars to commits that include the Wasm sysroot support (or patching them locally), and making Cargo use those patched sources during the Wasm build.

## Plan

1. Inventory grammars used by the Wasm build (go, java, php, ruby, c-sharp, python, typescript, others in `pkgs/parser-*`) and note current crate versions vs upstream commits that include the Wasm fix.
2. Add upstream grammar repos as git submodules under `./subs/tree-sitter-<lang>` at the chosen commit/tag that contains or will receive the Wasm sysroot changes.
3. Patch each grammar for Wasm: align `build.rs` with tree-sitter PR #4820 (detect wasm32, read `DEP_TREE_SITTER_LANGUAGE_WASM_*`, include the wasm headers, and compile the wasm `stdio/stdlib/string` sources), and adjust any `scanner.c` includes to avoid host-only headers.
4. Update the workspace to consume the patched grammars: add `patch.crates-io` entries pointing to `./subs/...`, bump parser crates to the matching grammar versions, and align `tree-sitter`/`tree-sitter-language` dependency versions to the release that carries the Wasm sysroot.
5. Re-run the Wasm build (`./scripts/build.sh` / `wasm-pack build --target nodejs`), verify no missing-header errors, and fix any language-specific fallout (e.g., extra includes or feature flags in scanners).
6. Document the outcome and, if needed, upstream the patches or pin to forks so future consumers inherit the Wasm-ready grammars and sysroot.
