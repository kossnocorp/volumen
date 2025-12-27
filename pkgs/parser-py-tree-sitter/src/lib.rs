mod comments;
mod queries;
mod scope;
mod spans;

use comments::CommentTracker;
use scope::ScopeTracker;
use spans::{extract_fstring_vars, span_shape_string_like};
use tree_sitter::{Node, Parser, Tree};
pub use volumen_parser_core::VolumenParser;
use volumen_types::*;

pub struct ParserPy {}

impl VolumenParser for ParserPy {
    fn parse(source: &str, filename: &str) -> ParseResult {
        // Initialize Tree-sitter parser
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .expect("Failed to load Python grammar");

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

    // Handle scope boundaries
    if kind == "function_definition" || kind == "class_definition" {
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

    // Handle expression statements (which contain assignments)
    if kind == "expression_statement" {
        // Check if it contains an assignment
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            let child = cursor.node();
            if child.kind() == "assignment" {
                process_assignment(&child, source, filename, comments, scopes, prompts);
                return; // Don't process children again
            }
        }
    }

    // Handle assignments directly
    if kind == "assignment" {
        process_assignment(node, source, filename, comments, scopes, prompts);
        return; // Don't process children of assignments
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
    process_assignment_with_annotations(
        node, source, filename, comments, scopes, prompts, None, None,
    );
}

/// Process an assignment node with inherited annotations (for chained assignments).
fn process_assignment_with_annotations(
    node: &Node,
    source: &str,
    filename: &str,
    comments: &CommentTracker,
    scopes: &mut ScopeTracker,
    prompts: &mut Vec<Prompt>,
    inherited_annotations: Option<Vec<PromptAnnotation>>,
    parent_stmt_bounds: Option<(u32, u32)>,
) {
    let (stmt_start, stmt_end) =
        parent_stmt_bounds.unwrap_or_else(|| (node.start_byte() as u32, node.end_byte() as u32));

    // Collect annotations - use inherited if provided, otherwise collect from comments
    let all_annotations = if let Some(inherited) = inherited_annotations {
        inherited
    } else {
        let leading_annotations = comments.collect_adjacent_leading(stmt_start);
        let inline_annotations = comments.collect_inline_prompt(stmt_start, stmt_end);
        let mut all_annotations = leading_annotations.clone();
        all_annotations.extend(inline_annotations);
        all_annotations
    };

    let has_prompt_annotation = !all_annotations.is_empty();

    // Get assignment components using field names
    let left = match node.child_by_field_name("left") {
        Some(n) => n,
        None => return,
    };

    let right_node = node.child_by_field_name("right");
    let has_type_annotation = node.child_by_field_name("type").is_some();

    // Handle different assignment patterns
    match left.kind() {
        "identifier" => {
            // Simple assignment: var = value
            let ident_name = left.utf8_text(source.as_bytes()).unwrap_or("");
            process_identifier_assignment(
                ident_name,
                right_node,
                has_type_annotation,
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

            // Handle chained assignments: a = b = value
            // If the right side is also an assignment, we need to:
            // 1. Find the actual string value at the end of the chain
            // 2. Create a prompt for this identifier if it's a prompt
            // 3. Process the inner assignment recursively
            if let Some(right) = right_node {
                if right.kind() == "assignment" {
                    // For chained assignments, find the ultimate value by walking the chain
                    let mut current = right;
                    while let Some(inner_right) = current.child_by_field_name("right") {
                        if inner_right.kind() == "assignment" {
                            current = inner_right;
                        } else {
                            // Found the ultimate value
                            let is_prompt =
                                is_prompt_variable(ident_name, has_prompt_annotation, scopes);
                            if is_prompt && is_string_like(&inner_right) {
                                scopes.mark_prompt_ident(ident_name);

                                if has_type_annotation && has_prompt_annotation {
                                    scopes
                                        .store_def_annotation(ident_name, all_annotations.to_vec());
                                    scopes.mark_annotated(ident_name);
                                }

                                create_prompt_from_string(
                                    &inner_right,
                                    source,
                                    filename,
                                    stmt_start,
                                    stmt_end,
                                    comments,
                                    &all_annotations,
                                    prompts,
                                );
                            }
                            break;
                        }
                    }

                    // Process the nested assignment with preserved bounds
                    process_assignment_with_annotations(
                        &right,
                        source,
                        filename,
                        comments,
                        scopes,
                        prompts,
                        Some(all_annotations.clone()),
                        Some((stmt_start, stmt_end)),
                    );
                }
            }
        }
        "pattern_list" | "tuple_pattern" | "list_pattern" => {
            // Multi-assignment: a, b = values or [a, b] = values
            process_multi_assignment(
                &left,
                right_node,
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
    right_node: Option<Node>,
    has_type_annotation: bool,
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
    // Check if it's a type-only annotation (no value)
    if right_node.is_none() {
        if has_type_annotation && has_prompt_annotation {
            // Store the annotation for later use
            scopes.store_def_annotation(ident_name, annotations.to_vec());
            scopes.mark_annotated(ident_name);
            scopes.mark_prompt_ident(ident_name);
        }
        return;
    }

    let right = match right_node {
        Some(n) => n,
        None => return,
    };

    // Determine if this is a prompt
    let is_prompt = is_prompt_variable(ident_name, has_prompt_annotation, scopes);

    if is_prompt {
        // Mark as prompt identifier (even if value isn't a string yet)
        scopes.mark_prompt_ident(ident_name);

        // Store definition annotations if this is annotated
        if has_type_annotation && has_prompt_annotation {
            scopes.store_def_annotation(ident_name, annotations.to_vec());
            scopes.mark_annotated(ident_name);
        }

        // Check if it's a string or f-string
        if is_string_like(&right) {
            // Annotations from comment tracker are already validated to contain @prompt
            // Get annotations (from current statement or from definition)
            let final_annotations = if !annotations.is_empty() {
                annotations.to_vec()
            } else {
                scopes.get_def_annotation(ident_name).unwrap_or_default()
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
                prompts,
            );
        }
    }
}

/// Process multi-assignment (tuple unpacking).
#[allow(clippy::too_many_arguments)]
fn process_multi_assignment(
    left: &Node,
    right_node: Option<Node>,
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
    let right = match right_node {
        Some(n) => n,
        None => return,
    };

    // Extract identifiers from left side
    let mut identifiers = Vec::new();
    extract_identifiers(left, source, &mut identifiers);

    // Extract value ranges from right side
    let mut value_ranges = Vec::new();
    extract_value_ranges(&right, &mut value_ranges);

    // Match identifiers with values
    for (i, ident_name) in identifiers.iter().enumerate() {
        if let Some((start, end, kind)) = value_ranges.get(i) {
            let is_prompt = is_prompt_variable(ident_name, has_prompt_annotation, scopes);

            if is_prompt && (kind == "string" || kind == "concatenated_string") {
                scopes.mark_prompt_ident(ident_name);

                // Create a temporary node-like structure using ranges
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

/// Extract identifiers from a pattern (tuple/list).
fn extract_identifiers(node: &Node, source: &str, identifiers: &mut Vec<String>) {
    if node.kind() == "identifier" {
        if let Ok(name) = node.utf8_text(source.as_bytes()) {
            identifiers.push(name.to_string());
        }
        return;
    }

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

/// Extract value ranges from a tuple/list.
/// Returns (start_byte, end_byte, kind) for each value.
fn extract_value_ranges(node: &Node, ranges: &mut Vec<(usize, usize, String)>) {
    let kind = node.kind();
    if kind == "string" || kind == "concatenated_string" {
        ranges.push((node.start_byte(), node.end_byte(), kind.to_string()));
        return;
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            extract_value_ranges(&child, ranges);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Create a prompt from a byte range (for multi-assignment).
#[allow(clippy::too_many_arguments)]
fn create_prompt_from_range(
    start: usize,
    end: usize,
    source: &str,
    filename: &str,
    stmt_start: u32,
    stmt_end: u32,
    comments: &CommentTracker,
    annotations: &[PromptAnnotation],
    prompts: &mut Vec<Prompt>,
) {
    // Calculate spans manually
    let bytes = source.as_bytes();

    // Find first quote character
    let mut i = start;
    while i < end {
        let c = bytes[i];
        if c == b'\'' || c == b'\"' || c == b'f' || c == b'r' {
            break;
        }
        i += 1;
    }

    // Check if it's an f-string
    let is_fstring = i < end && (bytes[i] == b'f' || bytes[i] == b'r');
    if is_fstring {
        i += 1; // Skip f or r
        // Handle fr/rf prefix
        if i < end && (bytes[i] == b'f' || bytes[i] == b'r') {
            i += 1;
        }
    }

    // Find quote character
    while i < end {
        let c = bytes[i];
        if c == b'\'' || c == b'\"' {
            break;
        }
        i += 1;
    }

    let quote_pos = i;
    let quote_char = if quote_pos < end {
        bytes[quote_pos]
    } else {
        b'\''
    };

    // Detect triple quotes
    let mut quote_len = 1u32;
    if quote_pos + 2 < end
        && bytes[quote_pos + 1] == quote_char
        && bytes[quote_pos + 2] == quote_char
    {
        quote_len = 3;
    }

    let outer = (start as u32, end as u32);

    let inner = (
        (quote_pos as u32).saturating_add(quote_len),
        outer.1.saturating_sub(quote_len),
    );

    let span = SpanShape { outer, inner };

    // Extract expression text
    let exp = source[span.outer.0 as usize..span.outer.1 as usize].to_string();

    // Extract variables if f-string (simplified version)
    let vars = Vec::new(); // TODO: Extract vars from f-string in multi-assignment

    // Build content tokens
    let content = build_content_tokens(&span, &vars);

    // Calculate enclosure - only include valid @prompt leading comments
    let enclosure_start = comments
        .get_leading_start(stmt_start)
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

/// Check if a variable should be considered a prompt.
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

/// Check if a node is a string-like value.
fn is_string_like(node: &Node) -> bool {
    node.kind() == "string" || node.kind() == "concatenated_string"
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

    for var in vars {
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

/// Create a prompt from a string node.
#[allow(clippy::too_many_arguments)]
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

    // Extract expression text
    let exp = source[span.outer.0 as usize..span.outer.1 as usize].to_string();

    // Extract variables if f-string
    let vars = if is_fstring(string_node, source) {
        extract_fstring_vars(string_node, source)
    } else {
        Vec::new()
    };

    // Build content tokens
    let content = build_content_tokens(&span, &vars);

    // Calculate enclosure - only include valid @prompt leading comments
    let enclosure_start = comments
        .get_leading_start(stmt_start)
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

/// Check if a string is an f-string.
fn is_fstring(node: &Node, source: &str) -> bool {
    let text = source[node.start_byte()..node.end_byte()].to_string();
    text.starts_with('f') || text.starts_with("fr") || text.starts_with("rf")
}

/// Format Tree-sitter error message.
fn format_tree_sitter_error(root: &Node, source: &str) -> String {
    if !root.has_error() {
        return "Unknown syntax error".to_string();
    }

    // Find first ERROR node
    find_error_node(root, source)
}

/// Find and format error node.
fn find_error_node(node: &Node, source: &str) -> String {
    if node.is_error() || node.is_missing() {
        let pos = node.start_position();
        return format!(
            "Syntax error at line {}, column {}",
            pos.row + 1,
            pos.column + 1
        );
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            let result = find_error_node(&child, source);
            if result != "Unknown syntax error" {
                return result;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    "Unknown syntax error".to_string()
}
