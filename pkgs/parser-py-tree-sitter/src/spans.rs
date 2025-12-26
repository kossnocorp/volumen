use tree_sitter::Node;
use volumen_types::{PromptVar, Span, SpanShape};

/// Calculate outer and inner spans for Python string literals.
/// Handles string prefixes (f, r, u, fr, rf, etc.) and triple-quoted strings.
pub fn span_shape_string_like(node: &Node, source: &str) -> SpanShape {
    let bytes = source.as_bytes();
    let start = node.start_byte();
    let end = node.end_byte();

    // Find first quote character (after any prefix like f, r, etc.)
    let mut i = start;
    while i < end {
        let c = bytes[i];
        if c == b'\'' || c == b'\"' {
            break;
        }
        i += 1;
    }

    let quote_pos = i;
    let quote_char = if quote_pos < end {
        bytes[quote_pos]
    } else {
        b'\''
    };

    // Detect triple-quoted strings
    let mut quote_len = 1u32;
    if quote_pos + 2 < end
        && bytes[quote_pos + 1] == quote_char
        && bytes[quote_pos + 2] == quote_char
    {
        quote_len = 3;
    }

    let outer = (start as u32, end as u32);

    let inner = (
        (quote_pos as u32).saturating_add(quote_len),
        outer.1.saturating_sub(quote_len),
    );

    SpanShape { outer, inner }
}

/// Extract variables from f-string interpolations.
/// Tree-sitter parses f-strings with `interpolation` nodes that contain the expressions.
pub fn extract_fstring_vars(node: &Node, source: &str) -> Vec<PromptVar> {
    let mut vars = Vec::new();
    let bytes = source.as_bytes();

    // Walk through all child nodes to find interpolations
    let mut cursor = node.walk();
    if !cursor.goto_first_child() {
        return vars;
    }

    loop {
        let child = cursor.node();

        // Tree-sitter represents f-string interpolations as "interpolation" nodes
        if child.kind() == "interpolation" {
            // The interpolation includes the { and }
            let outer_start = child.start_byte();
            let outer_end = child.end_byte();

            // Find the expression inside by skipping the opening {
            let mut expr_start = outer_start + 1;
            let mut expr_end = outer_end;

            // Skip whitespace after {
            while expr_start < outer_end && bytes[expr_start].is_ascii_whitespace() {
                expr_start += 1;
            }

            // Find the closing } and work backwards
            if outer_end > 0 {
                expr_end = outer_end - 1;
                // Skip whitespace before }
                while expr_end > expr_start && bytes[expr_end - 1].is_ascii_whitespace() {
                    expr_end -= 1;
                }
            }

            let outer = (outer_start as u32, outer_end as u32);

            let inner = (expr_start as u32, expr_end as u32);

            let exp = &source[outer_start..outer_end];

            vars.push(PromptVar {
                exp: exp.to_string(),
                span: SpanShape { outer, inner },
            });
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }

    vars
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::{Node, Parser};

    fn find_node_by_kind<'a>(node: &Node<'a>, kind: &str) -> Option<Node<'a>> {
        if node.kind() == kind {
            return Some(*node);
        }

        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if let Some(found) = find_node_by_kind(&child, kind) {
                    return Some(found);
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        None
    }

    #[test]
    fn test_span_shape_string_simple() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .unwrap();

        let source = r#""hello""#;
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        // Find the string node: module > expression_statement > string
        let string_node = find_node_by_kind(&root, "string").unwrap();

        let span = span_shape_string_like(&string_node, source);

        assert_eq!(span.outer.0, 0);
        assert_eq!(span.outer.1, 7);
        assert_eq!(span.inner.0, 1);
        assert_eq!(span.inner.1, 6);
    }

    #[test]
    fn test_span_shape_fstring() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .unwrap();

        let source = r#"f"hello {name}""#;
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        // Find the string node: module > expression_statement > string
        let string_node = find_node_by_kind(&root, "string").unwrap();

        let span = span_shape_string_like(&string_node, source);

        assert_eq!(span.outer.0, 0);
        assert_eq!(span.outer.1, 15);
        assert_eq!(span.inner.0, 2); // After f"
        assert_eq!(span.inner.1, 14); // Before "
    }

    #[test]
    fn test_extract_fstring_vars_single() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .unwrap();

        let source = r#"f"Hello {name}""#;
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        // Find the string node: module > expression_statement > string
        let string_node = find_node_by_kind(&root, "string").unwrap();

        let vars = extract_fstring_vars(&string_node, source);

        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0].exp, "{name}");
        assert_eq!(vars[0].span.outer.0, 8);
        assert_eq!(vars[0].span.outer.1, 14);
    }

    #[test]
    fn test_extract_fstring_vars_multiple() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .unwrap();

        let source = r#"f"Hello {first} {last}""#;
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        // Find the string node: module > expression_statement > string
        let string_node = find_node_by_kind(&root, "string").unwrap();

        let vars = extract_fstring_vars(&string_node, source);

        assert_eq!(vars.len(), 2);
        assert_eq!(vars[0].exp, "{first}");
        assert_eq!(vars[1].exp, "{last}");
    }
}
