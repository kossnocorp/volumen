mod comments;
mod scope;
mod spans;

use comments::CommentTracker;
use scope::ScopeTracker;
use spans::span_shape_string_like;
use tree_sitter::{Node, Parser, Tree};
pub use volumen_parser_core::VolumenParser;
use volumen_types::*;

pub struct ParserCs {}

impl VolumenParser for ParserCs {
    fn parse(source: &str, filename: &str) -> ParseResult {
        // Initialize Tree-sitter parser
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_c_sharp::LANGUAGE.into())
            .expect("Failed to load C# grammar");

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

    // Handle scope boundaries (method, class, struct)
    if kind == "method_declaration"
        || kind == "class_declaration"
        || kind == "struct_declaration"
        || kind == "constructor_declaration"
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

    // Handle global statements (C# 9.0+ top-level statements)
    if kind == "global_statement" {
        // Process children of global_statement which may contain local_declaration_statement
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
        return;
    }

    // Handle local declarations
    if kind == "local_declaration_statement" {
        process_local_declaration(node, source, filename, comments, scopes, prompts);
        return;
    }

    // Handle field declarations
    if kind == "field_declaration" {
        process_field_declaration(node, source, filename, comments, scopes, prompts);
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

/// Process a local declaration statement.
fn process_local_declaration(
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

    // Find variable_declarator nodes
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            if child.kind() == "variable_declaration" {
                // Process variable_declaration's variable_declarators
                let mut var_cursor = child.walk();
                if var_cursor.goto_first_child() {
                    loop {
                        let var_child = var_cursor.node();
                        if var_child.kind() == "variable_declarator" {
                            process_variable_declarator(
                                &var_child,
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
                        if !var_cursor.goto_next_sibling() {
                            break;
                        }
                    }
                }
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Process a field declaration (class-level variable).
fn process_field_declaration(
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

    // Find variable_declarator nodes in variable_declaration
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            if child.kind() == "variable_declaration" {
                // Process variable_declaration's variable_declarators
                let mut var_cursor = child.walk();
                if var_cursor.goto_first_child() {
                    loop {
                        let var_child = var_cursor.node();
                        if var_child.kind() == "variable_declarator" {
                            process_variable_declarator(
                                &var_child,
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
                        if !var_cursor.goto_next_sibling() {
                            break;
                        }
                    }
                }
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Process a variable_declarator node.
#[allow(clippy::too_many_arguments)]
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
    // Get identifier node
    let ident_node = match node.child_by_field_name("name") {
        Some(n) => n,
        None => return,
    };

    let ident_name = ident_node.utf8_text(source.as_bytes()).unwrap_or("");

    // Get value node - in C# it's not a named field, it's the last child after '='
    // Pattern: identifier = value
    let value_node = {
        let mut found_value = None;
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                // Skip identifier (name) and '=' token, take the first expression-like node
                if child.kind() != "identifier" && child.kind() != "=" {
                    found_value = Some(child);
                    break;
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        match found_value {
            Some(n) => n,
            None => return,
        }
    };

    // Determine if this is a prompt
    let is_prompt = is_prompt_variable(ident_name, has_prompt_annotation, scopes);

    if is_prompt {
        // Mark as prompt identifier
        scopes.mark_prompt_ident(ident_name);

        // Store definition annotations if this is annotated
        if has_prompt_annotation {
            scopes.store_def_annotation(ident_name, annotations.to_vec());
        }

        // Check if it's a string or binary expression
        if is_string_like(&value_node) {
            // Annotations from comment tracker are already validated to contain @prompt
            // Get annotations (from current statement or from definition)
            let final_annotations = if !annotations.is_empty() {
                annotations.to_vec()
            } else {
                scopes.get_def_annotation(ident_name).unwrap_or_default()
            };

            // Create prompt
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
            // Get annotations
            let final_annotations = if !annotations.is_empty() {
                annotations.to_vec()
            } else {
                scopes.get_def_annotation(ident_name).unwrap_or_default()
            };

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
        } else if value_node.kind() == "invocation_expression" {
            // Get annotations
            let final_annotations = if !annotations.is_empty() {
                annotations.to_vec()
            } else {
                scopes.get_def_annotation(ident_name).unwrap_or_default()
            };

            // Try to process as String.Format
            if let Some(prompt) = process_string_format(
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

/// Check if a node represents a string literal or interpolated string.
fn is_string_like(node: &Node) -> bool {
    matches!(
        node.kind(),
        "string_literal" | "interpolated_string_expression" | "verbatim_string_literal"
    )
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

    // Extract variables if interpolated string (C# has string interpolation)
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

/// Represents a segment in a string concatenation expression
#[derive(Debug, Clone)]
enum ConcatSegment {
    /// String literal segment with its span
    String(SpanShape),
    /// Variable identifier segment with its span
    Variable(SpanShape),
    /// Primitive value (number, boolean) that should be stringified
    Primitive(SpanShape),
    /// Other expression types (objects, arrays, etc.) that we don't handle
    Other,
}

/// Process a binary expression as concatenation
fn process_concatenation(
    binary_node: &Node,
    source: &str,
    filename: &str,
    stmt_start: u32,
    stmt_end: u32,
    comments: &CommentTracker,
    annotations: &[PromptAnnotation],
) -> Option<Prompt> {
    // Check if this is an addition binary expression (+ operator)
    let operator_node = binary_node.child_by_field_name("operator")?;
    let operator_text = operator_node.utf8_text(source.as_bytes()).ok()?;
    if operator_text != "+" {
        return None;
    }

    // Extract segments from the binary expression tree
    let segments = extract_concat_segments(binary_node, source);

    // Check if we have any non-string/non-identifier segments (objects, arrays, etc.)
    // In that case, don't treat this as a prompt
    if segments.iter().any(|s| matches!(s, ConcatSegment::Other)) {
        return None;
    }

    if segments.is_empty() {
        return None;
    }

    // Build synthetic span for the concatenated result
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
    let inner = (inner_start, inner_end);
    let span = SpanShape { outer, inner };

    // Extract vars (variables only, not primitives)
    let vars: Vec<PromptVar> = segments
        .iter()
        .filter_map(|seg| match seg {
            ConcatSegment::Variable(var_span) => Some(PromptVar {
                span: var_span.clone(),
            }),
            _ => None,
        })
        .collect();

    // Build content tokens from segments
    let content = build_concat_content_tokens(&segments);

    // Calculate enclosure
    let enclosure_start = comments.get_any_leading_start(stmt_start).unwrap_or(stmt_start);
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

/// Extract concatenation segments from a binary expression
fn extract_concat_segments(node: &Node, source: &str) -> Vec<ConcatSegment> {
    if node.kind() == "binary_expression" {
        // Check if it's a + operator
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
        "string_literal" | "verbatim_string_literal" | "interpolated_string_expression" => {
            let span = span_shape_string_like(node, source);
            vec![ConcatSegment::String(span)]
        }
        "identifier" | "invocation_expression" | "member_access_expression" | "element_access_expression" => {
            let outer = (node.start_byte() as u32, node.end_byte() as u32);
            let inner = outer;
            let outer_expanded = expand_to_operators(outer, source);
            vec![ConcatSegment::Variable(SpanShape {
                outer: outer_expanded,
                inner,
            })]
        }
        "integer_literal" | "real_literal" | "boolean_literal" | "true_literal" | "false_literal" => {
            let outer = (node.start_byte() as u32, node.end_byte() as u32);
            let inner = outer;
            let outer_expanded = expand_to_operators(outer, source);
            vec![ConcatSegment::Primitive(SpanShape {
                outer: outer_expanded,
                inner,
            })]
        }
        // Objects, arrays, new expressions, etc. - mark as "other" to skip prompt detection
        "object_creation_expression" | "array_creation_expression" | "implicit_array_creation_expression"
        | "initializer_expression" | "anonymous_object_creation_expression" => {
            vec![ConcatSegment::Other]
        }
        _ => vec![],
    }
}

/// Expand span to include surrounding operators and spaces
fn expand_to_operators(span: (u32, u32), source: &str) -> (u32, u32) {
    let (start, end) = span;
    let mut new_start = start;
    let mut new_end = end;

    let code_bytes = source.as_bytes();

    // Expand left to include " + "
    let mut pos = start.saturating_sub(1) as usize;
    while pos > 0 {
        match code_bytes.get(pos) {
            Some(b' ') | Some(b'\t') => pos -= 1,
            Some(b'+') => {
                new_start = pos as u32;
                // Also consume space before +
                if pos > 0 && matches!(code_bytes.get(pos - 1), Some(b' ') | Some(b'\t')) {
                    new_start = (pos - 1) as u32;
                }
                break;
            }
            _ => break,
        }
    }

    // Expand right to include " + "
    let mut pos = end as usize;
    while pos < code_bytes.len() {
        match code_bytes.get(pos) {
            Some(b' ') | Some(b'\t') => pos += 1,
            Some(b'+') => {
                new_end = (pos + 1) as u32;
                // Also consume space after +
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

/// Build content tokens from concatenation segments
fn build_concat_content_tokens(segments: &[ConcatSegment]) -> Vec<PromptContentToken> {
    let mut var_idx = 0u32;
    segments
        .iter()
        .map(|seg| match seg {
            ConcatSegment::String(span) => {
                PromptContentToken::PromptContentTokenStr(PromptContentTokenStr {
                    r#type: PromptContentTokenStrTypeStr,
                    span: span.inner,
                })
            }
            ConcatSegment::Variable(span) => {
                let token = PromptContentToken::PromptContentTokenVar(PromptContentTokenVar {
                    r#type: PromptContentTokenVarTypeVar,
                    span: span.inner,
                    index: var_idx,
                });
                var_idx += 1;
                token
            }
            ConcatSegment::Primitive(span) => {
                PromptContentToken::PromptContentTokenStr(PromptContentTokenStr {
                    r#type: PromptContentTokenStrTypeStr,
                    span: span.inner,
                })
            }
            ConcatSegment::Other => {
                // This shouldn't happen as we filter these out earlier
                PromptContentToken::PromptContentTokenStr(PromptContentTokenStr {
                    r#type: PromptContentTokenStrTypeStr,
                    span: (0, 0),
                })
            }
        })
        .collect()
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

// Format function support

/// Process String.Format call: String.Format("Hello {0}", name)
#[allow(clippy::too_many_arguments)]
fn process_string_format(
    node: &Node,
    source: &str,
    filename: &str,
    stmt_start: u32,
    stmt_end: u32,
    comments: &CommentTracker,
    annotations: &[PromptAnnotation],
) -> Option<Prompt> {
    // Get the member access (String.Format part)
    let func_node = node.child_by_field_name("function")?;
    if func_node.kind() != "member_access_expression" {
        return None;
    }
    
    // Check if it's String.Format
    let func_text = func_node.utf8_text(source.as_bytes()).ok()?;
    if func_text != "String.Format" {
        return None;
    }
    
    // Get arguments
    let args_node = node.child_by_field_name("arguments")?;
    let mut arg_nodes = Vec::new();
    let mut cursor = args_node.walk();
    for child in args_node.children(&mut cursor) {
        if child.is_named() && child.kind() == "argument" {
            // Get the actual expression from the argument
            if let Some(expr) = child.child(0) {
                arg_nodes.push(expr);
            }
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
    let placeholders = parse_csharp_placeholders(format_str_content);
    
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

/// Parse C# format placeholders: {0}, {1}, {2}, etc.
fn parse_csharp_placeholders(format_str: &str) -> Vec<(usize, usize)> {
    let mut placeholders = Vec::new();
    let chars: Vec<char> = format_str.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        if chars[i] == '{' {
            // Check for escaped {{
            if i + 1 < chars.len() && chars[i + 1] == '{' {
                i += 2;
                continue;
            }
            
            let start = i;
            i += 1;
            
            // Find closing }
            while i < chars.len() && chars[i] != '}' {
                i += 1;
            }
            
            if i < chars.len() && chars[i] == '}' {
                // Check for escaped }}
                if i + 1 < chars.len() && chars[i + 1] == '}' {
                    i += 2;
                    continue;
                }
                
                i += 1; // Include the }
                placeholders.push((start, i));
                continue;
            }
        }
        i += 1;
    }
    
    placeholders
}
