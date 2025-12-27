use tree_sitter::Node;
use volumen_types::{PromptVar, SpanShape};

/// Calculate outer and inner spans for a string-like node.
/// For PHP, this handles single-quoted, double-quoted strings, and heredocs.
pub fn span_shape_string_like(node: &Node, source: &str) -> SpanShape {
    let start = node.start_byte();
    let end = node.end_byte();

    let outer = (start as u32, end as u32);

    // For heredocs, the content doesn't have quotes to skip
    let kind = node.kind();
    if kind == "heredoc" || kind == "nowdoc" || kind == "heredoc_body" {
        // Heredocs: the entire node is the content
        return SpanShape {
            outer: outer.clone(),
            inner: outer,
        };
    }

    // For PHP strings, we need to skip the quotes
    // Determine quote length (usually 1)
    let bytes = source.as_bytes();
    let mut quote_len = 1u32;

    // Check quote markers
    if start < bytes.len() {
        let first_char = bytes[start];
        if first_char == b'\'' || first_char == b'"' {
            quote_len = 1;
        }
    }

    let inner = (
        (start as u32).saturating_add(quote_len),
        (end as u32).saturating_sub(quote_len),
    );

    SpanShape { outer, inner }
}

/// Extract variables from interpolated string expressions (e.g., "Hello {$user}").
/// PHP uses various node types for string interpolation:
/// - Simple variables: {$var}
/// - Complex expressions: {$obj->prop} or {$arr['key']}
pub fn extract_interpolation_vars(node: &Node, source: &str) -> Vec<PromptVar> {
    let mut vars = Vec::new();

    fn walk_node(node: &Node, source: &str, vars: &mut Vec<PromptVar>) {
        let kind = node.kind();

        // PHP tree-sitter uses different node types for interpolation
        // Common types: "variable_name", "simple_variable", "encapsed_variable", "complex_variable"
        if kind == "simple_variable" || kind == "complex_variable" || kind == "variable_name" {
            // For simple variables like {$user}, the entire node is the variable
            let outer_start = node.start_byte() as u32;
            let outer_end = node.end_byte() as u32;

            // Check if this is surrounded by braces
            let exp = source[outer_start as usize..outer_end as usize].to_string();

            // For PHP, we need to check the context to see if it's in braces
            // The parent or surrounding context might have the braces
            let (actual_outer_start, actual_outer_end) = if outer_start > 0
                && source.as_bytes().get((outer_start - 1) as usize) == Some(&b'{')
            {
                // This is a braced variable like {$var}
                let start = outer_start - 1;
                let end = if source.as_bytes().get(outer_end as usize) == Some(&b'}') {
                    outer_end + 1
                } else {
                    outer_end
                };
                (start, end)
            } else {
                (outer_start, outer_end)
            };

            let full_exp =
                source[actual_outer_start as usize..actual_outer_end as usize].to_string();
            let inner_start = if actual_outer_start < outer_start {
                outer_start
            } else {
                actual_outer_start
            };
            let inner_end = if actual_outer_end > outer_end {
                outer_end
            } else {
                actual_outer_end
            };

            vars.push(PromptVar {
                span: SpanShape {
                    outer: (actual_outer_start, actual_outer_end),
                    inner: (inner_start, inner_end),
                },
            });
        }

        // Recursively walk children
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                walk_node(&cursor.node(), source, vars);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }

    walk_node(node, source, &mut vars);
    vars
}

/// Check if a node is an interpolated string (has variable interpolation).
pub fn is_interpolated_string(node: &Node) -> bool {
    let kind = node.kind();
    if kind != "string" && kind != "string_content" && kind != "encapsed_string" {
        return false;
    }

    fn has_interpolation(node: &Node) -> bool {
        let kind = node.kind();
        if kind == "simple_variable" || kind == "complex_variable" || kind == "variable_name" {
            return true;
        }

        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                if has_interpolation(&cursor.node()) {
                    return true;
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        false
    }

    has_interpolation(node)
}

/// Check if a node is a string-like node (including heredocs).
pub fn is_string_like(node: &Node) -> bool {
    matches!(
        node.kind(),
        "string" | "string_content" | "encapsed_string" | "heredoc" | "nowdoc" | "heredoc_body"
    )
}
