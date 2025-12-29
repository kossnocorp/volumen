# Parser Unification Plan

## Overview

Unify the parser implementations by removing slower alternatives and keeping the most performant parsers for each language.

## Current State

### TypeScript Parsers
- **`parser-ts` (Oxc)** - KEEP ⚡
  - Performance: ~0.22ms for 10k lines
  - Fastest TypeScript parser available
- **`parser-ts-tree-sitter` (Tree-sitter)** - REMOVE ❌
  - Performance: ~4.3ms for 10k lines
  - ~20x slower than Oxc
  - No longer needed

### Python Parsers
- **`parser-py` (RustPython)** - REMOVE ❌
  - Performance: ~2.6ms for 10k lines
  - Slower than both alternatives
  - Middle-ground performance not needed
- **`parser-py-ruff` (Ruff)** - KEEP ⚡
  - Performance: ~0.45ms for 10k lines
  - Fastest Python parser
  - Primary recommended parser
- **`parser-py-tree-sitter` (Tree-sitter)** - KEEP 🔧
  - Performance: ~3.8ms for 10k lines
  - Keep as fallback/compatibility option
  - Slower but more battle-tested

## Benefits

1. **Simplified Codebase**: Fewer parsers to maintain and test
2. **Better Performance**: Keeping the fastest parsers (Oxc for TS, Ruff primary for Python)
3. **Reduced Build Times**: Fewer dependencies to compile
4. **Clearer API**: Single recommended parser per language (with Tree-sitter fallback for Python)
5. **Smaller Binary Size**: Less code to bundle in WASM and native builds

## Implementation Tasks

### 1. Workspace Configuration

#### Remove from `Cargo.toml` workspace members:
```toml
# Remove these lines:
"pkgs/parser-ts-tree-sitter",
"pkgs/parser-py",
```

### 2. Update Package Dependencies

#### `pkgs/parser/Cargo.toml`
Remove the RustPython dependency:
```toml
# Remove this line:
volumen_parser_py = { version = "0.3.3", path = "../parser-py" }
```

#### `pkgs/parser-tests/Cargo.toml`
Remove both parser dependencies:
```toml
# Remove these lines:
volumen_parser_py = { version = "0.3.3", path = "../parser-py" }
volumen_parser_ts_tree_sitter = { version = "0.3.3", path = "../parser-ts-tree-sitter" }
```

#### `pkgs/benchmarks/Cargo.toml`
Remove both parser dependencies:
```toml
# Remove these lines:
volumen_parser_py = { version = "0.3.3", path = "../parser-py" }
volumen_parser_ts_tree_sitter = { version = "0.3.3", path = "../parser-ts-tree-sitter" }
```

### 3. Update Benchmark Files

#### `pkgs/benchmarks/benches/typescript_parsers.rs`
- Remove import: `use volumen_parser_ts_tree_sitter::ParserTs;`
- Remove all Tree-sitter benchmark blocks:
  - Lines 209-215 (small benchmark)
  - Lines 227-233 (medium benchmark)
  - Lines 247-253 (large benchmark)
- Simplify to only benchmark Oxc parser

#### `pkgs/benchmarks/benches/python_parsers.rs`
- Remove import: `use volumen_parser_py::ParserPy;`
- Remove all RustPython benchmark blocks:
  - Lines 162-168 (small benchmark)
  - Lines 189-195 (medium benchmark)
  - Lines 216-222 (large benchmark)
- Keep only Ruff and Tree-sitter benchmarks

### 4. Update Test Utilities

#### `pkgs/parser-tests/tests/utils.rs`

**Remove imports (lines 7, 12):**
```rust
// Remove these lines:
use volumen_parser_py::ParserPy as ParserPyRustPython;
use volumen_parser_ts_tree_sitter::ParserTs as ParserTsTreeSitter;
```

**Update `TS_PARSERS` array (lines 19-22):**
```rust
// Change from:
static TS_PARSERS: &Parsers = &[
    ("ParserTsOxc", ParserTsOxc::parse),
    ("ParserTsTreeSitter", ParserTsTreeSitter::parse),
];

// To:
static TS_PARSERS: &Parsers = &[
    ("ParserTsOxc", ParserTsOxc::parse),
];
```

**Update `PY_PARSERS` array (lines 24-28):**
```rust
// Change from:
static PY_PARSERS: &Parsers = &[
    ("ParserPyRustPython", ParserPyRustPython::parse),
    ("ParserPyRuff", ParserPyRuff::parse),
    ("ParserPyTreeSitter", ParserPyTreeSitter::parse),
];

// To:
static PY_PARSERS: &Parsers = &[
    ("ParserPyRuff", ParserPyRuff::parse),
    ("ParserPyTreeSitter", ParserPyTreeSitter::parse),
];
```

### 5. Update Token Count Data

#### `pkgs/benchmarks/token_counts.json`
Remove entries for removed parsers:
- Remove `"RustPython"` entries from `"python_parsers"`
- Remove `"Tree-sitter"` entries from `"typescript_parsers"`

### 6. Physical Directory Cleanup

Delete the following directories and all their contents:
- `pkgs/parser-ts-tree-sitter/`
- `pkgs/parser-py/`

### 7. Test Snapshot Updates

After removing parsers, regenerate test snapshots:
```bash
cd pkgs/parser-tests
cargo test -- --ignored
```

This will remove snapshots for the deleted parsers and ensure remaining tests pass.

### 8. Documentation Updates

- Update any README files that reference the removed parsers
- Update benchmark results documentation
- Update any architecture diagrams or docs showing parser options

## Validation Steps

### 1. Verify Compilation
```bash
cargo build --all
```

### 2. Run All Tests
```bash
cargo test --all
```

### 3. Run Benchmarks
```bash
cd pkgs/benchmarks
cargo bench
```

### 4. Verify WASM Build
```bash
cd pkgs/wasm
./scripts/build.sh
```

### 5. Check for Orphaned References
```bash
# Search for any remaining references to removed parsers
rg "parser-ts-tree-sitter|parser_ts_tree_sitter|ParserTsTreeSitter" --type rust
rg "parser-py[^-]|parser_py[^_]|ParserPyRustPython" --type rust
```

## Risks & Mitigations

### Risk: Test Coverage Loss
- **Mitigation**: Remaining parsers (Oxc, Ruff, Tree-sitter for Python) cover all test cases
- All existing tests will still run, just with fewer parser implementations

### Risk: User Dependencies
- **Mitigation**: Users depend on `volumen_parser::Parser` which remains unchanged
- Internal implementation details are not part of public API

### Risk: Missing Edge Cases
- **Mitigation**: Tree-sitter parser kept for Python as fallback
- Oxc and Ruff are production-ready, well-tested parsers

## Rollback Plan

If issues arise:
1. Revert the commit that removes the parsers
2. Or manually restore from git history:
   ```bash
   git checkout HEAD~1 -- pkgs/parser-ts-tree-sitter
   git checkout HEAD~1 -- pkgs/parser-py
   ```

## Success Criteria

- [ ] All tests pass with remaining parsers
- [ ] Benchmarks run successfully and show expected performance
- [ ] WASM build completes without errors
- [ ] No references to removed parsers remain in codebase
- [ ] Build times are reduced
- [ ] Documentation is updated

## Timeline Estimate

- Configuration updates: 15 minutes
- Code updates: 30 minutes
- Testing and validation: 30 minutes
- Documentation: 15 minutes
- **Total: ~1.5 hours**

## Notes

- This is a non-breaking change for external users
- The unified `volumen_parser::Parser` facade continues to work
- Parser selection happens internally based on file extension
- Python gets dual parser support (Ruff primary, Tree-sitter fallback)
- TypeScript uses only Oxc (fastest option)
