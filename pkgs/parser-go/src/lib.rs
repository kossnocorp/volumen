mod comments;
mod scope;
mod spans;

use comments::CommentTracker;
use scope::ScopeTracker;
use spans::span_shape_string_like;
use tree_sitter::{Node, Parser, Tree};
pub use volumen_parser_core::VolumenParser;
use volumen_types::*;

pub struct ParserGo {}

impl VolumenParser for ParserGo {
    fn parse(source: &str, filename: &str) -> ParseResult {
        // Initialize Tree-sitter parser
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_go::LANGUAGE.into())
            .expect("Failed to load Go grammar");

        // Parse source
        let tree = match parser.parse(source, None) {
            Some(tree) => tree,
            None => {
                return ParseResult::ParseResultError(ParseResultError {
                    state: ParseResultErrorStateError,
                    error: "Failed to parse source".to_string(),
                });
            }
        };

        // Check for parse errors
        let root = tree.root_node();
        if root.has_error() {
            let error_msg = format_tree_sitter_error(&root, source);
            return ParseResult::ParseResultError(ParseResultError {
                state: ParseResultErrorStateError,
                error: error_msg,
            });
        }

        // Extract comments
        let comment_tracker = CommentTracker::new(&tree, source);

        // Initialize state
        let mut prompts = Vec::new();
        let mut scope_tracker = ScopeTracker::new();

        // Process tree
        process_tree(
            &tree,
            &root,
            source,
            filename,
            &comment_tracker,
            &mut scope_tracker,
            &mut prompts,
        );

        ParseResult::ParseResultSuccess(ParseResultSuccess {
            state: ParseResultSuccessStateSuccess,
            prompts,
        })
    }
}

/// Format tree-sitter parse errors into a readable message.
fn format_tree_sitter_error(node: &Node, source: &str) -> String {
    let mut errors = Vec::new();
    collect_errors(node, source, &mut errors);
    if errors.is_empty() {
        "Parse error: unknown".to_string()
    } else {
        format!("Parse errors:\n{}", errors.join("\n"))
    }
}

fn collect_errors(node: &Node, source: &str, errors: &mut Vec<String>) {
    if node.is_error() || node.is_missing() {
        let start = node.start_position();
        let error_text = node
            .utf8_text(source.as_bytes())
            .unwrap_or("<unknown>")
            .chars()
            .take(50)
            .collect::<String>();
        errors.push(format!(
            "  Line {}, Column {}: {}",
            start.row + 1,
            start.column + 1,
            error_text
        ));
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            collect_errors(&cursor.node(), source, errors);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Process the entire syntax tree to extract prompts.
fn process_tree(
    _tree: &Tree,
    root: &Node,
    source: &str,
    filename: &str,
    comments: &CommentTracker,
    scopes: &mut ScopeTracker,
    prompts: &mut Vec<Prompt>,
) {
    // Recursively traverse the tree
    traverse_node(root, source, filename, comments, scopes, prompts);
}

/// Recursively traverse nodes to find assignments and manage scopes.
fn traverse_node(
    node: &Node,
    source: &str,
    filename: &str,
    comments: &CommentTracker,
    scopes: &mut ScopeTracker,
    prompts: &mut Vec<Prompt>,
) {
    let kind = node.kind();

    // Handle scope boundaries (function, method)
    if kind == "function_declaration" || kind == "method_declaration" {
        scopes.enter_scope();

        // Process children
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                traverse_node(&child, source, filename, comments, scopes, prompts);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        scopes.exit_scope();
        return;
    }

    // Handle short variable declarations (x := value)
    if kind == "short_var_declaration" {
        process_short_var_declaration(node, source, filename, comments, scopes, prompts);
        return;
    }

    // Handle var declarations (var x = value)
    if kind == "var_declaration" {
        process_var_declaration(node, source, filename, comments, scopes, prompts);
        return;
    }

    // Handle assignment statements (x = value)
    if kind == "assignment_statement" {
        process_assignment(node, source, filename, comments, scopes, prompts);
        return;
    }

    // Recursively process children
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            traverse_node(&child, source, filename, comments, scopes, prompts);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Process a short variable declaration (x := value).
fn process_short_var_declaration(
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
    let leading_annotations = comments.collect_adjacent_leading(stmt_start);
    let inline_annotations = comments.collect_inline_prompt(stmt_start, stmt_end);
    let mut all_annotations = leading_annotations.clone();
    all_annotations.extend(inline_annotations);

    let has_prompt_annotation = !all_annotations.is_empty();

    // Get left and right sides
    let left = match node.child_by_field_name("left") {
        Some(n) => n,
        None => return,
    };

    let right = match node.child_by_field_name("right") {
        Some(n) => n,
        None => return,
    };

    // Extract identifiers from left side
    let mut identifiers = Vec::new();
    extract_identifiers(&left, source, &mut identifiers);

    // Unwrap expression_list if present (Go wraps right side in expression_list)
    let actual_right = if right.kind() == "expression_list" {
        // Get first child of expression_list
        right.child(0).unwrap_or(right)
    } else {
        right
    };

    // Process the right side - if it's a single string, match with first identifier
    if is_string_like(&actual_right) && !identifiers.is_empty() {
        let ident_name = &identifiers[0];
        let is_prompt = is_prompt_variable(ident_name, has_prompt_annotation, scopes);

        if is_prompt {
            scopes.mark_prompt_ident(ident_name);

            if has_prompt_annotation {
                scopes.store_def_annotation(ident_name, all_annotations.clone());
            }

            // Annotations from comment tracker are already validated to contain @prompt
            // Get annotations (from current statement or from definition)
            let final_annotations = if !all_annotations.is_empty() {
                all_annotations.clone()
            } else {
                scopes.get_def_annotation(ident_name).unwrap_or_default()
            };

            create_prompt_from_string(
                &actual_right,
                source,
                filename,
                stmt_start,
                stmt_end,
                comments,
                &final_annotations,
                prompts,
            );
        }
    } else if actual_right.kind() == "binary_expression" && !identifiers.is_empty() {
        let ident_name = &identifiers[0];
        let is_prompt = is_prompt_variable(ident_name, has_prompt_annotation, scopes);

        if is_prompt {
            scopes.mark_prompt_ident(ident_name);

            if has_prompt_annotation {
                scopes.store_def_annotation(ident_name, all_annotations.clone());
            }

            let final_annotations = if !all_annotations.is_empty() {
                all_annotations.clone()
            } else {
                scopes.get_def_annotation(ident_name).unwrap_or_default()
            };

            // Try to process as concatenation
            if let Some(prompt) = process_concatenation(
                &actual_right,
                source,
                filename,
                stmt_start,
                stmt_end,
                comments,
                &final_annotations,
            ) {
                prompts.push(prompt);
            }
        }
    } else if actual_right.kind() == "call_expression" && !identifiers.is_empty() {
        let ident_name = &identifiers[0];
        let is_prompt = is_prompt_variable(ident_name, has_prompt_annotation, scopes);

        if is_prompt {
            scopes.mark_prompt_ident(ident_name);

            if has_prompt_annotation {
                scopes.store_def_annotation(ident_name, all_annotations.clone());
            }

            let final_annotations = if !all_annotations.is_empty() {
                all_annotations.clone()
            } else {
                scopes.get_def_annotation(ident_name).unwrap_or_default()
            };

            // Try strings.Join first
            if let Some(prompt) = process_strings_join(
                &actual_right,
                source,
                filename,
                stmt_start,
                stmt_end,
                comments,
                &final_annotations,
            ) {
                prompts.push(prompt);
            // Then try fmt.Sprintf
            } else if let Some(prompt) = process_sprintf_call(
                &actual_right,
                source,
                filename,
                stmt_start,
                stmt_end,
                comments,
                &final_annotations,
            ) {
                prompts.push(prompt);
            }
        }
    } else if (actual_right.kind() == "composite_literal" || actual_right.kind() == "slice_literal") && !identifiers.is_empty() {
        let ident_name = &identifiers[0];
        let is_prompt = is_prompt_variable(ident_name, has_prompt_annotation, scopes);

        if is_prompt {
            scopes.mark_prompt_ident(ident_name);

            if has_prompt_annotation {
                scopes.store_def_annotation(ident_name, all_annotations.clone());
            }

            let final_annotations = if !all_annotations.is_empty() {
                all_annotations.clone()
            } else {
                scopes.get_def_annotation(ident_name).unwrap_or_default()
            };

            // Try to process as array
            if let Some(prompt) = process_array(
                &actual_right,
                source,
                filename,
                stmt_start,
                stmt_end,
                comments,
                &final_annotations,
            ) {
                prompts.push(prompt);
            }
        }
    }
}

/// Process a var declaration (var x = value).
fn process_var_declaration(
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
    let leading_annotations = comments.collect_adjacent_leading(stmt_start);
    let inline_annotations = comments.collect_inline_prompt(stmt_start, stmt_end);
    let mut all_annotations = leading_annotations.clone();
    all_annotations.extend(inline_annotations);

    let has_prompt_annotation = !all_annotations.is_empty();

    // Find var_spec nodes
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            if child.kind() == "var_spec" {
                process_var_spec(
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

/// Process an assignment statement (x = value).
fn process_assignment(
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
    let leading_annotations = comments.collect_adjacent_leading(stmt_start);
    let inline_annotations = comments.collect_inline_prompt(stmt_start, stmt_end);
    let mut all_annotations = leading_annotations.clone();
    all_annotations.extend(inline_annotations);

    // Get left and right sides
    // In Go, assignment_statement has "left" and "right" fields
    let left_list = match node.child_by_field_name("left") {
        Some(n) => n,
        None => return,
    };

    let right_list = match node.child_by_field_name("right") {
        Some(n) => n,
        None => return,
    };

    // Collect identifiers from left side
    // Left side might be a single identifier or an expression_list
    let mut left_idents = Vec::new();
    if left_list.kind() == "identifier" {
        let ident_name = left_list.utf8_text(source.as_bytes()).unwrap_or("");
        left_idents.push((ident_name.to_string(), left_list));
    } else if left_list.kind() == "expression_list" {
        let mut cursor = left_list.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier" {
                    let ident_name = child.utf8_text(source.as_bytes()).unwrap_or("");
                    left_idents.push((ident_name.to_string(), child));
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }

    // Collect values from right side
    // Right side might be a single expression or an expression_list
    let mut right_values = Vec::new();
    if right_list.kind() == "expression_list" {
        let mut cursor2 = right_list.walk();
        if cursor2.goto_first_child() {
            loop {
                let child = cursor2.node();
                right_values.push(child);
                if !cursor2.goto_next_sibling() {
                    break;
                }
            }
        }
    } else {
        right_values.push(right_list);
    }

    // Process each left identifier that's a prompt variable
    for (idx, (ident_name, _ident_node)) in left_idents.iter().enumerate() {
        if scopes.is_prompt_ident(ident_name) {
            // Get corresponding right value (or last if fewer rights than lefts)
            let right_value = if idx < right_values.len() {
                right_values[idx]
            } else if !right_values.is_empty() {
                right_values[right_values.len() - 1]
            } else {
                continue;
            };

            // Check if right side is a string
            if is_string_like(&right_value) {
                // Get annotations (from current statement or from definition)
                let final_annotations = if !all_annotations.is_empty() {
                    all_annotations.clone()
                } else {
                    scopes.get_def_annotation(ident_name).unwrap_or_default()
                };

                // Create prompt from the string
                create_prompt_from_string(
                    &right_value,
                    source,
                    filename,
                    stmt_start,
                    stmt_end,
                    comments,
                    &final_annotations,
                    prompts,
                );
            }
        }
    }
}

/// Process a var_spec node.
#[allow(clippy::too_many_arguments)]
fn process_var_spec(
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
    // Get name node
    let name_node = match node.child_by_field_name("name") {
        Some(n) => n,
        None => return,
    };

    let ident_name = name_node.utf8_text(source.as_bytes()).unwrap_or("");

    // Get value node
    let value_node = match node.child_by_field_name("value") {
        Some(n) => n,
        None => return,
    };

    // Determine if this is a prompt
    let is_prompt = is_prompt_variable(ident_name, has_prompt_annotation, scopes);

    if is_prompt {
        scopes.mark_prompt_ident(ident_name);

        if has_prompt_annotation {
            scopes.store_def_annotation(ident_name, annotations.to_vec());
        }

        // Annotations from comment tracker are already validated to contain @prompt
        // Get annotations (from current statement or from definition)
        let final_annotations = if !annotations.is_empty() {
            annotations.to_vec()
        } else {
            scopes.get_def_annotation(ident_name).unwrap_or_default()
        };

        if is_string_like(&value_node) {
            create_prompt_from_string(
                &value_node,
                source,
                filename,
                stmt_start,
                stmt_end,
                comments,
                &final_annotations,
                prompts,
            );
        } else if value_node.kind() == "binary_expression" {
            // Try to process as concatenation
            if let Some(prompt) = process_concatenation(
                &value_node,
                source,
                filename,
                stmt_start,
                stmt_end,
                comments,
                &final_annotations,
            ) {
                prompts.push(prompt);
            }
        }
    }
}

/// Check if a variable should be treated as a prompt.
fn is_prompt_variable(ident_name: &str, has_annotation: bool, scopes: &ScopeTracker) -> bool {
    ident_name.to_lowercase().contains("prompt")
        || has_annotation
        || scopes.is_prompt_ident(ident_name)
}

/// Check if a node represents a string literal.
fn is_string_like(node: &Node) -> bool {
    matches!(
        node.kind(),
        "interpreted_string_literal" | "raw_string_literal"
    )
}

/// Extract identifiers from an expression_list or identifier.
fn extract_identifiers(node: &Node, source: &str, identifiers: &mut Vec<String>) {
    if node.kind() == "identifier" {
        if let Ok(name) = node.utf8_text(source.as_bytes()) {
            identifiers.push(name.to_string());
        }
        return;
    }

    // Recursively walk children
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            if child.kind() == "identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    identifiers.push(name.to_string());
                }
            } else {
                extract_identifiers(&child, source, identifiers);
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Create a prompt from a string node.
fn create_prompt_from_string(
    string_node: &Node,
    source: &str,
    filename: &str,
    stmt_start: u32,
    stmt_end: u32,
    comments: &CommentTracker,
    annotations: &[PromptAnnotation],
    prompts: &mut Vec<Prompt>,
) {
    // Calculate spans
    let span = span_shape_string_like(string_node, source);
    let content_span = span.inner;

    // Go doesn't have native string interpolation, so vars is always empty
    let vars = Vec::new();

    // Calculate enclosure - use get_any_leading_start to include ANY leading comment (valid or not)
    let enclosure_start = comments
        .get_any_leading_start(stmt_start)
        .unwrap_or(stmt_start);
    let enclosure = (enclosure_start, stmt_end);

    prompts.push(Prompt {
        file: filename.to_string(),
        span,
        enclosure,
        vars,
        annotations: annotations.to_vec(),
        content: vec![PromptContentToken::PromptContentTokenStr(
            PromptContentTokenStr {
                r#type: PromptContentTokenStrTypeStr,
                span: content_span,
            }
        )],
        joint: SpanShape {
            outer: (0, 0),
            inner: (0, 0),
        },
    });
}

/// Represents a segment in a string concatenation expression
#[derive(Debug, Clone)]
enum ConcatSegment {
    String(SpanShape),
    Variable(SpanShape),
    Primitive(SpanShape),
    Other,
}

fn process_concatenation(
    binary_node: &Node,
    source: &str,
    filename: &str,
    stmt_start: u32,
    stmt_end: u32,
    comments: &CommentTracker,
    annotations: &[PromptAnnotation],
) -> Option<Prompt> {
    let operator_node = binary_node.child_by_field_name("operator")?;
    let operator_text = operator_node.utf8_text(source.as_bytes()).ok()?;
    if operator_text != "+" {
        return None;
    }

    let segments = extract_concat_segments(binary_node, source);
    if segments.iter().any(|s| matches!(s, ConcatSegment::Other)) {
        return None;
    }
    if segments.is_empty() {
        return None;
    }

    let outer = (binary_node.start_byte() as u32, binary_node.end_byte() as u32);
    let inner_start = match segments.first() {
        Some(ConcatSegment::String(span)) => span.inner.0,
        Some(ConcatSegment::Variable(span) | ConcatSegment::Primitive(span)) => span.outer.0,
        _ => outer.0,
    };
    let inner_end = match segments.last() {
        Some(ConcatSegment::String(span)) => span.inner.1,
        Some(ConcatSegment::Variable(span) | ConcatSegment::Primitive(span)) => span.outer.1,
        _ => outer.1,
    };
    let span = SpanShape { outer, inner: (inner_start, inner_end) };

    let vars: Vec<PromptVar> = segments
        .iter()
        .filter_map(|seg| match seg {
            ConcatSegment::Variable(var_span) => Some(PromptVar { span: var_span.clone() }),
            _ => None,
        })
        .collect();

    let mut var_idx = 0u32;
    let content = segments
        .iter()
        .map(|seg| match seg {
            ConcatSegment::String(span) => PromptContentToken::PromptContentTokenStr(
                PromptContentTokenStr {
                    r#type: PromptContentTokenStrTypeStr,
                    span: span.inner,
                }
            ),
            ConcatSegment::Variable(span) => {
                let token = PromptContentToken::PromptContentTokenVar(
                    PromptContentTokenVar {
                        r#type: PromptContentTokenVarTypeVar,
                        span: span.inner,
                        index: var_idx,
                    }
                );
                var_idx += 1;
                token
            },
            ConcatSegment::Primitive(span) => PromptContentToken::PromptContentTokenStr(
                PromptContentTokenStr {
                    r#type: PromptContentTokenStrTypeStr,
                    span: span.inner,
                }
            ),
            ConcatSegment::Other => PromptContentToken::PromptContentTokenStr(
                PromptContentTokenStr {
                    r#type: PromptContentTokenStrTypeStr,
                    span: (0, 0),
                }
            ),
        })
        .collect();

    let enclosure_start = comments.get_any_leading_start(stmt_start).unwrap_or(stmt_start);
    Some(Prompt {
        file: filename.to_string(),
        span,
        enclosure: (enclosure_start, stmt_end),
        vars,
        annotations: annotations.to_vec(),
        content,
        joint: SpanShape { outer: (0, 0), inner: (0, 0) },
    })
}

fn extract_concat_segments(node: &Node, source: &str) -> Vec<ConcatSegment> {
    if node.kind() == "binary_expression" {
        if let Some(operator_node) = node.child_by_field_name("operator") {
            if let Ok(operator_text) = operator_node.utf8_text(source.as_bytes()) {
                if operator_text == "+" {
                    let mut segments = Vec::new();
                    if let Some(left) = node.child_by_field_name("left") {
                        segments.extend(extract_concat_segments(&left, source));
                    }
                    if let Some(right) = node.child_by_field_name("right") {
                        segments.extend(extract_concat_segments(&right, source));
                    }
                    return segments;
                }
            }
        }
        return vec![];
    }

    match node.kind() {
        "interpreted_string_literal" | "raw_string_literal" => {
            let span = span_shape_string_like(node, source);
            vec![ConcatSegment::String(span)]
        }
        "identifier" | "call_expression" | "selector_expression" => {
            let outer = (node.start_byte() as u32, node.end_byte() as u32);
            let outer_expanded = expand_to_operators(outer, source);
            vec![ConcatSegment::Variable(SpanShape { outer: outer_expanded, inner: outer })]
        }
        "int_literal" | "float_literal" | "true" | "false" => {
            let outer = (node.start_byte() as u32, node.end_byte() as u32);
            let outer_expanded = expand_to_operators(outer, source);
            vec![ConcatSegment::Primitive(SpanShape { outer: outer_expanded, inner: outer })]
        }
        "slice_literal" | "composite_literal" => vec![ConcatSegment::Other],
        _ => vec![],
    }
}

fn expand_to_operators(span: (u32, u32), source: &str) -> (u32, u32) {
    let (start, end) = span;
    let mut new_start = start;
    let mut new_end = end;
    let code_bytes = source.as_bytes();

    let mut pos = start.saturating_sub(1) as usize;
    while pos > 0 {
        match code_bytes.get(pos) {
            Some(b' ') | Some(b'\t') => pos -= 1,
            Some(b'+') => {
                new_start = pos as u32;
                if pos > 0 && matches!(code_bytes.get(pos - 1), Some(b' ') | Some(b'\t')) {
                    new_start = (pos - 1) as u32;
                }
                break;
            }
            _ => break,
        }
    }

    let mut pos = end as usize;
    while pos < code_bytes.len() {
        match code_bytes.get(pos) {
            Some(b' ') | Some(b'\t') => pos += 1,
            Some(b'+') => {
                new_end = (pos + 1) as u32;
                if pos + 1 < code_bytes.len()
                    && matches!(code_bytes.get(pos + 1), Some(b' ') | Some(b'\t'))
                {
                    new_end = (pos + 2) as u32;
                }
                break;
            }
            _ => break,
        }
    }

    (new_start, new_end)
}

// Format function support

/// Process fmt.Sprintf call: fmt.Sprintf("Hello %s", name)
#[allow(clippy::too_many_arguments)]
fn process_sprintf_call(
    node: &Node,
    source: &str,
    filename: &str,
    stmt_start: u32,
    stmt_end: u32,
    comments: &CommentTracker,
    annotations: &[PromptAnnotation],
) -> Option<Prompt> {
    // Get function being called
    let func_node = node.child_by_field_name("function")?;
    
    // Check if it's fmt.Sprintf or fmt.Printf
    let func_text = func_node.utf8_text(source.as_bytes()).ok()?;
    if func_text != "fmt.Sprintf" && func_text != "fmt.Printf" {
        return None;
    }
    
    // Get arguments
    let args_node = node.child_by_field_name("arguments")?;
    let mut arg_nodes = Vec::new();
    let mut cursor = args_node.walk();
    for child in args_node.children(&mut cursor) {
        if child.is_named() && child.kind() != "," {
            arg_nodes.push(child);
        }
    }
    
    if arg_nodes.is_empty() {
        return None;
    }
    
    // First argument should be the format string
    let format_str_node = arg_nodes[0];
    if !is_string_like(&format_str_node) {
        return None;
    }
    
    // Parse format string
    let format_str_span = span_shape_string_like(&format_str_node, source);
    let format_str_content = &source[format_str_span.inner.0 as usize..format_str_span.inner.1 as usize];
    let placeholders = parse_printf_placeholders(format_str_content);
    
    if placeholders.is_empty() {
        return None;
    }
    
    // Build vars from remaining arguments
    let mut vars = Vec::new();
    for (arg_idx, arg_node) in arg_nodes.iter().skip(1).enumerate() {
        if arg_idx >= placeholders.len() {
            break;
        }
        
        let arg_start = arg_node.start_byte() as u32;
        let arg_end = arg_node.end_byte() as u32;
        vars.push(PromptVar {
            span: SpanShape {
                outer: (arg_start, arg_end),
                inner: (arg_start, arg_end),
            },
        });
    }
    
    // Build content tokens
    let mut content = Vec::new();
    let mut pos = 0usize;
    let format_inner_start = format_str_span.inner.0;
    
    for (placeholder_idx, (start, end)) in placeholders.iter().enumerate() {
        // Add string token before placeholder
        if pos < *start {
            content.push(PromptContentToken::PromptContentTokenStr(
                PromptContentTokenStr {
                    r#type: PromptContentTokenStrTypeStr,
                    span: (format_inner_start + pos as u32, format_inner_start + *start as u32),
                },
            ));
        }
        
        // Add var token for placeholder (if we have a corresponding arg)
        if placeholder_idx < vars.len() {
            content.push(PromptContentToken::PromptContentTokenVar(
                PromptContentTokenVar {
                    r#type: PromptContentTokenVarTypeVar,
                    span: (format_inner_start + *start as u32, format_inner_start + *end as u32),
                    index: placeholder_idx as u32,
                },
            ));
        }
        
        pos = *end;
    }
    
    // Add trailing string token
    if pos < format_str_content.len() {
        content.push(PromptContentToken::PromptContentTokenStr(
            PromptContentTokenStr {
                r#type: PromptContentTokenStrTypeStr,
                span: (format_inner_start + pos as u32, format_str_span.inner.1),
            },
        ));
    }
    
    let enclosure_start = comments.get_any_leading_start(stmt_start).unwrap_or(stmt_start);
    
    Some(Prompt {
        file: filename.to_string(),
        span: SpanShape {
            outer: (node.start_byte() as u32, node.end_byte() as u32),
            inner: format_str_span.inner,
        },
        enclosure: (enclosure_start, stmt_end),
        vars,
        annotations: annotations.to_vec(),
        content,
        joint: SpanShape {
            outer: (0, 0),
            inner: (0, 0),
        },
    })
}

/// Parse printf-style placeholders: %s, %d, %v, %f, etc.
fn parse_printf_placeholders(format_str: &str) -> Vec<(usize, usize)> {
    let mut placeholders = Vec::new();
    let chars: Vec<char> = format_str.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        if chars[i] == '%' {
            // Check for escaped %%
            if i + 1 < chars.len() && chars[i + 1] == '%' {
                i += 2;
                continue;
            }
            
            let start = i;
            i += 1;
            
            // Skip flags, width, precision
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '-' || chars[i] == '+' || chars[i] == ' ' || chars[i] == '#' || chars[i] == '.' || chars[i] == '*') {
                i += 1;
            }
            
            // Get format specifier
            if i < chars.len() && (chars[i].is_alphabetic() || chars[i] == '%') {
                i += 1;
                placeholders.push((start, i));
                continue;
            }
        }
        i += 1;
    }
    
    placeholders
}

/// Process array/slice: []string{"Hello ", user, "!"}
#[allow(clippy::too_many_arguments)]
fn process_array(
    node: &Node,
    source: &str,
    filename: &str,
    stmt_start: u32,
    stmt_end: u32,
    comments: &CommentTracker,
    annotations: &[PromptAnnotation],
) -> Option<Prompt> {
    // Extract array elements using recursive helper (like strings.Join)
    fn extract_elements(
        node: &Node,
        source: &str,
        vars: &mut Vec<PromptVar>,
        content: &mut Vec<PromptContentToken>,
        var_idx: &mut u32,
    ) {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                match child.kind() {
                    "interpreted_string_literal" | "raw_string_literal" => {
                        let span = span_shape_string_like(&child, source);
                        content.push(PromptContentToken::PromptContentTokenStr(
                            PromptContentTokenStr {
                                r#type: PromptContentTokenStrTypeStr,
                                span: span.inner,
                            },
                        ));
                    }
                    "identifier" => {
                        let outer = (child.start_byte() as u32, child.end_byte() as u32);
                        vars.push(PromptVar {
                            span: SpanShape { outer, inner: outer },
                        });
                        content.push(PromptContentToken::PromptContentTokenVar(
                            PromptContentTokenVar {
                                r#type: PromptContentTokenVarTypeVar,
                                span: outer,
                                index: *var_idx,
                            },
                        ));
                        *var_idx += 1;
                    }
                    "call_expression" => {
                        let outer = (child.start_byte() as u32, child.end_byte() as u32);
                        vars.push(PromptVar {
                            span: SpanShape { outer, inner: outer },
                        });
                        content.push(PromptContentToken::PromptContentTokenVar(
                            PromptContentTokenVar {
                                r#type: PromptContentTokenVarTypeVar,
                                span: outer,
                                index: *var_idx,
                            },
                        ));
                        *var_idx += 1;
                    }
                    "{" | "}" | "," => {
                        // Skip delimiters
                    }
                    _ => {
                        // Recurse into other nodes (like literal_value, composite_literal, etc.)
                        extract_elements(&child, source, vars, content, var_idx);
                    }
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }

    let mut vars = Vec::new();
    let mut content = Vec::new();
    let mut var_idx = 0u32;

    extract_elements(node, source, &mut vars, &mut content, &mut var_idx);

    let outer = (node.start_byte() as u32, node.end_byte() as u32);
    
    // Find literal_value for inner span calculation
    let mut literal_start = None;
    let mut literal_end = None;
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            if child.kind() == "literal_value" {
                literal_start = Some(child.start_byte() as u32);
                literal_end = Some(child.end_byte() as u32);
                break;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    
    let inner_start = literal_start.map(|s| s + 1).unwrap_or(outer.0 + 1);
    let inner_end = literal_end.map(|e| e - 1).unwrap_or(outer.1 - 1);
    let span = SpanShape {
        outer,
        inner: (inner_start, inner_end),
    };

    let enclosure_start = comments
        .get_any_leading_start(stmt_start)
        .unwrap_or(stmt_start);

    Some(Prompt {
        file: filename.to_string(),
        span,
        enclosure: (enclosure_start, stmt_end),
        vars,
        annotations: annotations.to_vec(),
        content,
        joint: SpanShape {
            outer: (0, 0),
            inner: (0, 0),
        },
    })
}

/// Process strings.Join: strings.Join([]string{"Hello", user, "!"}, "\n")
#[allow(clippy::too_many_arguments)]
fn process_strings_join(
    node: &Node,
    source: &str,
    filename: &str,
    stmt_start: u32,
    stmt_end: u32,
    comments: &CommentTracker,
    annotations: &[PromptAnnotation],
) -> Option<Prompt> {
    // Check if it's a selector_expression (package.function)
    let mut func_node = None;
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            if child.kind() == "selector_expression" {
                func_node = Some(child);
                break;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    if let Some(func) = func_node {
        // Check package.function is strings.Join
        let func_text = func.utf8_text(source.as_bytes()).ok()?;
        if func_text != "strings.Join" {
            return None;
        }
    } else {
        return None;
    }

    // Find arguments
    let mut args_node = None;
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            if child.kind() == "argument_list" {
                args_node = Some(child);
                break;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    let args = args_node?;

    // Extract array (first arg) and separator (second arg)
    let mut array_node = None;
    let mut sep_node = None;
    let mut arg_idx = 0;

    let mut cursor = args.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            if child.kind() == "composite_literal" || child.kind() == "slice_literal" {
                if arg_idx == 0 {
                    array_node = Some(child);
                }
                arg_idx += 1;
            } else if child.kind() == "interpreted_string_literal" || child.kind() == "raw_string_literal" {
                if arg_idx == 1 {
                    sep_node = Some(child);
                }
                arg_idx += 1;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    let array = array_node?;
    let joint = if let Some(sep) = sep_node {
        span_shape_string_like(&sep, source)
    } else {
        SpanShape {
            outer: (0, 0),
            inner: (0, 0),
        }
    };

    // Extract array elements using recursive helper (like PHP)
    fn extract_elements_with_joints(
        node: &Node,
        source: &str,
        vars: &mut Vec<PromptVar>,
        content: &mut Vec<PromptContentToken>,
        var_idx: &mut u32,
        joint: &SpanShape,
        first: &mut bool,
    ) {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                match child.kind() {
                    "interpreted_string_literal" | "raw_string_literal" => {
                        // Insert joint before this element (not before first)
                        if !*first && (joint.outer.0 != 0 || joint.outer.1 != 0) {
                            content.push(PromptContentToken::PromptContentTokenJoint(
                                PromptContentTokenJoint {
                                    r#type: PromptContentTokenJointTypeJoint,
                                },
                            ));
                        }
                        *first = false;
                        let span = span_shape_string_like(&child, source);
                        content.push(PromptContentToken::PromptContentTokenStr(
                            PromptContentTokenStr {
                                r#type: PromptContentTokenStrTypeStr,
                                span: span.inner,
                            },
                        ));
                    }
                    "identifier" => {
                        // Insert joint before this element (not before first)
                        if !*first && (joint.outer.0 != 0 || joint.outer.1 != 0) {
                            content.push(PromptContentToken::PromptContentTokenJoint(
                                PromptContentTokenJoint {
                                    r#type: PromptContentTokenJointTypeJoint,
                                },
                            ));
                        }
                        *first = false;
                        let outer = (child.start_byte() as u32, child.end_byte() as u32);
                        vars.push(PromptVar {
                            span: SpanShape { outer, inner: outer },
                        });
                        content.push(PromptContentToken::PromptContentTokenVar(
                            PromptContentTokenVar {
                                r#type: PromptContentTokenVarTypeVar,
                                span: outer,
                                index: *var_idx,
                            },
                        ));
                        *var_idx += 1;
                    }
                    "{" | "}" | "," => {
                        // Skip delimiters
                    }
                    _ => {
                        // Recurse into other nodes (like literal_value, composite_literal, etc.)
                        extract_elements_with_joints(&child, source, vars, content, var_idx, joint, first);
                    }
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }

    let mut vars = Vec::new();
    let mut content = Vec::new();
    let mut var_idx = 0u32;
    let mut first = true;

    extract_elements_with_joints(&array, source, &mut vars, &mut content, &mut var_idx, &joint, &mut first);

    let outer = (node.start_byte() as u32, node.end_byte() as u32);
    let array_span = (array.start_byte() as u32, array.end_byte() as u32);
    
    // Find literal_value for inner span calculation
    let mut literal_start = None;
    let mut literal_end = None;
    let mut cursor = array.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            if child.kind() == "literal_value" {
                literal_start = Some(child.start_byte() as u32);
                literal_end = Some(child.end_byte() as u32);
                break;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    
    let inner_start = literal_start.map(|s| s + 1).unwrap_or(array_span.0 + 1);
    let inner_end = literal_end.map(|e| e - 1).unwrap_or(array_span.1 - 1);
    let span = SpanShape {
        outer,
        inner: (inner_start, inner_end),
    };

    let enclosure_start = comments
        .get_any_leading_start(stmt_start)
        .unwrap_or(stmt_start);

    Some(Prompt {
        file: filename.to_string(),
        span,
        enclosure: (enclosure_start, stmt_end),
        vars,
        annotations: annotations.to_vec(),
        content,
        joint,
    })
}
