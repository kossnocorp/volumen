use tree_sitter::Node;
use volumen_types::{PromptVar, Span, SpanShape};

/// Calculate outer and inner spans for a string-like node.
/// For C#, this handles string literals, interpolated strings, and verbatim strings.
pub fn span_shape_string_like(node: &Node, source: &str) -> SpanShape {
    let start = node.start_byte();
    let end = node.end_byte();

    let outer = Span {
        start: start as u32,
        end: end as u32,
    };

    let kind = node.kind();
    let bytes = source.as_bytes();

    // For interpolated strings ($"...") or verbatim interpolated strings ($@"...")
    if kind == "interpolated_string_expression" {
        // Check for $@" or @$" prefix (verbatim interpolated)
        let mut prefix_len = 0;
        if start < bytes.len() {
            if bytes[start] == b'$' {
                prefix_len = 1;
                if start + 1 < bytes.len() && bytes[start + 1] == b'@' {
                    prefix_len = 2;
                }
            } else if bytes[start] == b'@' && start + 1 < bytes.len() && bytes[start + 1] == b'$' {
                prefix_len = 2;
            }
        }

        // Add 1 for the opening quote after the prefix
        let inner_start = start as u32 + prefix_len + 1;
        let inner_end = (end as u32).saturating_sub(1); // Skip closing quote

        return SpanShape {
            outer,
            inner: Span {
                start: inner_start,
                end: inner_end,
            },
        };
    }

    // For verbatim string literals (@"...")
    if kind == "verbatim_string_literal" {
        // Skip @ and opening quote
        let inner_start = (start as u32) + 2;
        let inner_end = (end as u32).saturating_sub(1); // Skip closing quote

        return SpanShape {
            outer,
            inner: Span {
                start: inner_start,
                end: inner_end,
            },
        };
    }

    // For regular string literals ("...")
    let quote_len = 1u32;
    let inner = Span {
        start: (start as u32).saturating_add(quote_len),
        end: (end as u32).saturating_sub(quote_len),
    };

    SpanShape { outer, inner }
}

/// Extract variables from interpolated string expressions (e.g., $"Hello {name}").
pub fn extract_interpolation_vars(node: &Node, source: &str) -> Vec<PromptVar> {
    let mut vars = Vec::new();

    // Only process if this is an interpolated string
    if node.kind() != "interpolated_string_expression" {
        return vars;
    }

    // Walk through children to find interpolation nodes
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            if child.kind() == "interpolation" {
                // Interpolation includes { and }
                let outer_start = child.start_byte() as u32;
                let outer_end = child.end_byte() as u32;

                // Inner content is between { and }
                let inner_start = outer_start + 1; // Skip {
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
