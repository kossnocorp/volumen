# Heredoc and Multiline String Whitespace Stripping Plan

**Date**: 2025-12-28  
**Issue**: Parsers incorrectly handle whitespace in heredoc/multiline strings that have automatic indentation stripping

## Problem Statement

String definitions that remove minimal common indentation (like Ruby's `<<~` squiggly heredoc, PHP's flexible heredocs, and Java's text blocks) must create tokens for each line starting from **after the removed indentation** until (and including if present) `\n`.

Currently, the parsers include the leading whitespace in the token spans, which is incorrect for languages that strip this whitespace.

## Languages Affected

### 🔴 Ruby - CONFIRMED BUGS

**Affected Syntax**: `<<~TEXT` (squiggly heredoc)

**Current Behavior**:
```ruby
# @prompt
system = <<~TEXT
  You are a helpful assistant.
  You will answer the user's questions.
TEXT
```

Current `inner` span content:
```
"  You are a helpful assistant.\n  You will answer the user's questions.\n"
```

**Expected Behavior**:
```
"You are a helpful assistant.\nYou will answer the user's questions.\n"
```

**Ruby Heredoc Variants**:
- `<<~TEXT` - **Strips** minimal common leading whitespace ❌ BUG
- `<<TEXT` - Preserves all whitespace ✅ OK
- `<<'TEXT'` - Preserves all whitespace (no interpolation) ✅ OK
- `<<"TEXT"` - Preserves all whitespace (with interpolation) ✅ OK

**Algorithm for `<<~TEXT`**:
1. Find the minimum leading whitespace across all non-empty lines
2. Strip that amount of whitespace from the beginning of each line
3. Create token spans that start AFTER the stripped whitespace

### 🔴 PHP - MULTIPLE ISSUES

**Issue 1: Span Calculation**
- Current: `outer` and `inner` spans are identical
- Current: Both include the heredoc markers (`<<<TEXT`)
- Expected: `outer` should include markers, `inner` should be the body only

**Issue 2: Flexible Heredoc/Nowdoc (PHP 7.3+)**

PHP 7.3 introduced flexible heredoc syntax where the closing delimiter can be indented:

```php
<?php
// @prompt
$system = <<<TEXT
    You are a helpful assistant.
    You will answer questions.
    TEXT;
```

The indentation of the closing `TEXT` determines how much whitespace to strip from all lines. This behaves similarly to Ruby's `<<~`.

**Algorithm**:
1. Detect the indentation level of the closing delimiter
2. Strip that amount of leading whitespace from each line in the body
3. Create token spans starting after the stripped whitespace

### 🟡 Java - NEEDS INVESTIGATION

**Affected Syntax**: Text blocks (`"""..."""`) (Java 15+)

**Incidental Whitespace Stripping**:
Java text blocks automatically strip "incidental whitespace" based on the position of the closing `"""`:

```java
// @prompt
String system = """
    You are a helpful assistant.
    """;
```

**Algorithm** (per JEP 378):
1. Find the line with the closing `"""` 
2. Count its leading whitespace (this is the "incidental whitespace")
3. Strip that amount from the beginning of each line in the content
4. The opening `"""` line (if it contains only whitespace) is discarded
5. Trailing whitespace on the closing `"""` line is discarded

**Current Test Status**:
The test in `07_syntax_java.rs` shows:
```
"inner": "\"\"\n    You are a helpful assistant.\n    \"\""
```

This includes leading spaces that should likely be stripped. Needs verification against actual Java behavior.

## Technical Approach

### Token Span Creation Rules

For strings with automatic indentation stripping, we need to:

1. **Identify the stripping variant**: Detect `<<~` vs `<<`, flexible heredoc closing delimiter indentation, text block closing delimiter position
2. **Calculate stripped positions**: For each line:
   - Start position = original position + stripped whitespace count
   - End position = original end position (includes `\n` if present)
3. **Create separate tokens per line**: Each line gets its own token starting after stripped whitespace
4. **Handle interpolation**: Variables must be positioned relative to the stripped content

### Implementation Strategy

#### Ruby Parser (`pkgs/parser-rb/`)

**Files to modify**:
- `src/spans.rs`: Update `span_shape_string_like` function
- `src/lib.rs`: Update token creation in `build_content_tokens`

**Approach**:
1. Detect `<<~` heredoc variant in `span_shape_string_like`
2. Calculate minimum leading whitespace across all lines
3. When building content tokens, adjust spans to skip stripped whitespace
4. Handle interpolation positions relative to stripped content

#### PHP Parser (`pkgs/parser-php/`)

**Files to modify**:
- `src/spans.rs`: Fix span calculation for heredocs
- `src/lib.rs`: Add flexible heredoc detection and whitespace stripping

**Approach**:
1. Fix basic span calculation to separate outer (with markers) from inner (body only)
2. Detect closing delimiter indentation
3. Strip that amount of whitespace from each line
4. Adjust token spans accordingly

#### Java Parser (`pkgs/parser-java/`)

**Files to check**:
- `src/spans.rs`: Likely needs text block handling
- `src/lib.rs`: May need updates for incidental whitespace

**Approach**:
1. First, verify actual Java text block behavior with test program
2. Implement incidental whitespace detection
3. Strip whitespace based on closing delimiter position
4. Create appropriate token spans

## Testing Plan

### Phase 1: Verification
1. Create test programs in Ruby, PHP, and Java that output actual string content
2. Verify the whitespace stripping behavior for each variant
3. Document byte positions for test cases

### Phase 2: Implementation
1. Implement Ruby `<<~` fix
2. Run Ruby tests, update snapshots
3. Implement PHP heredoc fixes
4. Run PHP tests, update snapshots
5. Implement Java text block fixes (if needed)
6. Run Java tests, update snapshots

### Phase 3: Documentation
1. Update `matrix.md` with findings
2. Add whitespace handling column or notation
3. Mark tests with appropriate bug indicators

## Test Cases to Add/Verify

### Ruby
- [ ] `<<~TEXT` with 2-space indentation
- [ ] `<<~TEXT` with 4-space indentation  
- [ ] `<<~TEXT` with tab indentation
- [ ] `<<~TEXT` with mixed content (some lines more indented)
- [ ] `<<~TEXT` with interpolation `#{var}`
- [ ] `<<TEXT` (verify no stripping)

### PHP
- [ ] `<<<TEXT` with indented closing delimiter (PHP 7.3+)
- [ ] `<<<TEXT` with non-indented closing (traditional)
- [ ] `<<<'TEXT'` nowdoc with indented closing
- [ ] Heredoc with interpolation

### Java  
- [ ] Text block with indented closing `"""`
- [ ] Text block with closing at column 0
- [ ] Text block with content more indented than closing
- [ ] Empty text block

## Success Criteria

1. ✅ Ruby `<<~` heredoc creates tokens starting after stripped whitespace
2. ✅ PHP flexible heredoc correctly strips whitespace based on closing delimiter
3. ✅ Java text blocks strip incidental whitespace (if applicable)
4. ✅ All test snapshots updated and passing
5. ✅ `matrix.md` updated with whitespace handling status
6. ✅ No regressions in other language parsers

## References

- Ruby squiggly heredoc: https://ruby-doc.org/core-3.1.0/doc/syntax/literals_rdoc.html#label-Here+Documents
- PHP flexible heredoc: https://www.php.net/manual/en/language.types.string.php#language.types.string.syntax.heredoc
- Java text blocks (JEP 378): https://openjdk.org/jeps/378
