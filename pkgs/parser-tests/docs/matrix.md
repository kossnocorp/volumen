# Tests Matrix

## Feature-Language Matrix

| Test Lang:                        | `ts` | `py` | `ruby` | `php` | `go` | `cs` | `java` |
| --------------------------------- | ---- | ---- | ------ | ----- | ---- | ---- | ------ |
| `01_var_name_{{lang}}.rs`         | +++  | +++  | +++    | +++   | ++   | ++   | +++    |
| `02_var_comment_{{lang}}.rs`      | +++  | +++  | +++    | +++   | ++   | ++   | ++     |
| `03_inline_comment_{{lang}}.rs`   | +++  | +++  | +++    | +++   | ++   | ++   | +++    |
| `04_prompt_vars_{{lang}}.rs`      | +++  | +++  | +++    | +++   | ++   | ++   | ++     |
| `05_multiple_prompts_{{lang}}.rs` | +++  | +++  | ++     | ++    | ++   | ++   | +++    |
| `06_annotations_{{lang}}.rs`      | +++  | ++   | ++     | ++!   | ++!  | ++!  | ++!    |
| `07_syntax_{{lang}}.rs`           | +++  | +++  | ++!    | +++   | ++!  | +++  | ++     |
| `08_concat_{{lang}}.rs`           | ++!  | ++!  | ++!    | ++!   | ++!  | ++!  | ++!    |
| `09_fn_{{lang}}.rs`               | +++  | ++!  | ++!    | ++!   | ++!  | ++!  | ++!    |
| `10_array_{{lang}}.rs`            | ++!  | ++!  | ++!    | ++!   | ++!  | ++!  | ++!    |

**Legend**:

- `-`: Test file does not exist
- `+`: Test file exists
- `++`: All tests exist (but snapshots are incorrect or empty `@""`)
- `+++`: All test snapshots are correct, or the tests are not applicable (i.e., `03_inline_comment_py.rs` for `py`)

**Languages**:

- `ts`: TypeScript
- `py`: Python
- `ruby`: Ruby
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

### `01_var_name_{{lang}}.rs`

| Test Lang: | `ts` | `py` | `ruby` | `php` | `go` | `cs` | `java` |
| ---------- | ---- | ---- | ------ | ----- | ---- | ---- | ------ |
| `simple`   | +++  | +++  | +++    | +++   | ++   | ++   | +++    |
| `nested`   | +++  | +++  | +++    | +++   | ++   | ++   | +++    |

### `02_var_comment_{{lang}}.rs`

| Test Lang:              | `ts` | `py` | `ruby` | `php` | `go` | `cs` | `java` |
| ----------------------- | ---- | ---- | ------ | ----- | ---- | ---- | ------ |
| `simple`                | +++  | +++  | ++     | +++   | ++   | +++  | ++     |
| `inline`                | +++  | -    | -      | ++    | ++   | ++   | ++     |
| `doc`                   | +++  | -    | -      | ++    | -    | ++   | ++     |
| `assigned`              | +++  | +++  | +++    | +++   | ++   | ++   | ++     |
| `assigned_late_comment` | +++  | +++  | +++    | +++   | ++   | ++   | ++     |
| `reassigned`            | +++  | +++  | +++    | +++   | ++   | ++   | ++     |
| `inexact`               | +++  | +++  | ++     | +++!  | ++   | ++   | ++     |
| `mixed`                 | +++  | +++  | +++!   | ++    | ++   | ++   | ++     |
| `mixed_nested`          | +++  | +++  | ++     | ++    | ++   | ++   | ++     |
| `mixed_none`            | +++  | +++  | ++     | ++    | ++   | ++   | ++     |
| `mixed_assign`          | +++  | +++  | ++     | ++    | ++   | ++   | ++     |
| `mixed_reassign`        | +++  | +++  | +++    | +++   | -    | -    | -      |
| `mixed_reassign_inline` | +++  | -    | -      | -     | -    | -    | -      |
| `spaced`                | +++  | +++  | +++!   | ++    | ++   | ++   | ++     |
| `dirty`                 | +++  | +++  | ++     | ++    | ++   | ++   | ++     |
| `multi`                 | +++  | +++  | +++    | +++   | ++   | ++   | ++     |
| `destructuring`         | +++  | +++  | +++    | +++   | -    | -    | -      |
| `chained`               | +++  | +++  | +++!   | +++!  | -    | -    | -      |

### `03_inline_comment_{{lang}}.rs`

| Test Lang: | `ts` | `py` | `ruby` | `php` | `go` | `cs` | `java` |
| ---------- | ---- | ---- | ------ | ----- | ---- | ---- | ------ |
| `simple`   | +++  | ~    | ~      | +++   | +++! | +++! | +++    |
| `doc`      | +++  | ~    | ~      | +++   | -    | +++! | +++    |
| `inexact`  | +++  | ~    | ~      | +++   | +++  | +++  | +++    |
| `dirty`    | +++  | ~    | ~      | +++   | +++! | +++! | +++    |

### `04_prompt_vars_{{lang}}.rs`

| Test Lang:      | `ts` | `py` | `ruby` | `php` | `go` | `cs` | `java` |
| --------------- | ---- | ---- | ------ | ----- | ---- | ---- | ------ |
| `single_var`    | +++  | +++  | +++    | +++   | ~    | +++  | ~      |
| `multiple_vars` | +++  | +++  | +++    | +++   | ~    | +++  | ~      |
| `exp`           | +++  | +++  | +++    | +++!  | ~    | +++  | ~      |
| `exp_complex`   | +++  | +++  | +++    | +++!  | ~    | +++  | ~      |

### `05_multiple_prompts_{{lang}}.rs`

| Test Lang: | `ts` | `py` | `ruby` | `php` | `go` | `cs` | `java` |
| ---------- | ---- | ---- | ------ | ----- | ---- | ---- | ------ |
| `multiple` | +++  | +++  | ++     | ++    | ++!  | ++   | ++!    |

- `go`: parser returns no prompts even with annotated strings.
- `java`: prompts captured but no vars because inputs lack interpolation.
- `ruby`/`php`/`cs`: snapshots are empty or pending.

### `06_annotations_{{lang}}.rs`

| Test Lang:         | `ts` | `py` | `ruby` | `php` | `go` | `cs` | `java` |
| ------------------ | ---- | ---- | ------ | ----- | ---- | ---- | ------ |
| `multiple`         | +++  | -    | -      | ++!   | ++!  | ++!  | ++!    |
| `multiline`        | +++  | +++  | +++    | ++!   | ++!  | ++!  | ++!    |
| `multiline_nested` | +++  | +++  | +++    | +++   | ++!  | +++  | +++    |

- `py` and `ruby` omit the `multiple` test case.
- `php` and `java` capture only `/* @prompt */` for `multiple` and `multiline`, missing preceding comment or block annotations seen in `ts`.
- `go` snapshots are empty for all tests; no prompt data captured.
- `cs` snapshots are empty for `multiple` and `multiline`.

### `07_syntax_{{lang}}.rs`

| Test Lang:               | `ts` | `py` | `ruby` | `php` | `go` | `cs` | `java` |
| ------------------------ | ---- | ---- | ------ | ----- | ---- | ---- | ------ |
| `invalid`                | +++  | +++  | +++    | +++   | +++  | +++  | +++    |
| `jsx`                    | +++  | ~    | ~      | ~     | ~    | ~    | ~      |
| `typed_template`         | +++  | ~    | ~      | ~     | ~    | ~    | ~      |
| `tsx_template`           | +++  | ~    | ~      | ~     | ~    | ~    | ~      |
| `multiline_plain`        | -    | +++  | ++!    | +++   | ++!  | +++  | +++    |
| `multiline_interpolated` | +++  | +++  | ++!    | +++   | -    | +++  | -      |

- `ruby` heredoc tests only capture the `<<~TEXT` marker, dropping the body and any interpolated vars.
- `php` matches the TS/PY baselines, including annotations and interpolated vars.
- `go` snapshots are empty (`@""`) and only cover a plain raw string; there is no interpolated variant.
- `cs` matches TS/PY for verbatim and interpolated strings with annotations.
- `java` covers only the plain text block; there is no interpolated variant.
- `ts` lacks a plain multiline-without-vars case; JSX/typed rows are TS-only (~ for other languages).

### `08_concat_{{lang}}.rs`

| Test Lang: | `ts` | `py` | `ruby` | `php` | `go` | `cs` | `java` |
| ---------- | ---- | ---- | ------ | ----- | ---- | ---- | ------ |
| `concat`   | ++!  | ++!  | ++!    | ++!   | ++!  | ++!  | ++!    |

- All languages: snapshots are empty (`@""`); prompts, interpolations, and annotations are not captured.
- `py`: asserts parse success but still records no prompt data.
- `php`: uses `g$greeting` and captures no prompts; other languages similarly miss prompt data.

### `09_fn_{{lang}}.rs`

| Test Lang:  | `ts` | `py` | `ruby` | `php` | `go` | `cs` | `java` |
| ----------- | ---- | ---- | ------ | ----- | ---- | ---- | ------ |
| `format_fn` | ~    | ++!  | ++!    | ++!   | ++!  | ++!  | ++!    |

- `ts`: no common `format` helper is available; test is skipped.
- `py`/`ruby`/`php`/`go`/`cs`/`java`: test sources exist but all snapshots are empty (`@""`); prompts, interpolations, and annotations are not captured.

### `10_array_{{lang}}.rs`

| Test Lang:     | `ts` | `py` | `ruby` | `php` | `go` | `cs` | `java` |
| -------------- | ---- | ---- | ------ | ----- | ---- | ---- | ------ |
| `join_method`  | ++!  | ++!  | ++!    | ++!   | ++!  | ++!  | ++!    |
| `array_simple` | ++!  | ++!  | ++!    | ++!   | ++!  | ++!  | ++!    |

- All languages: snapshots are empty (`@""`); prompts/interpolations/annotations are not captured for join or array cases.
- `ts`/`py` baselines also miss prompt data, so other languages inherit the gap.
