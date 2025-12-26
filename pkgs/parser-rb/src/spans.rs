use tree_sitter::Node;
use volumen_types::{PromptVar, Span, SpanShape};

/// Calculate outer and inner spans for a string-like node.
/// For Ruby, this handles single-quoted, double-quoted strings, and heredocs.
pub fn span_shape_string_like(node: &Node, source: &str) -> SpanShape {
    let start = node.start_byte();
    let end = node.end_byte();

    // Handle heredocs by using the body range for the inner span
    let kind = node.kind();
    if kind == "string" {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "heredoc_body" {
                    let outer = (start as u32, end as u32);

                    let inner = (child.start_byte() as u32, child.end_byte() as u32);

                    return SpanShape { outer, inner };
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }

    if kind == "heredoc_beginning" {
        let text = &source[start..end.min(source.len())];
        let mut label = "";

        if let Some(pos) = text.find("<<") {
            let mut label_start = pos + 2;
            if text[label_start..].starts_with('~') || text[label_start..].starts_with('-') {
                label_start += 1;
            }

            let mut label_chars = text[label_start..].chars();
            if let Some(first) = label_chars.next() {
                if first == '\'' || first == '"' {
                    let remaining = &text[label_start + 1..];
                    if let Some(end_quote) = remaining.find(first) {
                        label = &remaining[..end_quote];
                    } else {
                        label = remaining;
                    }
                } else {
                    let rest = &text[label_start..];
                    let end_idx = rest.find(|c: char| c.is_whitespace()).unwrap_or(rest.len());
                    label = &rest[..end_idx];
                }
            }
        }

        if !label.is_empty() {
            let after_begin = &source[end.min(source.len())..];
            if let Some(first_newline) = after_begin.find('\n') {
                let mut body_start = end + first_newline + 1;
                let mut body_end = source.len();

                let mut cursor = body_start;
                for line in source[body_start..].split_inclusive('\n') {
                    let line_without_newline = line.trim_end_matches('\n');
                    if line_without_newline.trim() == label {
                        body_end = cursor;
                        break;
                    }
                    cursor += line.len();
                }

                let outer = (start as u32, body_end as u32);
                let inner = (body_start as u32, body_end as u32);

                return SpanShape { outer, inner };
            }
        }

        let fallback = (start as u32, end as u32);
        return SpanShape {
            outer: fallback.clone(),
            inner: fallback,
        };
    }

    if kind == "heredoc_body" {
        let outer = (start as u32, end as u32);
        return SpanShape {
            outer: outer.clone(),
            inner: outer,
        };
    }

    // For percent strings, skip the leading "%q"/"%Q" and the closing delimiter
    let bytes = source.as_bytes();
    if start + 2 < bytes.len()
        && bytes[start] == b'%'
        && (bytes[start + 1] == b'q' || bytes[start + 1] == b'Q')
    {
        let delimiter = bytes[start + 2];
        let closing = match delimiter {
            b'(' => b')',
            b'[' => b']',
            b'{' => b'}',
            b'<' => b'>',
            _ => delimiter,
        };

        let inner_start = start as u32 + 3; // %, q/Q, delimiter
        let mut inner_end = end as u32;

        if end > start + 2 && bytes[end - 1] == closing {
            inner_end -= 1;
        }

        return SpanShape {
            outer: (start as u32, end as u32),
            inner: (inner_start, inner_end),
        };
    }

    // For quoted Ruby strings, skip surrounding quotes
    let mut quote_len = 1u32;

    if start < bytes.len() {
        let first_char = bytes[start];
        if first_char == b'\'' || first_char == b'"' {
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

    let outer = (start as u32, end as u32);

    let inner = (
        (start as u32).saturating_add(quote_len),
        (end as u32).saturating_sub(quote_len),
    );

    SpanShape { outer, inner }
}

/// Extract variables from interpolated string expressions (e.g., "Hello #{name}").
pub fn extract_interpolation_vars(node: &Node, source: &str) -> Vec<PromptVar> {
    let mut vars = Vec::new();

    fn walk(node: &Node, source: &str, vars: &mut Vec<PromptVar>) {
        if node.kind() == "interpolation" {
            let outer_start = node.start_byte() as u32;
            let outer_end = node.end_byte() as u32;

            let inner_start = outer_start + 2; // Skip #{
            let inner_end = outer_end.saturating_sub(1); // Skip }

            let exp = source[outer_start as usize..outer_end as usize].to_string();

            vars.push(PromptVar {
                exp,
                span: SpanShape {
                    outer: (outer_start, outer_end),
                    inner: (inner_start, inner_end),
                },
            });
            return;
        }

        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                walk(&child, source, vars);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }

    walk(node, source, &mut vars);

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
    matches!(
        node.kind(),
        "string" | "string_content" | "heredoc_body" | "heredoc_beginning"
    )
}
