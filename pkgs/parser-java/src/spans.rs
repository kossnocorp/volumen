use tree_sitter::Node;
use volumen_types::SpanShape;

/// Information about text block incidental whitespace stripping
#[derive(Debug, Clone)]
pub struct TextBlockInfo {
    /// Whether this text block strips whitespace (all text blocks do in Java 15+)
    pub strips_whitespace: bool,
    /// The body content range
    pub body_start: u32,
    pub body_end: u32,
}

/// Calculate outer and inner spans for a string-like node.
/// For Java, this handles string literals and text blocks (Java 15+).
pub fn span_shape_string_like(node: &Node, source: &str) -> SpanShape {
    let start = node.start_byte();
    let end = node.end_byte();

    let outer = (start as u32, end as u32);

    let kind = node.kind();
    
    // Get bytes for checking text block markers
    let bytes = source.as_bytes();

    // For text blocks (""" ... """), the quotes are 3 characters
    // Note: tree-sitter-java uses "string_literal" for both regular strings and text blocks
    if kind == "string_literal" && start + 3 <= bytes.len() 
        && bytes[start] == b'"' 
        && bytes[start + 1] == b'"' 
        && bytes[start + 2] == b'"' 
    {
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

/// Detect if this is a text block and get its info
pub fn get_text_block_info(node: &Node, source: &str) -> Option<TextBlockInfo> {
    let start = node.start_byte();
    let end = node.end_byte();
    let text = &source[start..end];
    
    // Check for opening """
    if !text.starts_with("\"\"\"") {
        return None;
    }
    
    // Find the first newline after opening """
    let first_newline = text.find('\n')?;
    let body_start = start + first_newline + 1;
    
    // Find closing """
    let closing_pos = text.rfind("\"\"\"")?;
    if closing_pos < 3 {
        return None; // Closing is same as opening
    }
    
    // Find the newline before the closing """
    // The content ends at the newline character before the closing delimiter line
    let text_before_closing = &text[..closing_pos];
    let body_end = if let Some(last_newline_pos) = text_before_closing.rfind('\n') {
        start + last_newline_pos + 1  // Include the newline in the content
    } else {
        // No newline before closing - content is on same line as opening (unusual but valid)
        start + closing_pos
    };
    
    Some(TextBlockInfo {
        strips_whitespace: true,
        body_start: body_start as u32,
        body_end: body_end as u32,
    })
}


