# Tests Matrix

## Feature-Language Matrix

| Test Lang:                        | `ts` | `py` | `ruby` | `php` | `go` | `cs` | `java` |
| --------------------------------- | ---- | ---- | ------ | ----- | ---- | ---- | ------ |
| `01_var_name_{{lang}}.rs`         | +++  | +++  | +++    | +++   | ++   | ++   | +++    |
| `02_var_comment_{{lang}}.rs`      | +++  | +++  | +++    | +++   | ++   | ++   | ++     |
| `03_inline_comment_{{lang}}.rs`   | +++  | +++  | +++    | +++   | ++   | ++   | +++    |
| `04_prompt_vars_{{lang}}.rs`      | +++  | +++  | +++    | +++   | ++   | ++   | ++     |
| `05_multiple_prompts_{{lang}}.rs` | +++  | +++  | ++     | ++    | ++   | ++   | +++    |
| `06_annotations_{{lang}}.rs`      | +++  | +++  | +++    | +++   | ++   | ++   | +++    |
| `07_syntax_{{lang}}.rs`           | +++  | +++  | +++    | +++   | ++   | ++   | +++    |
| `08_concat_{{lang}}.rs`           | ++   | ++   | ++     | ++    | ++   | ++   | ++     |
| `09_fn_{{lang}}.rs`               | +++  | ++   | ++     | ++    | ++   | ++   | ++     |
| `10_array_{{lang}}.rs`            | ++   | ++   | ++     | ++    | ++   | ++   | ++     |

**Legend**:

- `-`: Test file does not exist
- `+`: Test file exists
- `++`: All tests exist (but snapshots are incorrect or empty `@""`)
- `+++`: All tests snapshots are correct or the tests are not applicable (i.e., `03_inline_comment_py.rs` for `py`)

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

- `*` (added to one of the symbols above, e.g., `+++*`): There's a bug in the test, either in the test source code or the snapshot

### `01_var_name_{{lang}}.rs`

| Test Lang: | `ts` | `py` | `ruby` | `php` | `go` | `cs` | `java` |
| ---------- | ---- | ---- | ------ | ----- | ---- | ---- | ------ |
| `simple`   | +++  | +++  | +++    | +++   | ++   | ++   | +++    |
| `nested`   | +++  | +++  | +++    | +++   | ++   | ++   | +++    |

### `02_var_comment_{{lang}}.rs`

TODO

### `03_inline_comment_{{lang}}.rs`

TODO

### `04_prompt_vars_{{lang}}.rs`

TODO

### `05_multiple_prompts_{{lang}}.rs`

TODO

### `06_annotations_{{lang}}.rs`

TODO

### `07_syntax_{{lang}}.rs`

TODO

### `08_concat_{{lang}}.rs`

TODO

### `09_fn_{{lang}}.rs`

TODO

### `10_array_{{lang}}.rs`

TODO
