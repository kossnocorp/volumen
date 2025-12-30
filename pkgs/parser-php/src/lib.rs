mod comments;
mod scope;
mod spans;

use comments::CommentTracker;
use scope::ScopeTracker;
use spans::span_shape_string_like;
use tree_sitter::{Node, Parser, Tree};
pub use volumen_parser_core::VolumenParser;
use volumen_types::*;

pub struct ParserPhp {}

impl VolumenParser for ParserPhp {
    fn parse(source: &str, filename: &str) -> ParseResult {
        // Initialize Tree-sitter parser
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_php::LANGUAGE_PHP.into())
            .expect("Failed to load PHP grammar");

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

    // Handle scope boundaries (function, class, method)
    if kind == "function_definition" || kind == "class_declaration" || kind == "method_declaration"
    {
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

    // Handle expression statements that contain assignments
    if kind == "expression_statement" {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            let child = cursor.node();
            if child.kind() == "assignment_expression" {
                process_assignment(&child, source, filename, comments, scopes, prompts);
                return;
            }
        }
    }

    // Handle assignments directly
    if kind == "assignment_expression" {
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

/// Process an assignment node to extract prompts.
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

    // Handle different assignment patterns
    match left.kind() {
        "variable_name" | "simple_variable" => {
            // Simple assignment: $var = value
            let ident_name = left.utf8_text(source.as_bytes()).unwrap_or("");
            process_identifier_assignment(
                ident_name,
                &right,
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
        "list_literal" => {
            // Multi-assignment: list($a, $b) = values or [$a, $b] = values
            process_multi_assignment(
                &left,
                &right,
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
        _ => {}
    }
}

/// Process a simple identifier assignment.
#[allow(clippy::too_many_arguments)]
fn process_identifier_assignment(
    ident_name: &str,
    right: &Node,
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
    // Determine if this is a prompt
    let is_prompt = is_prompt_variable(ident_name, has_prompt_annotation, scopes);

    if is_prompt {
        // Mark as prompt identifier
        scopes.mark_prompt_ident(ident_name);

        // Store definition annotations if this is annotated
        if has_prompt_annotation {
            scopes.store_def_annotation(ident_name, annotations.to_vec());
        }

        // Check if it's a chained assignment ($a = $b = "value")
        if right.kind() == "assignment_expression" {
            // Get the left side of the chained assignment to mark it as prompt too
            if let Some(chained_left) = right.child_by_field_name("left") {
                let chained_ident = chained_left.utf8_text(source.as_bytes()).unwrap_or("");
                scopes.mark_prompt_ident(chained_ident);
                
                // Store definition annotations for the chained variable too
                if has_prompt_annotation {
                    scopes.store_def_annotation(chained_ident, annotations.to_vec());
                }
            }
            
            // Get the actual value (rightmost in the chain)
            if let Some(chained_right) = right.child_by_field_name("right") {
                // Recursively process the chained assignment to create prompts for all variables
                process_identifier_assignment(
                    ident_name,
                    &chained_right,
                    has_prompt_annotation,
                    annotations,
                    stmt_start,
                    stmt_end,
                    source,
                    filename,
                    comments,
                    scopes,
                    prompts,
                );
                
                // Also process the middle variable(s) in the chain
                if let Some(chained_left) = right.child_by_field_name("left") {
                    let chained_ident = chained_left.utf8_text(source.as_bytes()).unwrap_or("");
                    process_identifier_assignment(
                        chained_ident,
                        &chained_right,
                        has_prompt_annotation,
                        annotations,
                        stmt_start,
                        stmt_end,
                        source,
                        filename,
                        comments,
                        scopes,
                        prompts,
                    );
                }
            }
        } 
        // Check if it's a string or binary expression
        else if is_string_like(right) {
            // Annotations from comment tracker are already validated to contain @prompt
            // Get annotations (from current statement or from definition)
            let final_annotations = if !annotations.is_empty() {
                annotations.to_vec()
            } else {
                scopes.get_def_annotation(ident_name).unwrap_or_default()
            };

            // Create prompt
            create_prompt_from_string(
                right,
                source,
                filename,
                stmt_start,
                stmt_end,
                comments,
                &final_annotations,
                prompts,
            );
        } else if right.kind() == "binary_expression" {
            // Get annotations
            let final_annotations = if !annotations.is_empty() {
                annotations.to_vec()
            } else {
                scopes.get_def_annotation(ident_name).unwrap_or_default()
            };

            // Try to process as concatenation
            if let Some(prompt) = process_concatenation(
                right,
                source,
                filename,
                stmt_start,
                stmt_end,
                comments,
                &final_annotations,
            ) {
                prompts.push(prompt);
            }
        } else if right.kind() == "function_call_expression" 
            || right.kind() == "member_call_expression" 
            || right.kind() == "scoped_call_expression" {
            // Get annotations
            let final_annotations = if !annotations.is_empty() {
                annotations.to_vec()
            } else {
                scopes.get_def_annotation(ident_name).unwrap_or_default()
            };

            // Try to process as implode first
            if let Some(prompt) = process_implode_call(
                right,
                source,
                filename,
                stmt_start,
                stmt_end,
                comments,
                &final_annotations,
            ) {
                prompts.push(prompt);
            // Then try sprintf/printf
            } else if let Some(prompt) = process_sprintf_call(
                right,
                source,
                filename,
                stmt_start,
                stmt_end,
                comments,
                &final_annotations,
            ) {
                prompts.push(prompt);
            }
        } else if right.kind() == "array_creation_expression" {
            // Get annotations
            let final_annotations = if !annotations.is_empty() {
                annotations.to_vec()
            } else {
                scopes.get_def_annotation(ident_name).unwrap_or_default()
            };

            // Try to process as array
            if let Some(prompt) = process_array(
                right,
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

/// Process multi-assignment (destructuring).
#[allow(clippy::too_many_arguments)]
fn process_multi_assignment(
    left: &Node,
    right: &Node,
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
    // Extract identifiers from left side
    let mut identifiers = Vec::new();
    extract_identifiers(left, source, &mut identifiers);

    // Extract value ranges from right side
    let mut value_ranges = Vec::new();
    extract_value_ranges(right, &mut value_ranges);

    // Match identifiers with values
    for (i, ident_name) in identifiers.iter().enumerate() {
        if let Some((start, end, kind)) = value_ranges.get(i) {
            let is_prompt = is_prompt_variable(ident_name, has_prompt_annotation, scopes);

            if is_prompt && (*kind == "string" || *kind == "encapsed_string") {
                scopes.mark_prompt_ident(ident_name);

                // Create prompt from range
                create_prompt_from_range(
                    *start,
                    *end,
                    source,
                    filename,
                    stmt_start,
                    stmt_end,
                    comments,
                    annotations,
                    prompts,
                );
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
        "string" | "encapsed_string" | "string_value" | "heredoc" | "nowdoc" | "heredoc_body"
    )
}

/// Extract identifiers from a list_literal or similar pattern.
fn extract_identifiers(node: &Node, source: &str, identifiers: &mut Vec<String>) {
    let kind = node.kind();

    if kind == "variable_name" || kind == "simple_variable" {
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
            extract_identifiers(&child, source, identifiers);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Extract value ranges from the right side of an assignment.
fn extract_value_ranges(node: &Node, values: &mut Vec<(u32, u32, &'static str)>) {
    let kind = node.kind();

    if kind == "string" || kind == "encapsed_string" || kind == "string_value" {
        values.push((node.start_byte() as u32, node.end_byte() as u32, kind));
        return;
    }

    if kind == "array_creation_expression" {
        // Walk children to find string elements
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                extract_value_ranges(&child, values);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        return;
    }

    // For other nodes, recursively check children
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            if is_string_like(&child) {
                extract_value_ranges(&child, values);
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

    // Extract variables if interpolated string
    let vars = spans::extract_interpolation_vars(string_node, source);

    // Build content tokens
    let content = build_content_tokens(&span, &vars);

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
        content,
        joint: SpanShape {
            outer: (0, 0),
            inner: (0, 0),
        },
    });
}

/// Build content tokens from span and variables.
/// For prompts without variables, returns a single str token.
/// For prompts with variables, returns interleaved str and var tokens.
fn build_content_tokens(span: &SpanShape, vars: &[PromptVar]) -> Vec<PromptContentToken> {
    if vars.is_empty() {
        // Simple case: single str token
        return vec![PromptContentToken::PromptContentTokenStr(
            PromptContentTokenStr {
                r#type: PromptContentTokenStrTypeStr,
                span: span.inner,
            },
        )];
    }

    let mut tokens = Vec::new();
    let mut pos = span.inner.0;

    for (var_idx, var) in vars.iter().enumerate() {
        // Add str token before variable (if any content)
        if pos < var.span.outer.0 {
            tokens.push(PromptContentToken::PromptContentTokenStr(
                PromptContentTokenStr {
                    r#type: PromptContentTokenStrTypeStr,
                    span: (pos, var.span.outer.0),
                },
            ));
        }

        // Add var token
        tokens.push(PromptContentToken::PromptContentTokenVar(
            PromptContentTokenVar {
                r#type: PromptContentTokenVarTypeVar,
                span: var.span.outer,
                index: var_idx as u32,
            },
        ));

        pos = var.span.outer.1;
    }

    // Add trailing str token (if any content)
    if pos < span.inner.1 {
        tokens.push(PromptContentToken::PromptContentTokenStr(
            PromptContentTokenStr {
                r#type: PromptContentTokenStrTypeStr,
                span: (pos, span.inner.1),
            },
        ));
    }

    tokens
}

/// Create a prompt from a byte range.
#[allow(clippy::too_many_arguments)]
fn create_prompt_from_range(
    start: u32,
    end: u32,
    source: &str,
    filename: &str,
    stmt_start: u32,
    stmt_end: u32,
    comments: &CommentTracker,
    annotations: &[PromptAnnotation],
    prompts: &mut Vec<Prompt>,
) {
    // Create a temporary node-like structure for span calculation
    let exp = &source[start as usize..end as usize];

    // For simple strings, outer and inner are close
    let span = SpanShape {
        outer: (start, end),
        inner: (
            start + 1,             // Skip opening quote
            end.saturating_sub(1), // Skip closing quote
        ),
    };

    // No variables in range-based prompts (used for multi-assignment)
    let vars = Vec::new();

    // Build content tokens
    let content = build_content_tokens(&span, &vars);

    // Calculate enclosure
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
        content,
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
    // Check if this is a concatenation binary expression (. operator in PHP)
    let operator = binary_node.child_by_field_name("operator")?;
    let operator_text = operator.utf8_text(source.as_bytes()).ok()?;
    if operator_text != "." {
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
        if let Some(operator) = node.child_by_field_name("operator") {
            if let Ok(operator_text) = operator.utf8_text(source.as_bytes()) {
                if operator_text == "." {
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
        "string" | "encapsed_string" | "heredoc" | "nowdoc" => {
            let span = span_shape_string_like(node, source);
            vec![ConcatSegment::String(span)]
        }
        "variable_name" | "name" | "member_access_expression" | "scoped_property_access_expression"
        | "function_call_expression" | "member_call_expression" | "scoped_call_expression" => {
            let outer = (node.start_byte() as u32, node.end_byte() as u32);
            let outer_expanded = expand_to_operators(outer, source);
            vec![ConcatSegment::Variable(SpanShape { outer: outer_expanded, inner: outer })]
        }
        "integer" | "float" | "boolean" | "true" | "false" => {
            let outer = (node.start_byte() as u32, node.end_byte() as u32);
            let outer_expanded = expand_to_operators(outer, source);
            vec![ConcatSegment::Primitive(SpanShape { outer: outer_expanded, inner: outer })]
        }
        "array_creation_expression" | "object_creation_expression" => vec![ConcatSegment::Other],
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
            Some(b'.') => {
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
            Some(b'.') => {
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

/// Process sprintf/printf function call: sprintf("Hello %s", $name)
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
    // Get function name (try both "function" and "name" fields)
    let name_node = node.child_by_field_name("function")
        .or_else(|| node.child_by_field_name("name"))?;
    let func_name = name_node.utf8_text(source.as_bytes()).ok()?;
    
    // Check if it's sprintf or printf
    if func_name != "sprintf" && func_name != "printf" {
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
    
    // The argument might be wrapped, check if it or its child is string-like
    let actual_string_node = if is_string_like(&format_str_node) {
        format_str_node
    } else {
        // Try to find a string child
        let mut cursor = format_str_node.walk();
        let mut found = None;
        for child in format_str_node.children(&mut cursor) {
            if is_string_like(&child) {
                found = Some(child);
                break;
            }
        }
        found?
    };
    
    // Parse format string
    let format_str_span = span_shape_string_like(&actual_string_node, source);
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

/// Parse printf-style placeholders: %s, %d, %f, etc.
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

/// Process an array: ["Hello ", $user, "!"]
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
    // For PHP, the array_creation_expression might have the structure:
    // array_creation_expression -> [ -> array_initializer -> elements
    // We need to find the array_initializer or iterate deeply
    
    // Helper function to recursively extract strings and variables
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
                    "string" | "encapsed_string" => {
                        let span = span_shape_string_like(&child, source);
                        content.push(PromptContentToken::PromptContentTokenStr(
                            PromptContentTokenStr {
                                r#type: PromptContentTokenStrTypeStr,
                                span: span.inner,
                            },
                        ));
                    }
                    "variable_name" | "simple_variable" => {
                        let outer = (child.start_byte() as u32, child.end_byte() as u32);
                        vars.push(PromptVar {
                            span: SpanShape {
                                outer,
                                inner: outer,
                            },
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
                    "function_call_expression" | "member_call_expression" => {
                        let outer = (child.start_byte() as u32, child.end_byte() as u32);
                        vars.push(PromptVar {
                            span: SpanShape {
                                outer,
                                inner: outer,
                            },
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
                    "[" | "]" | "," | "array" | "(" | ")" => {
                        // Skip delimiters
                    }
                    _ => {
                        // Recurse into other nodes (like array_element_initializer)
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
    let inner = (outer.0 + 1, outer.1.saturating_sub(1));
    let span = SpanShape { outer, inner };

    let enclosure_start = comments
        .get_any_leading_start(stmt_start)
        .unwrap_or(stmt_start);
    let enclosure = (enclosure_start, stmt_end);

    Some(Prompt {
        file: filename.to_string(),
        span,
        enclosure,
        vars,
        annotations: annotations.to_vec(),
        content,
        joint: SpanShape {
            outer: (0, 0),
            inner: (0, 0),
        },
    })
}

/// Process an implode call: implode("\n", ["Hello", $user, "!"])
#[allow(clippy::too_many_arguments)]
fn process_implode_call(
    node: &Node,
    source: &str,
    filename: &str,
    stmt_start: u32,
    stmt_end: u32,
    comments: &CommentTracker,
    annotations: &[PromptAnnotation],
) -> Option<Prompt> {
    // Get function name (try both "function" and "name" fields)
    let name_node = node.child_by_field_name("function")
        .or_else(|| node.child_by_field_name("name"))?;
    let func_text = name_node.utf8_text(source.as_bytes()).ok()?;
    if func_text != "implode" && func_text != "join" {
        return None;
    }

    // Get arguments
    let args = node.child_by_field_name("arguments")?;

    // Extract separator (first arg) and array (second arg)
    let mut sep_node = None;
    let mut array_node = None;
    let mut arg_count = 0;

    let mut cursor = args.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            
            // PHP tree-sitter wraps arguments in "argument" nodes
            // We need to look inside them
            if child.kind() == "argument" {
                // Look at the first child of the argument node
                let mut arg_cursor = child.walk();
                if arg_cursor.goto_first_child() {
                    let arg_value = arg_cursor.node();
                    if arg_value.kind() == "string" || arg_value.kind() == "encapsed_string" {
                        if arg_count == 0 {
                            sep_node = Some(arg_value);
                        }
                        arg_count += 1;
                    } else if arg_value.kind() == "array_creation_expression" {
                        if arg_count == 1 {
                            array_node = Some(arg_value);
                        }
                        arg_count += 1;
                    }
                }
            } else if child.kind() == "string" || child.kind() == "encapsed_string" {
                // Direct string (fallback)
                if arg_count == 0 {
                    sep_node = Some(child);
                }
                arg_count += 1;
            } else if child.kind() == "array_creation_expression" {
                // Direct array (fallback)
                if arg_count == 1 {
                    array_node = Some(child);
                }
                arg_count += 1;
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

    // Extract array elements using recursive helper
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
                    "string" | "encapsed_string" => {
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
                    "variable_name" | "simple_variable" => {
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
                            span: SpanShape {
                                outer,
                                inner: outer,
                            },
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
                    "[" | "]" | "," | "array" | "(" | ")" => {
                        // Skip delimiters
                    }
                    _ => {
                        // Recurse into other nodes
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
    let inner = (array_span.0 + 1, array_span.1.saturating_sub(1));
    let span = SpanShape { outer, inner };

    let enclosure_start = comments
        .get_any_leading_start(stmt_start)
        .unwrap_or(stmt_start);
    let enclosure = (enclosure_start, stmt_end);

    Some(Prompt {
        file: filename.to_string(),
        span,
        enclosure,
        vars,
        annotations: annotations.to_vec(),
        content,
        joint,
    })
}
