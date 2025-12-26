use tree_sitter::Node;
use volumen_types::SpanShape;

/// Calculate outer and inner spans for a string-like node.
/// For Java, this handles string literals and text blocks (Java 15+).
pub fn span_shape_string_like(node: &Node, source: &str) -> SpanShape {
    let start = node.start_byte();
    let end = node.end_byte();

    let outer = (start as u32, end as u32);

    let kind = node.kind();

    // For text blocks (""" ... """), the quotes are 3 characters
    if kind == "text_block" {
        let bytes = source.as_bytes();
        let mut inner_start = start as u32;
        let mut inner_end = end as u32;

        // Check for opening """
        if start + 3 <= bytes.len()
            && bytes[start] == b'"'
            && bytes[start + 1] == b'"'
            && bytes[start + 2] == b'"'
        {
            inner_start = (start as u32) + 3;
        }

        // Check for closing """
        if end >= 3 && bytes[end - 3] == b'"' && bytes[end - 2] == b'"' && bytes[end - 1] == b'"' {
            inner_end = (end as u32) - 3;
        }

        return SpanShape {
            outer,
            inner: (inner_start, inner_end),
        };
    }

    // For regular string literals, skip the double quotes
    let bytes = source.as_bytes();
    let mut quote_len = 1u32;

    if start < bytes.len() && bytes[start] == b'"' {
        quote_len = 1;
    }

    let inner = (
        (start as u32).saturating_add(quote_len),
        (end as u32).saturating_sub(quote_len),
    );

    SpanShape { outer, inner }
}
