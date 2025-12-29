# Tests Matrix

## Feature-Language Matrix

| Test Lang:                        | `ts` | `py` | `rb` | `php` | `go` | `cs` | `java` |
| --------------------------------- | ---- | ---- | ---- | ----- | ---- | ---- | ------ |
| `01_var_name_{{lang}}.rs`         | +++  | +++  | +++  | +++   | ++   | ++   | +++    |
| `02_var_comment_{{lang}}.rs`      | +++  | +++  | +++  | +++   | ++   | ++   | ++     |
| `03_inline_comment_{{lang}}.rs`   | +++  | +++  | +++  | +++   | ++   | ++   | +++    |
| `04_prompt_vars_{{lang}}.rs`      | +++  | +++  | +++  | +++   | ++   | ++   | ++     |
| `05_multiple_prompts_{{lang}}.rs` | +++  | +++  | +++  | ++    | ++   | ++   | +++    |
| `06_annotations_{{lang}}.rs`      | +++  | ++   | +++  | ++!   | ++!  | ++!  | ++!    |
| `07_syntax_{{lang}}.rs`           | +++  | +++  | +++  | +++   | ++!  | +++  | +++    |
| `08_concat_{{lang}}.rs`           | +++  | +++  | +++  | +++   | +++  | +++  | +++    |
| `09_fn_{{lang}}.rs`               | +++  | +++  | +++  | +++   | +++  | +++  | +++    |
| `10_array_{{lang}}.rs`            | +++  | +++  | +++  | +++   | +++  | +++  | +++    |

**Legend**:

- `-`: Test file does not exist
- `+`: Test file exists
- `++`: All tests exist (but snapshots are incorrect or empty `@""`)
- `+++`: All test snapshots are correct, or the tests are not applicable (i.e., `03_inline_comment_py.rs` for `py`)

**Languages**:

- `ts`: TypeScript
- `py`: Python
- `rb`: Ruby
- `php`: PHP
- `go`: Go
- `cs`: C#
- `java`: Java

## Test-Language Matrix

**Legend**:

- `-`: Test absent
- `~`: Test skipped due to language limitations
- `+`: Test fn present
- `++`: Test source code present and correct
- `+++`: Test snapshot correct

Additional legend:

- `!` (added to one of the symbols above, e.g., `+++!`): There's a bug in the test, either in the test source code or the snapshot
- `🔸` (added to status with `!`, e.g., `+++!🔸`): Whitespace stripping bug in heredoc/multiline strings - token spans include whitespace that should be stripped

### `01_var_name_{{lang}}.rs`

| Test Lang: | `ts` | `py` | `rb` | `php` | `go` | `cs` | `java` |
| ---------- | ---- | ---- | ---- | ----- | ---- | ---- | ------ |
| `simple`   | +++  | +++  | +++  | +++   | ++   | ++   | +++    |
| `nested`   | +++  | +++  | +++  | +++   | ++   | ++   | +++    |

### `02_var_comment_{{lang}}.rs`

| Test Lang:              | `ts` | `py` | `rb` | `php` | `go` | `cs` | `java` |
| ----------------------- | ---- | ---- | ---- | ----- | ---- | ---- | ------ |
| `simple`                | +++  | +++  | ++   | +++   | ++   | +++  | ++     |
| `inline`                | +++  | -    | -    | ++    | ++   | ++   | ++     |
| `doc`                   | +++  | -    | -    | ++    | -    | ++   | ++     |
| `assigned`              | +++  | +++  | +++  | +++   | ++   | ++   | ++     |
| `assigned_late_comment` | +++  | +++  | +++  | +++   | ++   | ++   | ++     |
| `reassigned`            | +++  | +++  | +++  | +++   | ++   | ++   | ++     |
| `inexact`               | +++  | +++  | ++   | +++!  | ++   | ++   | ++     |
| `mixed`                 | +++  | +++  | +++! | ++    | ++   | ++   | ++     |
| `mixed_nested`          | +++  | +++  | ++   | ++    | ++   | ++   | ++     |
| `mixed_none`            | +++  | +++  | ++   | ++    | ++   | ++   | ++     |
| `mixed_assign`          | +++  | +++  | ++   | ++    | ++   | ++   | ++     |
| `mixed_reassign`        | +++! | +++! | +++  | +++   | -    | -    | -      |
| `mixed_reassign_inline` | +++! | -    | -    | -     | -    | -    | -      |
| `spaced`                | +++  | +++  | +++! | ++    | ++   | ++   | ++     |
| `dirty`                 | +++  | +++  | ++   | ++    | ++   | ++   | ++     |
| `multi`                 | +++  | +++  | +++  | +++   | ++   | ++   | ++     |
| `destructuring`         | +++  | +++  | +++  | +++   | -    | -    | -      |
| `chained`               | +++  | +++  | +++! | +++!  | -    | -    | -      |

**Known Issues**:

- `ts` `mixed_reassign` and `mixed_reassign_inline`: After removing the `exp` field from `PromptAnnotation`, the parsers need to be updated to collect ALL adjacent comments (not just `@prompt` ones) for reassignments and inline prompt cases. Currently ignored.
- `py` `mixed_reassign`: Same issue as TypeScript - needs parser refactoring to properly collect line comments for reassignments.

### `03_inline_comment_{{lang}}.rs`

| Test Lang: | `ts` | `py` | `rb` | `php` | `go` | `cs` | `java` |
| ---------- | ---- | ---- | ---- | ----- | ---- | ---- | ------ |
| `simple`   | +++  | ~    | ~    | +++   | +++! | +++! | +++    |
| `doc`      | +++  | ~    | ~    | +++   | -    | +++! | +++    |
| `inexact`  | +++  | ~    | ~    | +++   | +++  | +++  | +++    |
| `dirty`    | +++  | ~    | ~    | +++   | +++! | +++! | +++    |

### `04_prompt_vars_{{lang}}.rs`

| Test Lang:      | `ts` | `py` | `rb` | `php` | `go` | `cs` | `java` |
| --------------- | ---- | ---- | ---- | ----- | ---- | ---- | ------ |
| `single_var`    | +++  | +++  | +++  | +++   | ~    | +++  | ~      |
| `multiple_vars` | +++  | +++  | +++  | +++   | ~    | +++  | ~      |
| `exp`           | +++  | +++  | +++  | +++!  | ~    | +++  | ~      |
| `exp_complex`   | +++  | +++  | +++  | +++!  | ~    | +++  | ~      |

### `05_multiple_prompts_{{lang}}.rs`

| Test Lang: | `ts` | `py` | `rb` | `php` | `go` | `cs` | `java` |
| ---------- | ---- | ---- | ---- | ----- | ---- | ---- | ------ |
| `multiple` | +++  | +++  | +++  | ++    | ++!  | ++   | ++!    |

- `go`: parser returns no prompts even with annotated strings.
- `java`: prompts captured but no vars because inputs lack interpolation.
- `php`/`cs`: snapshots are empty or pending.

### `06_annotations_{{lang}}.rs`

| Test Lang:         | `ts` | `py` | `rb` | `php` | `go` | `cs` | `java` |
| ------------------ | ---- | ---- | ---- | ----- | ---- | ---- | ------ |
| `multiple`         | +++! | -    | -    | ++!   | ++!  | ++!  | ++!    |
| `multiline`        | +++! | +++  | +++  | ++!   | ++!  | ++!  | ++!    |
| `multiline_nested` | +++  | +++  | +++  | +++   | ++!  | +++  | +++    |

- `py` and `rb` omit the `multiple` test case.
- `php` and `java` capture only `/* @prompt */` for `multiple` and `multiline`, missing preceding comment or block annotations seen in `ts`.
- `go` snapshots are empty for all tests; no prompt data captured.
- `cs` snapshots are empty for `multiple` and `multiline`.

**Known Issues**:

- `ts` `multiple` and `multiline`: After removing the `exp` field from `PromptAnnotation`, when there's an inline `/* @prompt */` comment, the parser should also collect ALL adjacent leading comments (even those without `@prompt` like `// Hello, world` or block comments). This is needed for future extensibility (e.g., `@eval` directives). Currently ignored pending parser refactoring.

### `07_syntax_{{lang}}.rs`

| Test Lang:               | `ts` | `py` | `rb` | `php` | `go` | `cs` | `java` |
| ------------------------ | ---- | ---- | ---- | ----- | ---- | ---- | ------ |
| `invalid`                | +++  | +++  | +++  | +++   | +++  | +++  | +++    |
| `jsx`                    | +++  | ~    | ~    | ~     | ~    | ~    | ~      |
| `typed_template`         | +++  | ~    | ~    | ~     | ~    | ~    | ~      |
| `tsx_template`           | +++  | ~    | ~    | ~     | ~    | ~    | ~      |
| `multiline_plain`        | -    | +++  | +++  | +++   | ++!  | +++  | +++    |
| `multiline_interpolated` | +++  | +++  | +++  | +++   | -    | +++  | -      |

**Known Issues**:

- `rb`: All heredoc tests now correctly handle whitespace stripping for squiggly heredocs (`<<~TEXT`) ✅
- `php`: Heredoc span calculation fixed - `outer`/`inner` now correctly separated, markers excluded from inner ✅
- `java`: Text blocks now correctly strip incidental whitespace based on minimum indentation ✅
- `go`: Snapshots are empty (`@""`) and only cover a plain raw string; there is no interpolated variant.
- `cs`: Matches TS/PY for verbatim and interpolated strings with annotations - no whitespace stripping issues (verbatim strings preserve all whitespace by design).
- `ts`: Template literals preserve all whitespace (correct behavior) - no issues.
- `py`: Triple-quoted strings preserve all whitespace (correct behavior) - no issues.

### `08_concat_{{lang}}.rs`

| Test Lang:                   | `ts` | `py` | `rb` | `php` | `go` | `cs` | `java` |
| ---------------------------- | ---- | ---- | ---- | ----- | ---- | ---- | ------ |
| `concat`                     | +++  | +++  | +++  | +++   | +++  | +++  | +++    |
| `concat_with_primitives`     | +++  | -    | -    | +++   | -    | +++  | +++    |
| `concat_with_function_calls` | +++  | +++  | +++  | +++   | +++  | +++  | +++    |
| `concat_with_objects`        | +++  | +++  | +++  | +++   | +++  | +++  | +++    |

### `09_fn_{{lang}}.rs`

| Test Lang:  | `ts` | `py` | `rb` | `php` | `go` | `cs` | `java` |
| ----------- | ---- | ---- | ---- | ----- | ---- | ---- | ------ |
| `format_fn` | ~    | +++  | +++  | +++   | +++  | +++  | +++    |

**Implementation Details**:

- `ts`: No common `format` helper is available; test is skipped (~)
- `py`: Detects `.format()` method calls with Python-style placeholders (`{}`, `{0}`, `{name}`)
- `rb`: Detects `%` operator with printf-style placeholders (`%s`, `%d`, `%v`)
- `php`: Detects `sprintf()`/`printf()` functions with printf-style placeholders
- `go`: Detects `fmt.Sprintf()`/`fmt.Printf()` with Go-style placeholders (`%s`, `%d`, `%v`)
- `cs`: Detects `String.Format()` with C#-style numbered placeholders (`{0}`, `{1}`)
- `java`: Detects `String.format()` with printf-style placeholders

All implementations correctly parse format strings, extract placeholders, map arguments to variables with proper `index` field values, and generate appropriate `PromptContentToken` arrays. Snapshots have been generated and accepted for all languages.

### `10_array_{{lang}}.rs`

| Test Lang:     | `ts` | `py` | `rb` | `php` | `go` | `cs` | `java` |
| -------------- | ---- | ---- | ---- | ----- | ---- | ---- | ------ |
| `join_method`  | +++  | +++  | +++  | +++   | +++  | +++  | +++    |
| `array_simple` | +++  | +++  | +++  | +++   | +++  | +++  | +++    |
