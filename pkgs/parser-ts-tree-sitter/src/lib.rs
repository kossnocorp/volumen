mod comments;
mod scope;
mod spans;

use comments::CommentTracker;
use scope::ScopeTracker;
use spans::{extract_template_vars, is_string_like, is_template_string, span_shape_string_like};
use tree_sitter::Node;
pub use volumen_parser_core::VolumenParser;
use volumen_types::*;

pub struct ParserTs {}

impl VolumenParser for ParserTs {
    fn parse(source: &str, filename: &str) -> ParseResult {
        let mut parser = tree_sitter::Parser::new();

        // Determine language based on file extension
        let language = if filename.ends_with(".tsx") || filename.ends_with(".jsx") {
            tree_sitter_typescript::LANGUAGE_TSX
        } else {
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT
        };

        parser.set_language(&language.into()).unwrap();

        let tree = match parser.parse(source, None) {
            Some(t) => t,
            None => {
                return ParseResult::ParseResultError(ParseResultError {
                    state: ParseResultErrorStateError,
                    error: "Failed to parse TypeScript".to_string(),
                });
            }
        };

        // Check for syntax errors
        if has_syntax_errors(&tree.root_node(), source) {
            return ParseResult::ParseResultError(ParseResultError {
                state: ParseResultErrorStateError,
                error: "Unterminated string".to_string(),
            });
        }

        let comments = CommentTracker::new(&tree, source);
        let mut scopes = ScopeTracker::new();
        let mut prompts = Vec::new();

        process_tree(
            &tree.root_node(),
            source,
            filename,
            &comments,
            &mut scopes,
            &mut prompts,
        );

        ParseResult::ParseResultSuccess(ParseResultSuccess {
            state: ParseResultSuccessStateSuccess,
            prompts,
        })
    }
}

/// Check if the tree contains syntax errors.
fn has_syntax_errors(node: &Node, _source: &str) -> bool {
    if node.is_error() || node.kind() == "ERROR" {
        return true;
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            if has_syntax_errors(&cursor.node(), _source) {
                return true;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    false
}

/// Process the entire tree starting from the root.
fn process_tree(
    node: &Node,
    source: &str,
    filename: &str,
    comments: &CommentTracker,
    scopes: &mut ScopeTracker,
    prompts: &mut Vec<Prompt>,
) {
    traverse_node(node, source, filename, comments, scopes, prompts);
}

/// Recursively traverse the tree.
fn traverse_node(
    node: &Node,
    source: &str,
    filename: &str,
    comments: &CommentTracker,
    scopes: &mut ScopeTracker,
    prompts: &mut Vec<Prompt>,
) {
    let kind = node.kind();

    // Handle scope-creating nodes
    match kind {
        "function_declaration"
        | "arrow_function"
        | "function"
        | "method_definition"
        | "class_declaration" => {
            scopes.enter_scope();
        }
        _ => {}
    }

    // Handle variable declarations
    if kind == "lexical_declaration" || kind == "variable_declaration" {
        process_variable_declaration(node, source, filename, comments, scopes, prompts);
    }

    // Handle assignment expressions (reassignments)
    if kind == "expression_statement" {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            let child = cursor.node();
            if child.kind() == "assignment_expression" {
                process_assignment_expression(&child, source, filename, comments, scopes, prompts);
            }
        }
    }

    // Traverse children
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            traverse_node(&cursor.node(), source, filename, comments, scopes, prompts);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    // Exit scope
    match kind {
        "function_declaration"
        | "arrow_function"
        | "function"
        | "method_definition"
        | "class_declaration" => {
            scopes.exit_scope();
        }
        _ => {}
    }
}

/// Process variable declarations (const, let, var).
fn process_variable_declaration(
    node: &Node,
    source: &str,
    filename: &str,
    comments: &CommentTracker,
    scopes: &mut ScopeTracker,
    prompts: &mut Vec<Prompt>,
) {
    let stmt_start = node.start_byte() as u32;
    let stmt_end = node.end_byte() as u32;

    // Collect annotations
    let inline_annotations = comments.collect_inline_prompt(stmt_start, stmt_end);
    let has_inline_prompt = !inline_annotations.is_empty();

    // If there's an inline @prompt, collect ALL adjacent leading comments
    // Otherwise only collect leading comments that contain @prompt
    let leading_annotations = if has_inline_prompt {
        comments.collect_all_adjacent_leading(stmt_start)
    } else {
        comments.collect_adjacent_leading(stmt_start)
    };

    let mut all_annotations = leading_annotations.clone();
    all_annotations.extend(inline_annotations);

    let has_prompt_annotation = !all_annotations.is_empty();

    // Find variable_declarator nodes
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            if child.kind() == "variable_declarator" {
                process_variable_declarator(
                    &child,
                    has_prompt_annotation,
                    &all_annotations,
                    stmt_start,
                    stmt_end,
                    source,
                    filename,
                    comments,
                    scopes,
                    prompts,
                );
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Process a single variable declarator.
fn process_variable_declarator(
    node: &Node,
    has_prompt_annotation: bool,
    annotations: &[PromptAnnotation],
    stmt_start: u32,
    stmt_end: u32,
    source: &str,
    filename: &str,
    comments: &CommentTracker,
    scopes: &mut ScopeTracker,
    prompts: &mut Vec<Prompt>,
) {
    // Get the identifier (name)
    let name_node = match node.child_by_field_name("name") {
        Some(n) => n,
        None => return,
    };

    let ident_name = name_node.utf8_text(source.as_bytes()).unwrap_or("");

    // Check for type annotation
    let has_type_annotation = node.child_by_field_name("type").is_some();

    // Get the value
    let value_node = node.child_by_field_name("value");

    // Handle declaration without value (e.g., `let hello;` or `let hello: string;`)
    if value_node.is_none() {
        if has_prompt_annotation {
            // Store annotations and mark as prompt identifier
            scopes.store_def_annotation(ident_name, annotations.to_vec());
            scopes.mark_prompt_ident(ident_name);
            if has_type_annotation {
                scopes.mark_annotated(ident_name);
            }
        }
        return;
    }

    let value = value_node.unwrap();

    // Determine if this is a prompt
    let is_prompt = is_prompt_variable(ident_name, has_prompt_annotation, scopes);

    if is_prompt {
        // Mark as prompt identifier (even if value isn't a string yet)
        scopes.mark_prompt_ident(ident_name);

        // Store definition annotations if this has annotations
        if has_prompt_annotation {
            scopes.store_def_annotation(ident_name, annotations.to_vec());
        }

        // Mark as type-annotated if applicable
        if has_type_annotation {
            scopes.mark_annotated(ident_name);
        }

        // Check if it's a string or template string
        if is_string_like(&value) {
            // Get annotations (from current statement or from definition)
            let final_annotations = if !annotations.is_empty() {
                annotations.to_vec()
            } else {
                scopes.get_def_annotation(ident_name).unwrap_or_default()
            };

            // Create prompt
            create_prompt_from_string(
                &value,
                source,
                filename,
                stmt_start,
                stmt_end,
                comments,
                &final_annotations,
                true, // annotations are from current statement
                prompts,
            );
        }
    }
}

/// Determine if an identifier should be treated as a prompt.
fn is_prompt_variable(ident_name: &str, has_annotation: bool, scopes: &ScopeTracker) -> bool {
    // Name contains "prompt" (case-insensitive)
    if ident_name.to_lowercase().contains("prompt") {
        return true;
    }

    // Has @prompt annotation
    if has_annotation {
        return true;
    }

    // In scope tracking (was declared with @prompt)
    if scopes.is_prompt_ident(ident_name) {
        return true;
    }

    false
}

/// Process assignment expressions (reassignments like `hello = value`).
fn process_assignment_expression(
    node: &Node,
    source: &str,
    filename: &str,
    comments: &CommentTracker,
    scopes: &mut ScopeTracker,
    prompts: &mut Vec<Prompt>,
) {
    // Get the left side (identifier)
    let left = match node.child_by_field_name("left") {
        Some(n) => n,
        None => return,
    };

    if left.kind() != "identifier" {
        return; // Only handle simple identifiers
    }

    let ident_name = left.utf8_text(source.as_bytes()).unwrap_or("");

    // Get the right side (value)
    let right = match node.child_by_field_name("right") {
        Some(n) => n,
        None => return,
    };

    // Get the statement bounds (parent expression_statement)
    let parent_stmt = node.parent().unwrap();
    let stmt_start = parent_stmt.start_byte() as u32;
    let stmt_end = parent_stmt.end_byte() as u32;

    // Collect annotations
    let inline_annotations = comments.collect_inline_prompt(stmt_start, stmt_end);
    let has_inline_prompt = !inline_annotations.is_empty();

    // If there's an inline @prompt, collect ALL adjacent leading comments
    // Otherwise only collect leading comments that contain @prompt
    let leading_annotations = if has_inline_prompt {
        comments.collect_all_adjacent_leading(stmt_start)
    } else {
        comments.collect_adjacent_leading(stmt_start)
    };

    let mut all_annotations = leading_annotations.clone();
    all_annotations.extend(inline_annotations);

    let has_prompt_annotation = !all_annotations.is_empty();

    // Determine if this is a prompt
    let is_prompt = is_prompt_variable(ident_name, has_prompt_annotation, scopes);

    if is_prompt {
        scopes.mark_prompt_ident(ident_name);

        if is_string_like(&right) {
            // Get annotations (from current statement or from definition)
            let (final_annotations, from_current) = if !all_annotations.is_empty() {
                (all_annotations, true)
            } else {
                (
                    scopes.get_def_annotation(ident_name).unwrap_or_default(),
                    false,
                )
            };

            // Create prompt
            create_prompt_from_string(
                &right,
                source,
                filename,
                stmt_start,
                stmt_end,
                comments,
                &final_annotations,
                from_current,
                prompts,
            );
        }
    }
}

/// Create a prompt from a string or template string node.
fn create_prompt_from_string(
    string_node: &Node,
    source: &str,
    filename: &str,
    stmt_start: u32,
    stmt_end: u32,
    comments: &CommentTracker,
    annotations: &[PromptAnnotation],
    annotations_from_current_stmt: bool,
    prompts: &mut Vec<Prompt>,
) {
    // Calculate spans
    let span = span_shape_string_like(string_node, source);

    // Extract expression text
    let exp = source[span.outer.start as usize..span.outer.end as usize].to_string();

    // Extract variables if template string
    let vars = if is_template_string(string_node) {
        extract_template_vars(string_node, source)
    } else {
        Vec::new()
    };

    // Calculate enclosure
    // Only use annotation positions if they're from the current statement
    let enclosure_start = if annotations_from_current_stmt && !annotations.is_empty() {
        if annotations.len() >= 2 {
            // Multiple annotations: first one is leading comment
            annotations.first().unwrap().span.start
        } else {
            let ann = annotations.first().unwrap();
            // Check if annotation is before statement (leading)
            if ann.span.start < stmt_start {
                ann.span.start
            } else {
                // Inline annotation
                stmt_start
            }
        }
    } else {
        stmt_start
    };

    let enclosure = Span {
        start: enclosure_start,
        end: stmt_end,
    };

    prompts.push(Prompt {
        file: filename.to_string(),
        span,
        enclosure,
        exp,
        vars,
        annotations: annotations.to_vec(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use insta::assert_debug_snapshot;
    use volumen_parser_test::*;

    #[test]
    fn detect_inline() {
        let inline_src = r#"const greeting = /* @prompt */ `Welcome ${user}!`;"#;
        assert_debug_snapshot!(ParserTs::parse(inline_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 31,
                                end: 49,
                            },
                            inner: Span {
                                start: 32,
                                end: 48,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 50,
                        },
                        exp: "`Welcome ${user}!`",
                        vars: [
                            PromptVar {
                                exp: "${user}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 40,
                                        end: 47,
                                    },
                                    inner: Span {
                                        start: 42,
                                        end: 46,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 17,
                                    end: 30,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn detect_inline_jsdoc() {
        let inline_jsdoc_src = r#"const msg = /** @prompt */ "Hello world";"#;
        assert_debug_snapshot!(ParserTs::parse(inline_jsdoc_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 27,
                                end: 40,
                            },
                            inner: Span {
                                start: 28,
                                end: 39,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 41,
                        },
                        exp: "\"Hello world\"",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 12,
                                    end: 26,
                                },
                                exp: "/** @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn detect_inline_dirty() {
        let inline_dirty_src = r#"const greeting = /* @prompt greeting */ `Welcome ${user}!`;"#;
        assert_prompts_size(ParserTs::parse(inline_dirty_src, "prompts.ts"), 1)
    }

    #[test]
    fn detect_inline_none() {
        let inline_none_src = r#"const greeting = /* @prompting */ `Welcome ${user}!`;
const whatever = /* wrong@prompt */ "That's not it!";"#;
        assert_prompts_size(ParserTs::parse(inline_none_src, "prompts.ts"), 0)
    }

    #[test]
    fn detect_var_inline() {
        let var_comment_src = indoc! {r#"
            /* @prompt */
            const hello = `Hello, world!`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(var_comment_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 28,
                                end: 43,
                            },
                            inner: Span {
                                start: 29,
                                end: 42,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 44,
                        },
                        exp: "`Hello, world!`",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 13,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn detect_var_jsdoc() {
        let var_jsdoc_src = indoc! {r#"
            /** @prompt */
            const hello = `Hello, world!`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(var_jsdoc_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 29,
                                end: 44,
                            },
                            inner: Span {
                                start: 30,
                                end: 43,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 45,
                        },
                        exp: "`Hello, world!`",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 14,
                                },
                                exp: "/** @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn detect_var_comment_spaced() {
        let var_comment_src = indoc! {r#"
            // @prompt


            const hello = `Hello, world!`;

            // @prompt
            nope()

            const world = "Hello!";
        "#};
        assert_debug_snapshot!(ParserTs::parse(var_comment_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 27,
                                end: 42,
                            },
                            inner: Span {
                                start: 28,
                                end: 41,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 43,
                        },
                        exp: "`Hello, world!`",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 10,
                                },
                                exp: "// @prompt",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn multiline_annotation() {
        let src = indoc! {r#"
            // Hello
            // @prompt
            // world
            const msg = "Hello";
        "#};
        match ParserTs::parse(src, "prompts.ts") {
            ParseResult::ParseResultSuccess(ParseResultSuccess { prompts, .. }) => {
                assert_eq!(prompts.len(), 1, "expected exactly one prompt");
                let ann = &prompts[0].annotations;
                assert_eq!(ann.len(), 1, "expected a single combined annotation block");
                assert_eq!(ann[0].exp, "// Hello\n// @prompt\n// world");
            }
            _ => panic!("Expected ParseResultSuccess"),
        }
    }

    #[test]
    fn detect_multi() {
        let multi_src = indoc! {r#"
            const userPrompt = `Hello, ${name}!`;
            const greeting = /* @prompt */ `Welcome ${user}!`;
            // @prompt
            const farewell = `Goodbye ${user.name}!`;
            /** @prompt */
            const system = "You are an AI assistant";
            const regular = `Not a prompt ${value}`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(multi_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 19,
                                end: 36,
                            },
                            inner: Span {
                                start: 20,
                                end: 35,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 37,
                        },
                        exp: "`Hello, ${name}!`",
                        vars: [
                            PromptVar {
                                exp: "${name}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 27,
                                        end: 34,
                                    },
                                    inner: Span {
                                        start: 29,
                                        end: 33,
                                    },
                                },
                            },
                        ],
                        annotations: [],
                    },
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 69,
                                end: 87,
                            },
                            inner: Span {
                                start: 70,
                                end: 86,
                            },
                        },
                        enclosure: Span {
                            start: 38,
                            end: 88,
                        },
                        exp: "`Welcome ${user}!`",
                        vars: [
                            PromptVar {
                                exp: "${user}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 78,
                                        end: 85,
                                    },
                                    inner: Span {
                                        start: 80,
                                        end: 84,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 55,
                                    end: 68,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 117,
                                end: 140,
                            },
                            inner: Span {
                                start: 118,
                                end: 139,
                            },
                        },
                        enclosure: Span {
                            start: 89,
                            end: 141,
                        },
                        exp: "`Goodbye ${user.name}!`",
                        vars: [
                            PromptVar {
                                exp: "${user.name}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 126,
                                        end: 138,
                                    },
                                    inner: Span {
                                        start: 128,
                                        end: 137,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 89,
                                    end: 99,
                                },
                                exp: "// @prompt",
                            },
                        ],
                    },
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 172,
                                end: 197,
                            },
                            inner: Span {
                                start: 173,
                                end: 196,
                            },
                        },
                        enclosure: Span {
                            start: 142,
                            end: 198,
                        },
                        exp: "\"You are an AI assistant\"",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 142,
                                    end: 156,
                                },
                                exp: "/** @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn detect_reassign_var_comment() {
        let reassign_var_comment = indoc! {r#"
            // @prompt
            let hello;
            hello = 123;

            hello = `Assigned ${value}`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(
            reassign_var_comment,
            "prompts.ts"
        ), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 44,
                                end: 63,
                            },
                            inner: Span {
                                start: 45,
                                end: 62,
                            },
                        },
                        enclosure: Span {
                            start: 36,
                            end: 64,
                        },
                        exp: "`Assigned ${value}`",
                        vars: [
                            PromptVar {
                                exp: "${value}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 54,
                                        end: 62,
                                    },
                                    inner: Span {
                                        start: 56,
                                        end: 61,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 10,
                                },
                                exp: "// @prompt",
                            },
                        ],
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn detect_single_var() {
        let single_var_src = r#"const userPrompt = `Hello, ${name}!`;"#;
        assert_debug_snapshot!(ParserTs::parse(single_var_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 19,
                                end: 36,
                            },
                            inner: Span {
                                start: 20,
                                end: 35,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 37,
                        },
                        exp: "`Hello, ${name}!`",
                        vars: [
                            PromptVar {
                                exp: "${name}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 27,
                                        end: 34,
                                    },
                                    inner: Span {
                                        start: 29,
                                        end: 33,
                                    },
                                },
                            },
                        ],
                        annotations: [],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn detect_multi_vars() {
        let multi_vars_src =
            r#"const userPrompt = `Hello, ${name}! How is the weather today in ${city}?`;"#;
        assert_debug_snapshot!(ParserTs::parse(multi_vars_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 19,
                                end: 73,
                            },
                            inner: Span {
                                start: 20,
                                end: 72,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 74,
                        },
                        exp: "`Hello, ${name}! How is the weather today in ${city}?`",
                        vars: [
                            PromptVar {
                                exp: "${name}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 27,
                                        end: 34,
                                    },
                                    inner: Span {
                                        start: 29,
                                        end: 33,
                                    },
                                },
                            },
                            PromptVar {
                                exp: "${city}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 64,
                                        end: 71,
                                    },
                                    inner: Span {
                                        start: 66,
                                        end: 70,
                                    },
                                },
                            },
                        ],
                        annotations: [],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn parse_exp_vars() {
        let exp_vars_src = r#"const userPrompt = `Hello, ${user.name}! How is the weather today in ${user.location.city}?`;"#;
        assert_debug_snapshot!(ParserTs::parse(exp_vars_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 19,
                                end: 92,
                            },
                            inner: Span {
                                start: 20,
                                end: 91,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 93,
                        },
                        exp: "`Hello, ${user.name}! How is the weather today in ${user.location.city}?`",
                        vars: [
                            PromptVar {
                                exp: "${user.name}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 27,
                                        end: 39,
                                    },
                                    inner: Span {
                                        start: 29,
                                        end: 38,
                                    },
                                },
                            },
                            PromptVar {
                                exp: "${user.location.city}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 69,
                                        end: 90,
                                    },
                                    inner: Span {
                                        start: 71,
                                        end: 89,
                                    },
                                },
                            },
                        ],
                        annotations: [],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn parse_exp_vars_complex() {
        let exp_vars_src =
            r#"const userPrompt = `Hello, ${User.fullName({ ...user.name, last: null })}!`;"#;
        assert_debug_snapshot!(ParserTs::parse(exp_vars_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 19,
                                end: 75,
                            },
                            inner: Span {
                                start: 20,
                                end: 74,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 76,
                        },
                        exp: "`Hello, ${User.fullName({ ...user.name, last: null })}!`",
                        vars: [
                            PromptVar {
                                exp: "${User.fullName({ ...user.name, last: null })}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 27,
                                        end: 73,
                                    },
                                    inner: Span {
                                        start: 29,
                                        end: 72,
                                    },
                                },
                            },
                        ],
                        annotations: [],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn handle_invalid_syntax() {
        let invalid_src = r#"const invalid = `unclosed template"#;
        assert_debug_snapshot!(ParserTs::parse(invalid_src, "prompts.ts"), @r#"
        ParseResultError(
            ParseResultError {
                state: "error",
                error: "Unterminated string",
            },
        )
        "#);
    }

    #[test]
    fn parse_js_code() {
        let js_src = r#"const prompt = /* @prompt */ `Hello ${world}!`;"#;
        assert_debug_snapshot!(ParserTs::parse(js_src, "test.js"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "test.js",
                        span: SpanShape {
                            outer: Span {
                                start: 29,
                                end: 46,
                            },
                            inner: Span {
                                start: 30,
                                end: 45,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 47,
                        },
                        exp: "`Hello ${world}!`",
                        vars: [
                            PromptVar {
                                exp: "${world}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 36,
                                        end: 44,
                                    },
                                    inner: Span {
                                        start: 38,
                                        end: 43,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 15,
                                    end: 28,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn parse_jsx_code() {
        let jsx_src = indoc! {r#"
            const prompt = /* @prompt */ `Hello ${world}!`;
            const element = <div>{prompt}</div>;
        "#};
        assert_debug_snapshot!(ParserTs::parse(jsx_src, "test.jsx"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "test.jsx",
                        span: SpanShape {
                            outer: Span {
                                start: 29,
                                end: 46,
                            },
                            inner: Span {
                                start: 30,
                                end: 45,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 47,
                        },
                        exp: "`Hello ${world}!`",
                        vars: [
                            PromptVar {
                                exp: "${world}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 36,
                                        end: 44,
                                    },
                                    inner: Span {
                                        start: 38,
                                        end: 43,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 15,
                                    end: 28,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn parse_ts_code() {
        let ts_src = r#"const prompt : string = /* @prompt */ `Hello ${world}!`;"#;
        assert_debug_snapshot!(ParserTs::parse(ts_src, "test.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "test.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 38,
                                end: 55,
                            },
                            inner: Span {
                                start: 39,
                                end: 54,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 56,
                        },
                        exp: "`Hello ${world}!`",
                        vars: [
                            PromptVar {
                                exp: "${world}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 45,
                                        end: 53,
                                    },
                                    inner: Span {
                                        start: 47,
                                        end: 52,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 24,
                                    end: 37,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn parse_tsx_code() {
        let tsx_src = indoc! {r#"
            const prompt : string = /* @prompt */ `Hello ${world}!`;
            const element = <div>{prompt}</div>;
        "#};
        assert_debug_snapshot!(ParserTs::parse(tsx_src, "test.tsx"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "test.tsx",
                        span: SpanShape {
                            outer: Span {
                                start: 38,
                                end: 55,
                            },
                            inner: Span {
                                start: 39,
                                end: 54,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 56,
                        },
                        exp: "`Hello ${world}!`",
                        vars: [
                            PromptVar {
                                exp: "${world}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 45,
                                        end: 53,
                                    },
                                    inner: Span {
                                        start: 47,
                                        end: 52,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 24,
                                    end: 37,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn parse_spans_str() {
        let span_str_src = indoc! {r#"
            const systemPrompt = "You are a helpful assistant.";
        "#};
        let span_str_result = ParserTs::parse(span_str_src, "prompts.ts");
        assert_prompt_spans(span_str_src, span_str_result);
    }

    #[test]
    fn parse_spans_tmpl() {
        let span_tmpl_src = indoc! {r#"
            const userPrompt = `Hello, ${name}! How is the weather today in ${city}?`;
        "#};
        let span_tmpl_result = ParserTs::parse(span_tmpl_src, "prompts.ts");
        assert_prompt_spans(span_tmpl_src, span_tmpl_result);
    }

    #[test]
    fn parse_nested() {
        let nested_src = indoc! {r#"
            class Hello {
                world(self) {
                    const fn = () => {
                        const helloPrompt = `Hello, ${name}!`;

                        // @prompt
                        const alsoPrmpt = "Hi!";
                    };

                    return fn;
                }
            }
        "#};
        assert_debug_snapshot!(ParserTs::parse(nested_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 91,
                                end: 108,
                            },
                            inner: Span {
                                start: 92,
                                end: 107,
                            },
                        },
                        enclosure: Span {
                            start: 71,
                            end: 109,
                        },
                        exp: "`Hello, ${name}!`",
                        vars: [
                            PromptVar {
                                exp: "${name}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 99,
                                        end: 106,
                                    },
                                    inner: Span {
                                        start: 101,
                                        end: 105,
                                    },
                                },
                            },
                        ],
                        annotations: [],
                    },
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 164,
                                end: 169,
                            },
                            inner: Span {
                                start: 165,
                                end: 168,
                            },
                        },
                        enclosure: Span {
                            start: 123,
                            end: 170,
                        },
                        exp: "\"Hi!\"",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 123,
                                    end: 133,
                                },
                                exp: "// @prompt",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn multi_annotations() {
        let src = indoc! {r#"
            // Hello, world
            const hello = /* @prompt */ "asd";
        "#};
        assert_debug_snapshot!(ParserTs::parse(src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 44,
                                end: 49,
                            },
                            inner: Span {
                                start: 45,
                                end: 48,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 50,
                        },
                        exp: "\"asd\"",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 15,
                                },
                                exp: "// Hello, world",
                            },
                            PromptAnnotation {
                                span: Span {
                                    start: 30,
                                    end: 43,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn multiline_annotations() {
        let src = indoc! {r#"
            /*
             Multi
             Line
             Block
            */
            const hello = /* @prompt */ `x`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 54,
                                end: 57,
                            },
                            inner: Span {
                                start: 55,
                                end: 56,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 58,
                        },
                        exp: "`x`",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 25,
                                },
                                exp: "/*\n Multi\n Line\n Block\n*/",
                            },
                            PromptAnnotation {
                                span: Span {
                                    start: 40,
                                    end: 53,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn multiline_annotation_nested() {
        let src = indoc! {r#"
            function fn() {
                // Hello
                // @prompt
                // world
                const msg = "Hello";
            }
        "#};
        assert_debug_snapshot!(ParserTs::parse(src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 73,
                                end: 80,
                            },
                            inner: Span {
                                start: 74,
                                end: 79,
                            },
                        },
                        enclosure: Span {
                            start: 20,
                            end: 81,
                        },
                        exp: "\"Hello\"",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 20,
                                    end: 56,
                                },
                                exp: "// Hello\n    // @prompt\n    // world",
                            },
                        ],
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn reassign_no_comment() {
        let src = indoc! {r#"
            // @prompt
            let hello: string;
            hello = `Hi`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(src, "prompts.ts"),@r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 38,
                                end: 42,
                            },
                            inner: Span {
                                start: 39,
                                end: 41,
                            },
                        },
                        enclosure: Span {
                            start: 30,
                            end: 43,
                        },
                        exp: "`Hi`",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 10,
                                },
                                exp: "// @prompt",
                            },
                        ],
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn reassign_with_comment() {
        let src = indoc! {r#"
            // @prompt def
            let hello: string;
            // @prompt fresh
            hello = `Hi`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(src, "prompts.ts"),@r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 59,
                                end: 63,
                            },
                            inner: Span {
                                start: 60,
                                end: 62,
                            },
                        },
                        enclosure: Span {
                            start: 34,
                            end: 64,
                        },
                        exp: "`Hi`",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 34,
                                    end: 50,
                                },
                                exp: "// @prompt fresh",
                            },
                        ],
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn reassign_with_comment_multi() {
        let src = indoc! {r#"
            // @prompt def
            let hello: string;
            // hello
            hello = /* @prompt fresh */ `Hi`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(src, "prompts.ts"),@r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 71,
                                end: 75,
                            },
                            inner: Span {
                                start: 72,
                                end: 74,
                            },
                        },
                        enclosure: Span {
                            start: 34,
                            end: 76,
                        },
                        exp: "`Hi`",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 34,
                                    end: 42,
                                },
                                exp: "// hello",
                            },
                            PromptAnnotation {
                                span: Span {
                                    start: 51,
                                    end: 70,
                                },
                                exp: "/* @prompt fresh */",
                            },
                        ],
                    },
                ],
            },
        )
        "#);
    }
}
