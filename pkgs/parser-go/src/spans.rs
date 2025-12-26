use tree_sitter::Node;
use volumen_types::SpanShape;

/// Calculate outer and inner spans for a string-like node.
/// For Go, this handles interpreted strings ("...") and raw strings (`...`).
pub fn span_shape_string_like(node: &Node, source: &str) -> SpanShape {
    let start = node.start_byte();
    let end = node.end_byte();

    let outer = (start as u32, end as u32);

    let kind = node.kind();
    let bytes = source.as_bytes();

    // For raw strings (backticks), skip the backtick delimiters
    if kind == "raw_string_literal" {
        let quote_len = 1u32; // backtick is 1 character
        let inner = (
            (start as u32).saturating_add(quote_len),
            (end as u32).saturating_sub(quote_len),
        );
        return SpanShape { outer, inner };
    }

    // For interpreted strings, skip the double quotes
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
