use tree_sitter::Node;
use volumen_types::{PromptVar, SpanShape};

/// Information about heredoc whitespace stripping behavior for PHP
#[derive(Debug, Clone)]
pub struct HeredocInfo {
    /// Whether this heredoc strips whitespace (flexible heredoc - PHP 7.3+)
    pub strips_whitespace: bool,
    /// The body content range (starts after opening marker's newline)
    pub body_start: u32,
    pub body_end: u32,
}

/// Calculate outer and inner spans for a string-like node.
/// For PHP, this handles single-quoted, double-quoted strings, and heredocs.
pub fn span_shape_string_like(node: &Node, source: &str) -> SpanShape {
    let start = node.start_byte();
    let end = node.end_byte();

    // For heredocs, we need to parse the structure to find the body
    let kind = node.kind();
    if kind == "heredoc" || kind == "nowdoc" {
        return parse_heredoc_span(node, source, start, end);
    }
    
    if kind == "heredoc_body" {
        // Direct heredoc_body node - return as-is
        let outer = (start as u32, end as u32);
        return SpanShape {
            outer,
            inner: outer,
        };
    }

    let outer = (start as u32, end as u32);

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

/// Parse PHP heredoc/nowdoc to get correct outer and inner spans
fn parse_heredoc_span(node: &Node, source: &str, start: usize, end: usize) -> SpanShape {
    let text = &source[start..end.min(source.len())];
    
    // Find the opening marker (<<<TEXT or <<<'TEXT' or <<<"TEXT")
    if let Some(marker_start) = text.find("<<<") {
        let after_marker = &text[marker_start + 3..];
        
        // Skip whitespace after <<<
        let label_start_offset = after_marker.len() - after_marker.trim_start().len();
        let after_spaces = &after_marker[label_start_offset..];
        
        // Extract the label
        let (label, _is_quoted) = if let Some(first_char) = after_spaces.chars().next() {
            if first_char == '\'' || first_char == '"' {
                // Quoted label: <<<'TEXT' or <<<"TEXT"
                let remaining = &after_spaces[1..];
                if let Some(end_quote) = remaining.find(first_char) {
                    (remaining[..end_quote].to_string(), true)
                } else {
                    (remaining.to_string(), true)
                }
            } else {
                // Unquoted label: <<<TEXT
                let label_end = after_spaces
                    .find(|c: char| c.is_whitespace() || c == ';')
                    .unwrap_or(after_spaces.len());
                (after_spaces[..label_end].to_string(), false)
            }
        } else {
            return SpanShape {
                outer: (start as u32, end as u32),
                inner: (start as u32, end as u32),
            };
        };
        
        // Find the first newline after the opening marker
        if let Some(first_newline_offset) = text.find('\n') {
            let body_start = start + first_newline_offset + 1;
            
            // Find the closing marker
            let after_body = &source[body_start..];
            let mut body_end = end;
            
            for (line_offset, line) in after_body.split('\n').enumerate() {
                let line_trimmed = line.trim_end();
                // Check if this line is just the closing marker (with optional semicolon)
                if line_trimmed == label || line_trimmed == format!("{};", label) {
                    // Calculate the position of this line's start
                    let lines_before: Vec<_> = after_body.split('\n').take(line_offset).collect();
                    let chars_before: usize = lines_before.iter().map(|l| l.len() + 1).sum(); // +1 for \n
                    body_end = body_start + chars_before;
                    break;
                }
            }
            
            return SpanShape {
                outer: (start as u32, end as u32),
                inner: (body_start as u32, body_end as u32),
            };
        }
    }
    
    // Fallback
    let outer = (start as u32, end as u32);
    SpanShape {
        outer,
        inner: outer,
    }
}

/// Detect if this is a flexible heredoc (PHP 7.3+) and get its info
pub fn get_heredoc_info(node: &Node, source: &str) -> Option<HeredocInfo> {
    let kind = node.kind();
    if kind != "heredoc" && kind != "nowdoc" {
        return None;
    }
    
    let start = node.start_byte();
    let end = node.end_byte();
    let text = &source[start..end.min(source.len())];
    
    // Find opening marker
    let marker_start = text.find("<<<")?;
    let after_marker = &text[marker_start + 3..];
    
    // Extract label
    let label_start_offset = after_marker.len() - after_marker.trim_start().len();
    let after_spaces = &after_marker[label_start_offset..];
    
    let label = if let Some(first_char) = after_spaces.chars().next() {
        if first_char == '\'' || first_char == '"' {
            let remaining = &after_spaces[1..];
            if let Some(end_quote) = remaining.find(first_char) {
                remaining[..end_quote].to_string()
            } else {
                remaining.to_string()
            }
        } else {
            let label_end = after_spaces
                .find(|c: char| c.is_whitespace() || c == ';')
                .unwrap_or(after_spaces.len());
            after_spaces[..label_end].to_string()
        }
    } else {
        return None;
    };
    
    // Find body range
    let first_newline_offset = text.find('\n')?;
    let body_start = start + first_newline_offset + 1;
    let after_body = &source[body_start..];
    let mut body_end = end;
    let mut closing_indent = None;
    
    for (line_offset, line) in after_body.split('\n').enumerate() {
        let line_trimmed = line.trim_end();
        let trimmed_stripped = line_trimmed.trim_start();
        
        if trimmed_stripped == label || trimmed_stripped == format!("{};", label) {
            // Found closing marker - calculate its indentation
            let indent = line_trimmed.len() - trimmed_stripped.len();
            closing_indent = Some(indent);
            
            // Calculate body end position
            let lines_before: Vec<_> = after_body.split('\n').take(line_offset).collect();
            let chars_before: usize = lines_before.iter().map(|l| l.len() + 1).sum();
            body_end = body_start + chars_before;
            break;
        }
    }
    
    // If closing marker is indented, this is a flexible heredoc
    let strips_whitespace = closing_indent.unwrap_or(0) > 0;
    
    Some(HeredocInfo {
        strips_whitespace,
        body_start: body_start as u32,
        body_end: body_end as u32,
    })
}

/// Extract variables from interpolated string expressions (e.g., "Hello {$user}").
/// PHP uses various node types for string interpolation:
/// - Simple variables: {$var}
/// - Complex expressions: {$obj->prop} or {$arr['key']} or {$price > 100 ? 'expensive' : 'cheap'}
pub fn extract_interpolation_vars(node: &Node, source: &str) -> Vec<PromptVar> {
    let mut vars = Vec::new();
    let string_start = node.start_byte();
    let string_end = node.end_byte();
    
    // Scan through the string content looking for {$...} patterns
    // This handles all complex expressions by finding the brace pairs
    let bytes = source.as_bytes();
    let mut i = string_start;
    
    while i < string_end {
        // Look for opening brace followed by $
        if i + 1 < string_end && bytes[i] == b'{' && bytes[i + 1] == b'$' {
            // Find the matching closing brace
            let mut depth = 1;
            let mut j = i + 1;
            
            while j < string_end && depth > 0 {
                match bytes[j] {
                    b'{' => depth += 1,
                    b'}' => depth -= 1,
                    _ => {}
                }
                j += 1;
            }
            
            if depth == 0 {
                // Found a complete {$...} expression
                let outer_start = i as u32;
                let outer_end = j as u32;
                let inner_start = (i + 1) as u32;  // Skip the opening {
                let inner_end = (j - 1) as u32;     // Skip the closing }
                
                vars.push(PromptVar {
                    span: SpanShape {
                        outer: (outer_start, outer_end),
                        inner: (inner_start, inner_end),
                    },
                });
                
                i = j;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    
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
