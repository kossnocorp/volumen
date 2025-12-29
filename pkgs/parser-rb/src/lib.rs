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
        } else if right.kind() == "binary" {
            // Handle concatenation: "Hello, " + name + "!"
            let final_annotations = if !annotations.is_empty() {
                annotations.to_vec()
            } else {
                scopes.get_def_annotation(ident_name).unwrap_or_default()
            };
            
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

/// Build content tokens from span and variables.
/// For prompts without variables, returns a single str token.
/// For prompts with variables, returns interleaved str and var tokens.
/// For squiggly heredocs, creates tokens starting after stripped whitespace on each line.
fn build_content_tokens(
    span: &SpanShape,
    vars: &[PromptVar],
    source: &str,
    heredoc_info: Option<&spans::HeredocInfo>,
) -> Vec<PromptContentToken> {
    // Handle squiggly heredoc with whitespace stripping
    if let Some(info) = heredoc_info {
        if info.strips_whitespace {
            return build_heredoc_tokens(span, vars, source, info);
        }
    }

    // Standard token building (non-squiggly heredocs and regular strings)
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

/// Build content tokens for squiggly heredocs with whitespace stripping.
/// Creates separate tokens for each line, starting after the stripped whitespace.
fn build_heredoc_tokens(
    _span: &SpanShape,
    vars: &[PromptVar],
    source: &str,
    info: &spans::HeredocInfo,
) -> Vec<PromptContentToken> {
    let body_start = info.body_start as usize;
    let body_end = info.body_end as usize;
    let body_text = &source[body_start..body_end];
    
    // Calculate minimum indentation to strip
    let min_indent = calculate_min_heredoc_indent(body_text);
    
    let mut tokens = Vec::new();
    let mut current_pos = body_start;
    
    // Process each line
    for line in body_text.split_inclusive('\n') {
        let line_start = current_pos;
        let line_end = current_pos + line.len();
        
        // Calculate how much whitespace to skip on this line
        let line_without_newline = line.trim_end_matches('\n');
        let actual_indent = line_without_newline.len() - line_without_newline.trim_start().len();
        let strip_amount = actual_indent.min(min_indent);
        
        // Token starts after stripped whitespace
        let token_start = line_start + strip_amount;
        let token_end = line_end;
        
        // Check if there are any variables in this line
        let line_vars: Vec<_> = vars
            .iter()
            .filter(|v| {
                v.span.outer.0 >= token_start as u32 && v.span.outer.1 <= token_end as u32
            })
            .collect();
        
        if line_vars.is_empty() {
            // No variables in this line - single str token
            if token_start < token_end {
                tokens.push(PromptContentToken::PromptContentTokenStr(
                    PromptContentTokenStr {
                        r#type: PromptContentTokenStrTypeStr,
                        span: (token_start as u32, token_end as u32),
                    },
                ));
            }
        } else {
            // Has variables - create interleaved tokens
            let mut pos = token_start as u32;
            for var in line_vars {
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
            
            // Add trailing str token in this line (if any content)
            if pos < token_end as u32 {
                tokens.push(PromptContentToken::PromptContentTokenStr(
                    PromptContentTokenStr {
                        r#type: PromptContentTokenStrTypeStr,
                        span: (pos, token_end as u32),
                    },
                ));
            }
        }
        
        current_pos = line_end;
    }
    
    tokens
}

/// Calculate the minimum common leading whitespace in heredoc body
fn calculate_min_heredoc_indent(body_text: &str) -> usize {
    let mut min_indent = usize::MAX;
    
    for line in body_text.lines() {
        // Skip empty lines when calculating minimum indent
        if line.trim().is_empty() {
            continue;
        }
        
        let indent = line.len() - line.trim_start().len();
        if indent < min_indent {
            min_indent = indent;
        }
    }
    
    if min_indent == usize::MAX {
        0
    } else {
        min_indent
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

    // Extract variables if interpolated string
    let vars = spans::extract_interpolation_vars(&normalized_node, source);

    // Check if this is a squiggly heredoc
    let heredoc_info = spans::get_heredoc_info(&normalized_node, source);

    // Build content tokens
    let content = build_content_tokens(&span, &vars, source, heredoc_info.as_ref());

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

    // Build content tokens (no heredoc info for range-based prompts)
    let content = build_content_tokens(&span, &vars, source, None);

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

// Concatenation support

#[derive(Debug)]
enum ConcatSegment {
    String(SpanShape),
    Variable(SpanShape),
    Primitive(SpanShape),
    Other,
}

/// Process a binary expression for string concatenation
#[allow(clippy::too_many_arguments)]
fn process_concatenation(
    node: &Node,
    source: &str,
    filename: &str,
    stmt_start: u32,
    stmt_end: u32,
    comments: &CommentTracker,
    annotations: &[PromptAnnotation],
) -> Option<Prompt> {
    // Check if it's a + operator
    let operator = node.child_by_field_name("operator")?;
    let op_text = operator.utf8_text(source.as_bytes()).ok()?;
    if op_text != "+" {
        return None;
    }

    // Extract segments recursively
    let segments = extract_concat_segments(node, source);

    // Reject if no strings or contains complex objects
    let has_string = segments.iter().any(|s| matches!(s, ConcatSegment::String(_)));
    let has_other = segments.iter().any(|s| matches!(s, ConcatSegment::Other));

    if !has_string || has_other {
        return None;
    }

    // Build prompt outer span (entire concatenation expression)
    let prompt_outer = (node.start_byte() as u32, node.end_byte() as u32);

    // Find first and last string segments to determine inner span
    let first_string_pos = segments.iter().find_map(|s| match s {
        ConcatSegment::String(span) => Some(span.inner.0),
        _ => None,
    })?;

    let last_string_end = segments.iter().rev().find_map(|s| match s {
        ConcatSegment::String(span) => Some(span.inner.1),
        _ => None,
    })?;

    let prompt_inner = (first_string_pos, last_string_end);
    let span = SpanShape {
        outer: prompt_outer,
        inner: prompt_inner,
    };

    // Build vars and content tokens
    let mut vars = Vec::new();
    let mut content = Vec::new();

    for segment in &segments {
        match segment {
            ConcatSegment::String(s_span) => {
                // Add string token
                content.push(PromptContentToken::PromptContentTokenStr(
                    PromptContentTokenStr {
                        r#type: PromptContentTokenStrTypeStr,
                        span: (s_span.inner.0, s_span.inner.1),
                    },
                ));
            }
            ConcatSegment::Variable(v_span) | ConcatSegment::Primitive(v_span) => {
                // Expand to include operators
                let mut var_outer = v_span.clone();
                expand_to_operators(&mut var_outer, source, prompt_outer);

                let var = PromptVar {
                    span: SpanShape {
                        outer: var_outer.outer,
                        inner: v_span.inner,
                    },
                };

                // Add variable token (before pushing var)
                content.push(PromptContentToken::PromptContentTokenVar(
                    PromptContentTokenVar {
                        r#type: PromptContentTokenVarTypeVar,
                        span: var.span.inner,
                    },
                ));

                vars.push(var);
            }
            ConcatSegment::Other => {}
        }
    }

    // Calculate enclosure
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

/// Extract segments from a concatenation expression
fn extract_concat_segments(node: &Node, source: &str) -> Vec<ConcatSegment> {
    if node.kind() != "binary" {
        return classify_single_node(node, source);
    }

    // Check if it's a + operator
    let operator = match node.child_by_field_name("operator") {
        Some(n) => n,
        None => return classify_single_node(node, source),
    };
    let op_text = operator.utf8_text(source.as_bytes()).unwrap_or("");
    if op_text != "+" {
        return classify_single_node(node, source);
    }

    // Recursively process left and right
    let mut segments = Vec::new();

    if let Some(left) = node.child_by_field_name("left") {
        segments.extend(extract_concat_segments(&left, source));
    }

    if let Some(right) = node.child_by_field_name("right") {
        segments.extend(extract_concat_segments(&right, source));
    }

    segments
}

/// Classify a single node as a segment
fn classify_single_node(node: &Node, source: &str) -> Vec<ConcatSegment> {
    let kind = node.kind();

    match kind {
        "string" | "string_content" | "heredoc_body" => {
            // String literal
            let span = span_shape_string_like(node, source);
            vec![ConcatSegment::String(span)]
        }
        "identifier" | "constant" | "instance_variable" | "class_variable" | "global_variable" => {
            // Variable
            let outer = (node.start_byte() as u32, node.end_byte() as u32);
            let inner = outer;
            vec![ConcatSegment::Variable(SpanShape { outer, inner })]
        }
        "call" | "method_call" => {
            // Function/method call - treat as variable
            let outer = (node.start_byte() as u32, node.end_byte() as u32);
            let inner = outer;
            vec![ConcatSegment::Variable(SpanShape { outer, inner })]
        }
        "integer" | "float" | "true" | "false" => {
            // Primitives - Ruby requires .to_s conversion, so this would be a type error
            // But we can still parse it as a primitive for completeness
            let outer = (node.start_byte() as u32, node.end_byte() as u32);
            let inner = outer;
            vec![ConcatSegment::Primitive(SpanShape { outer, inner })]
        }
        "array" | "hash" => {
            // Complex objects - reject
            vec![ConcatSegment::Other]
        }
        "parenthesized_statements" => {
            // Unwrap parentheses
            let mut cursor = node.walk();
            if cursor.goto_first_child() {
                loop {
                    let child = cursor.node();
                    if child.kind() != "(" && child.kind() != ")" {
                        return classify_single_node(&child, source);
                    }
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
            }
            vec![ConcatSegment::Other]
        }
        _ => {
            // Unknown - reject
            vec![ConcatSegment::Other]
        }
    }
}

/// Expand variable span to include surrounding operators
fn expand_to_operators(var_span: &mut SpanShape, source: &str, prompt_outer: (u32, u32)) {
    let bytes = source.as_bytes();
    let mut new_start = var_span.outer.0;
    let mut new_end = var_span.outer.1;

    // Expand left to include " + "
    let mut i = new_start as usize;
    while i > prompt_outer.0 as usize {
        i -= 1;
        let c = bytes[i];
        if c == b'+' {
            // Include the + and any spaces before it
            while i > prompt_outer.0 as usize && (bytes[i - 1] == b' ' || bytes[i - 1] == b'\t') {
                i -= 1;
            }
            new_start = i as u32;
            break;
        } else if c != b' ' && c != b'\t' {
            break;
        }
    }

    // Expand right to include " + "
    let mut i = new_end as usize;
    while i < prompt_outer.1 as usize {
        let c = bytes[i];
        if c == b'+' {
            // Include the + and any spaces after it
            i += 1;
            while i < prompt_outer.1 as usize && (bytes[i] == b' ' || bytes[i] == b'\t') {
                i += 1;
            }
            new_end = i as u32;
            break;
        } else if c != b' ' && c != b'\t' {
            break;
        }
        i += 1;
    }

    var_span.outer = (new_start, new_end);
}
