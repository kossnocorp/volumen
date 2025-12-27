# TODO: Fix Annotation Collection for Reassignments and Inline Prompts

## Problem Summary

After removing the `exp` field from `PromptAnnotation` and `PromptVar` structures, several tests are failing because the parsers are not correctly collecting annotations for:

1. **Reassignments with non-`@prompt` line comments**
2. **Inline `@prompt` comments with non-`@prompt` leading comments**

## Affected Tests

### Python (`02_var_comment_py.rs`)
- `mixed_reassign` - Currently ignored

### TypeScript (`02_var_comment_ts.rs`)
- `mixed_reassign` - Currently ignored
- `mixed_reassign_inline` - Currently ignored

### TypeScript Annotations (`06_annotations_ts.rs`)
- `multiple` - Currently ignored
- `multiline` - Currently ignored

## Root Cause

The parsers currently filter out comment blocks that don't contain any `@prompt` annotations. This was done in the `collect_adjacent_leading_comments()` method with a check like:

```rust
// Check if any comment in the block contains @prompt
let has_prompt = block_ranges
    .iter()
    .any(|c| parse_annotation(&c.text).unwrap_or(false));
if !has_prompt {
    return Vec::new();  // ❌ This filters out all non-@prompt comments
}
```

However, the correct behavior should be:

## Required Behavior

### Variable Comments vs Line Comments

There are two types of comments:

1. **Variable Comments**: Adjacent to variable definitions, used to identify prompt variables
   ```python
   # Yeah           ← Variable comment
   # @prompt def    ← Variable comment (marks as prompt)
   hello: Union[str | int] = 123   ← Definition
   ```

2. **Line Comments**: Adjacent to the current statement/reassignment
   ```python
   # @prompting     ← Line comment (even though not @prompt, should be included)
   hello = "Hi"    ← Reassignment
   ```

### Annotation Collection Rules

1. **For variable definitions** (first assignment with type annotation):
   - Collect only comments that contain `@prompt` to identify new prompt variables
   - Store these as "definition annotations"

2. **For reassignments** of known prompt variables:
   - Collect ALL adjacent line comments (including non-`@prompt` ones)
   - Merge stored definition annotations with line comments
   - This allows future extensibility (e.g., `@eval` directives)

3. **For inline `@prompt` comments**:
   - When there's an inline `/* @prompt */` or `# @prompt`
   - Also collect ALL adjacent leading comments (even `// Hello, world`)
   - Merge them into annotations

4. **Enclosure calculation**:
   - Should include ALL adjacent comments, not just `@prompt` ones
   - Use `get_any_leading_start()` instead of `get_leading_start()`

### Annotation Merging

Annotations should be merged into a **single `PromptAnnotation`** as long as they are contiguous (only whitespace/linebreaks between them). Each line should have a separate span:

```rust
PromptAnnotation {
    spans: vec![
        SpanShape { outer: (0, 13), inner: (1, 13) },    // # @prompt def
        SpanShape { outer: (56, 68), inner: (57, 68) },  // # @prompting
    ]
}
```

## Implementation Plan

### Phase 1: Add `collect_all_adjacent_leading()` Methods

Add a new method to each parser's comment collection that returns ALL adjacent comments without filtering:

**Files to modify:**
- `pkgs/parser-py-ruff/src/lib.rs`
- `pkgs/parser-py/src/lib.rs`
- `pkgs/parser-py-tree-sitter/src/comments.rs`
- `pkgs/parser-ts-tree-sitter/src/comments.rs` (already has this)

**Implementation**: Clone `collect_adjacent_leading()` and remove the `has_prompt` validation check.

### Phase 2: Update Parser Logic

Update each parser to:

1. Keep using `collect_adjacent_leading()` for identifying NEW prompt variables
2. Use `collect_all_adjacent_leading()` for:
   - Reassignments of known prompt variables
   - When there's an inline `@prompt` comment

**Key decision point**: Need to know whether we're processing:
- A definition (has type annotation + has @prompt) → use `collect_adjacent_leading()`
- A reassignment (no type annotation, variable already a prompt) → use `collect_all_adjacent_leading()`

### Phase 3: Fix Enclosure Calculation

Change enclosure start calculation from `get_leading_start()` to `get_any_leading_start()` to include all adjacent comments.

**Files to modify:**
- All parser files where enclosure is calculated

### Phase 4: Update Tests

Remove `#[ignore]` annotations from the tests and verify they pass.

## Architecture Considerations

### Python Parsers

**RustPython** (`parser-py`): Uses a visitor pattern with annotation stacks. May need refactoring to:
- Track both "prompt-only" and "all comments" separately
- Determine at `visit_stmt_assign` time whether to use stored def annotations + line comments

**Ruff** (`parser-py-ruff`): Similar architecture, same refactoring needed

**Tree-sitter** (`parser-py-tree-sitter`): More straightforward, can check `scopes.get_def_annotation()` to determine if variable is already a prompt

### TypeScript Parsers

**Tree-sitter** (`parser-ts-tree-sitter`): Already has `collect_all_adjacent_leading()` method. Main issue is determining when to use it.

**Oxc** (`parser-ts`): Needs similar logic as tree-sitter

## Test Cases to Verify

### Python `mixed_reassign`
```python
# @prompt def               # 0-13 (variable comment)
hello: Union[str | int] = 123
hello = 456                  # (ignored - not a prompt)
# @prompting                # 56-68 (line comment)
hello = "Hi"                 # 69-81
```

Expected:
- `enclosure: (56, 81)` - includes line comment
- `annotations`: Single `PromptAnnotation` with two spans:
  1. `(0, 13)` - definition comment
  2. `(56, 68)` - line comment

### TypeScript `multiple`
```typescript
// Hello, world              # 0-15 (line comment, not @prompt)
const hello = /* @prompt */ "asd";  # 16-50
```

Expected:
- `enclosure: (0, 50)` - includes line comment
- `annotations`: Single `PromptAnnotation` with two spans:
  1. `(0, 15)` - leading comment
  2. `(30, 43)` - inline @prompt

## References

- Issue discussion: This session
- Related commit: 72b04be (Remove exp from data structures)
- Test matrix: `pkgs/parser-tests/docs/matrix.md`
