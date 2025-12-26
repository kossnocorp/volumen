mod comments;
mod scope;
mod spans;

use comments::CommentTracker;
use scope::ScopeTracker;
use spans::span_shape_string_like;
use tree_sitter::{Node, Parser, Tree};
pub use volumen_parser_core::VolumenParser;
use volumen_types::*;

pub struct ParserRb {}

impl VolumenParser for ParserRb {
    fn parse(source: &str, filename: &str) -> ParseResult {
        // Initialize Tree-sitter parser
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_ruby::LANGUAGE.into())
            .expect("Failed to load Ruby grammar");

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

    // Handle scope boundaries (method, class, module, block)
    if kind == "method" || kind == "class" || kind == "module" {
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

    // Handle prompt-marked identifier declarations (e.g., annotated standalone identifiers)
    if kind == "identifier" {
        process_identifier_declaration(node, source, comments, scopes);
    }

    // Handle assignments
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

/// Mark annotated standalone identifiers as prompt definitions so later assignments inherit them.
fn process_identifier_declaration(
    node: &Node,
    source: &str,
    comments: &CommentTracker,
    scopes: &mut ScopeTracker,
) {
    let stmt_start = node.start_byte() as u32;
    let stmt_end = node.end_byte() as u32;

    let mut annotations = comments.collect_adjacent_leading(stmt_start);
    let inline_annotations = comments.collect_inline_prompt(stmt_start, stmt_end);
    annotations.extend(inline_annotations);

    if annotations.is_empty() {
        return;
    }

    if let Ok(name) = node.utf8_text(source.as_bytes()) {
        scopes.mark_prompt_ident(name);
        scopes.store_def_annotation(name, annotations);
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

    // Handle chained assignments like `a = b = "Hi"`
    if right.kind() == "assignment" {
        process_chained_assignment(
            node,
            source,
            filename,
            has_prompt_annotation,
            &all_annotations,
            stmt_start,
            stmt_end,
            comments,
            scopes,
            prompts,
        );
        return;
    }

    // Handle different assignment patterns
    match left.kind() {
        "identifier" => {
            // Simple assignment: var = value
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
        "left_assignment_list" => {
            // Multi-assignment: a, b = values
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

/// Process chained assignments (e.g., `a = b = "Hi"`).
#[allow(clippy::too_many_arguments)]
fn process_chained_assignment(
    node: &Node,
    source: &str,
    filename: &str,
    has_prompt_annotation: bool,
    annotations: &[PromptAnnotation],
    stmt_start: u32,
    stmt_end: u32,
    comments: &CommentTracker,
    scopes: &mut ScopeTracker,
    prompts: &mut Vec<Prompt>,
) {
    let mut idents = Vec::new();
    let mut current = node.clone();
    let mut value_node: Option<Node> = None;

    loop {
        if let Some(left) = current.child_by_field_name("left") {
            extract_identifiers(&left, source, &mut idents);
        }

        if let Some(right) = current.child_by_field_name("right") {
            if right.kind() == "assignment" {
                current = right;
                continue;
            }
            value_node = Some(right);
        }
        break;
    }

    let Some(value) = value_node else {
        return;
    };

    if !is_string_like(&value) {
        return;
    }

    for ident in idents.iter() {
        if is_prompt_variable(ident, has_prompt_annotation, scopes) {
            scopes.mark_prompt_ident(ident);
            create_prompt_from_string(
                &value,
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

        // Check if it's a string
        if is_string_like(right) {
            // Check if current annotations contain at least one valid @prompt
            let has_valid_current_annotation = annotations
                .iter()
                .any(|a| volumen_parser_core::parse_annotation(&a.exp).unwrap_or(false));

            // Get annotations (from current statement or from definition)
            let final_annotations = if has_valid_current_annotation {
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

            if is_prompt && (*kind == "string" || *kind == "string_content") {
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

/// Check if a node represents a string literal or heredoc.
fn is_string_like(node: &Node) -> bool {
    matches!(
        node.kind(),
        "string" | "string_content" | "heredoc_body" | "heredoc_beginning"
    )
}

/// Extract identifiers from a left_assignment_list or similar pattern.
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

    if kind == "string" || kind == "string_content" {
        values.push((node.start_byte() as u32, node.end_byte() as u32, kind));
        return;
    }

    if kind == "array" {
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
    // Normalize heredoc nodes to the nearest enclosing "string" node so spans cover the body
    let mut normalized_node = *string_node;
    if matches!(string_node.kind(), "heredoc_beginning" | "heredoc_body") {
        let mut current = Some(*string_node);
        while let Some(node) = current {
            if node.kind() == "string" {
                normalized_node = node;
                break;
            }
            current = node.parent();
        }
    }

    // Calculate spans
    let span = span_shape_string_like(&normalized_node, source);

    // Extract expression text
    let exp = source[span.outer.0 as usize..span.outer.1 as usize].to_string();

    // Extract variables if interpolated string
    let vars = spans::extract_interpolation_vars(&normalized_node, source);

    // Calculate enclosure - use get_any_leading_start to include ANY leading comment (valid or not)
    let enclosure_start = comments
        .get_any_leading_start(stmt_start)
        .unwrap_or(stmt_start);
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

    // Calculate enclosure
    let enclosure_start = comments
        .get_any_leading_start(stmt_start)
        .unwrap_or(stmt_start);
    let enclosure = (enclosure_start, stmt_end);

    prompts.push(Prompt {
        file: filename.to_string(),
        span,
        enclosure,
        exp: exp.to_string(),
        vars: Vec::new(),
        annotations: annotations.to_vec(),
    });
}
