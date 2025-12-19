use tree_sitter::Node;
use volumen_types::{PromptVar, Span, SpanShape};

/// Calculate outer and inner spans for a string-like node.
/// For Ruby, this handles single-quoted, double-quoted strings, and heredocs.
pub fn span_shape_string_like(node: &Node, source: &str) -> SpanShape {
    let start = node.start_byte();
    let end = node.end_byte();

    let outer = Span {
        start: start as u32,
        end: end as u32,
    };

    // For heredocs, the content doesn't have quotes to skip
    let kind = node.kind();
    if kind == "heredoc_body" || kind == "heredoc_beginning" {
        // Heredocs: the entire node is the content
        return SpanShape { outer: outer.clone(), inner: outer };
    }

    // For Ruby strings, we need to skip the quotes
    // Determine quote length (usually 1)
    let bytes = source.as_bytes();
    let mut quote_len = 1u32;

    // Check for quote markers
    if start < bytes.len() {
        let first_char = bytes[start];
        if first_char == b'\'' || first_char == b'"' {
            // Check for triple quotes (though rare in Ruby)
            if start + 2 < bytes.len()
                && bytes[start + 1] == first_char
                && bytes[start + 2] == first_char
            {
                quote_len = 3;
            } else {
                quote_len = 1;
            }
        }
    }

    let inner = Span {
        start: (start as u32).saturating_add(quote_len),
        end: (end as u32).saturating_sub(quote_len),
    };

    SpanShape { outer, inner }
}

/// Extract variables from interpolated string expressions (e.g., "Hello #{name}").
pub fn extract_interpolation_vars(node: &Node, source: &str) -> Vec<PromptVar> {
    let mut vars = Vec::new();

    // Walk through children to find interpolation nodes
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            if child.kind() == "interpolation" {
                // Interpolation includes #{ and }
                let outer_start = child.start_byte() as u32;
                let outer_end = child.end_byte() as u32;

                // Inner content is between #{ and }
                let inner_start = outer_start + 2; // Skip #{
                let inner_end = outer_end - 1; // Skip }

                let exp = source[outer_start as usize..outer_end as usize].to_string();

                vars.push(PromptVar {
                    exp,
                    span: SpanShape {
                        outer: Span {
                            start: outer_start,
                            end: outer_end,
                        },
                        inner: Span {
                            start: inner_start,
                            end: inner_end,
                        },
                    },
                });
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    vars
}

/// Check if a node is an interpolated string (has interpolation children).
pub fn is_interpolated_string(node: &Node) -> bool {
    let kind = node.kind();
    if kind != "string" && kind != "string_content" {
        return false;
    }

    // Check if it has any interpolation children
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            if cursor.node().kind() == "interpolation" {
                return true;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    false
}

/// Check if a node is a string-like node (including heredocs).
pub fn is_string_like(node: &Node) -> bool {
    matches!(node.kind(), "string" | "string_content" | "heredoc_body" | "heredoc_beginning")
}
