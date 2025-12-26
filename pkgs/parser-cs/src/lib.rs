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

        // Check if it's a string
        if is_string_like(&value_node) {
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
                &value_node,
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

    // Extract expression text
    let exp = source[span.outer.0 as usize..span.outer.1 as usize].to_string();

    // Extract variables if interpolated string (C# has string interpolation)
    let vars = spans::extract_interpolation_vars(string_node, source);

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
