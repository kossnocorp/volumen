mod comments;
mod scope;
mod spans;

use comments::CommentTracker;
use scope::ScopeTracker;
use spans::{extract_template_vars, is_string_like, is_template_string, span_shape_string_like};
use tree_sitter::Node;
pub use volumen_parser_core::VolumenParser;
use volumen_parser_core::parse_annotation;
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
/// Recursively extract all identifiers from a pattern node (object_pattern, array_pattern, etc.)
fn extract_pattern_identifiers(node: &Node, source: &str, identifiers: &mut Vec<String>) {
    let kind = node.kind();

    // Match identifier node types
    if kind == "identifier" || kind == "shorthand_property_identifier_pattern" {
        if let Ok(name) = node.utf8_text(source.as_bytes()) {
            identifiers.push(name.to_string());
        }
        return;
    }

    // For object patterns with property assignments, extract from the value side
    if kind == "pair_pattern" {
        // pair_pattern has a "value" field with the actual pattern
        if let Some(value_node) = node.child_by_field_name("value") {
            extract_pattern_identifiers(&value_node, source, identifiers);
            return;
        }
    }

    // Recursively walk the pattern's children
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            extract_pattern_identifiers(&child, source, identifiers);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Recursively extract all string/template literal values from a value node
fn extract_value_strings<'a>(node: &Node<'a>, values: &mut Vec<Node<'a>>) {
    let kind = node.kind();

    if kind == "string" || kind == "template_string" {
        values.push(*node);
        return;
    }

    // Recursively walk the value's children
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            extract_value_strings(&child, values);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

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

    let name_kind = name_node.kind();

    // Check if this is a destructuring pattern
    if (name_kind == "object_pattern" || name_kind == "array_pattern") && has_prompt_annotation {
        // Handle destructuring pattern
        let value_node = match node.child_by_field_name("value") {
            Some(v) => v,
            None => return,
        };

        // Extract all identifiers from the pattern
        let mut identifiers = Vec::new();
        extract_pattern_identifiers(&name_node, source, &mut identifiers);

        // Extract all string values from the value
        let mut values = Vec::new();
        extract_value_strings(&value_node, &mut values);

        // Match identifiers with values by position
        for (i, ident_name) in identifiers.iter().enumerate() {
            if let Some(value) = values.get(i) {
                if is_prompt_variable(ident_name, has_prompt_annotation, scopes) {
                    // Mark as prompt identifier
                    scopes.mark_prompt_ident(ident_name);

                    // Store definition annotations
                    if has_prompt_annotation {
                        scopes.store_def_annotation(ident_name, annotations.to_vec());
                    }

                    // Create prompt for this value
                    create_prompt_from_string(
                        value,
                        source,
                        filename,
                        stmt_start,
                        stmt_end,
                        comments,
                        annotations,
                        true,
                        prompts,
                    );
                }
            }
        }
        return;
    }

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

        // Check if it's a chained assignment (e.g., const hello = world = "Hi")
        if value.kind() == "assignment_expression" {
            // Walk the chain to find the ultimate value
            let mut current = value;
            loop {
                let right = match current.child_by_field_name("right") {
                    Some(r) => r,
                    None => break,
                };

                if right.kind() == "assignment_expression" {
                    current = right;
                } else if is_string_like(&right) {
                    // Found the ultimate value - create prompt for current identifier
                    let final_annotations = if !annotations.is_empty() {
                        annotations.to_vec()
                    } else {
                        scopes.get_def_annotation(ident_name).unwrap_or_default()
                    };

                    create_prompt_from_string(
                        &right,
                        source,
                        filename,
                        stmt_start,
                        stmt_end,
                        comments,
                        &final_annotations,
                        true,
                        prompts,
                    );
                    break;
                } else {
                    break;
                }
            }

            // Recursively process the chained assignment
            process_assignment_expression_with_annotations(
                &value,
                source,
                filename,
                comments,
                scopes,
                prompts,
                annotations,
                stmt_start,
                stmt_end,
            );
        } else if is_string_like(&value) {
            // Check if it's a string or template string
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

/// Process assignment expression with inherited annotations (for chained assignments).
fn process_assignment_expression_with_annotations(
    node: &Node,
    source: &str,
    filename: &str,
    comments: &CommentTracker,
    scopes: &mut ScopeTracker,
    prompts: &mut Vec<Prompt>,
    inherited_annotations: &[PromptAnnotation],
    stmt_start: u32,
    stmt_end: u32,
) {
    // Get the left side (identifier)
    let left = match node.child_by_field_name("left") {
        Some(n) => n,
        None => return,
    };

    if left.kind() != "identifier" {
        return;
    }

    let ident_name = left.utf8_text(source.as_bytes()).unwrap_or("");

    // Get the right side (value)
    let right = match node.child_by_field_name("right") {
        Some(n) => n,
        None => return,
    };

    // Determine if this is a prompt
    let is_prompt = is_prompt_variable(ident_name, !inherited_annotations.is_empty(), scopes);

    if is_prompt {
        scopes.mark_prompt_ident(ident_name);

        // Check if it's a chained assignment
        if right.kind() == "assignment_expression" {
            // Walk the chain to find the ultimate value
            let mut current = right;
            loop {
                let r = match current.child_by_field_name("right") {
                    Some(r) => r,
                    None => break,
                };

                if r.kind() == "assignment_expression" {
                    current = r;
                } else if is_string_like(&r) {
                    // Found the ultimate value
                    create_prompt_from_string(
                        &r,
                        source,
                        filename,
                        stmt_start,
                        stmt_end,
                        comments,
                        inherited_annotations,
                        true,
                        prompts,
                    );
                    break;
                } else {
                    break;
                }
            }

            // Recursively process the nested assignment
            process_assignment_expression_with_annotations(
                &right,
                source,
                filename,
                comments,
                scopes,
                prompts,
                inherited_annotations,
                stmt_start,
                stmt_end,
            );
        } else if is_string_like(&right) {
            create_prompt_from_string(
                &right,
                source,
                filename,
                stmt_start,
                stmt_end,
                comments,
                inherited_annotations,
                true,
                prompts,
            );
        }
    }
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
            // Check if current annotations contain at least one valid @prompt
            let has_valid_current_annotation = all_annotations
                .iter()
                .any(|a| parse_annotation(&a.exp).unwrap_or(false));

            // Get annotations (from current statement or from definition)
            let (final_annotations, from_current) = if has_valid_current_annotation {
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
    let exp = source[span.outer.0 as usize..span.outer.1 as usize].to_string();

    // Extract variables if template string
    let vars = if is_template_string(string_node) {
        extract_template_vars(string_node, source)
    } else {
        Vec::new()
    };

    // Calculate enclosure
    // Always check for leading comments, even if they're invalid annotations
    let leading_start = comments.get_any_leading_start(stmt_start);
    let enclosure_start = if annotations_from_current_stmt && !annotations.is_empty() {
        if annotations.len() >= 2 {
            // Multiple annotations: first one is leading comment
            annotations.first().unwrap().span.0
        } else {
            let ann = annotations.first().unwrap();
            // Check if annotation is before statement (leading)
            if ann.span.0 < stmt_start {
                ann.span.0
            } else {
                // Inline annotation
                stmt_start
            }
        }
    } else {
        // Even with stored definition annotations, include leading comments in enclosure
        leading_start.unwrap_or(stmt_start)
    };

    let enclosure = (enclosure_start, stmt_end);

    prompts.push(Prompt {
        file: filename.to_string(),
        span,
        enclosure,
        exp,
        vars,
        annotations: annotations.to_vec(),
    });
}
