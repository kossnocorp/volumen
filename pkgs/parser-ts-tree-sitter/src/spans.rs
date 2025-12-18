use tree_sitter::Node;
use volumen_types::{PromptVar, Span, SpanShape};

/// Calculate span shape for string literals and template strings.
pub fn span_shape_string_like(node: &Node, source: &str) -> SpanShape {
    let outer_start = node.start_byte() as u32;
    let outer_end = node.end_byte() as u32;

    // For template strings, we need to find the actual content between ` and `
    // For regular strings, content is between " and " or ' and '
    let text = node.utf8_text(source.as_bytes()).unwrap_or("");

    // Determine quote type and calculate inner span
    let (inner_start, inner_end) = if text.starts_with('`') {
        // Template string: `content`
        (outer_start + 1, outer_end - 1)
    } else if text.starts_with('"') || text.starts_with('\'') {
        // Regular string: "content" or 'content'
        (outer_start + 1, outer_end - 1)
    } else {
        // Fallback
        (outer_start, outer_end)
    };

    SpanShape {
        outer: Span {
            start: outer_start,
            end: outer_end,
        },
        inner: Span {
            start: inner_start,
            end: inner_end,
        },
    }
}

/// Extract variables from template string expressions.
pub fn extract_template_vars(node: &Node, source: &str) -> Vec<PromptVar> {
    let mut vars = Vec::new();

    // Walk through children to find template_substitution nodes
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            if child.kind() == "template_substitution" {
                // Template substitution includes ${ and }
                let outer_start = child.start_byte() as u32;
                let outer_end = child.end_byte() as u32;

                // Inner content is between ${ and }
                let inner_start = outer_start + 2; // Skip ${
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

/// Check if a node is a template string (has template_substitution children).
pub fn is_template_string(node: &Node) -> bool {
    if node.kind() != "template_string" {
        return false;
    }

    // Check if it has any template_substitution children
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            if cursor.node().kind() == "template_substitution" {
                return true;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    false
}

/// Check if a node is a string-like node (string or template_string).
pub fn is_string_like(node: &Node) -> bool {
    matches!(node.kind(), "string" | "template_string")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    #[test]
    fn test_span_shape_string_simple() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();

        let source = r#""hello""#;
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        // Find the string node: program > expression_statement > string
        let mut cursor = root.walk();
        cursor.goto_first_child(); // Go to expression_statement
        cursor.goto_first_child(); // Go to string
        let string_node = cursor.node();

        let span = span_shape_string_like(&string_node, source);

        assert_eq!(span.outer.start, 0);
        assert_eq!(span.outer.end, 7);
        assert_eq!(span.inner.start, 1);
        assert_eq!(span.inner.end, 6);
    }

    #[test]
    fn test_span_shape_template() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();

        let source = r#"`hello ${name}`"#;
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        // Find the template_string node
        let mut cursor = root.walk();
        cursor.goto_first_child(); // Go to expression_statement
        cursor.goto_first_child(); // Go to template_string
        let string_node = cursor.node();

        let span = span_shape_string_like(&string_node, source);

        assert_eq!(span.outer.start, 0);
        assert_eq!(span.outer.end, 15);
        assert_eq!(span.inner.start, 1);
        assert_eq!(span.inner.end, 14);
    }

    #[test]
    fn test_extract_template_vars_single() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();

        let source = r#"`Hello ${name}`"#;
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        // Find the template_string node
        let mut cursor = root.walk();
        cursor.goto_first_child(); // Go to expression_statement
        cursor.goto_first_child(); // Go to template_string
        let string_node = cursor.node();

        let vars = extract_template_vars(&string_node, source);

        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0].exp, "${name}");
        assert_eq!(vars[0].span.outer.start, 7);
        assert_eq!(vars[0].span.outer.end, 14);
    }

    #[test]
    fn test_extract_template_vars_multiple() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();

        let source = r#"`Hello ${first} ${last}`"#;
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        // Find the template_string node
        let mut cursor = root.walk();
        cursor.goto_first_child(); // Go to expression_statement
        cursor.goto_first_child(); // Go to template_string
        let string_node = cursor.node();

        let vars = extract_template_vars(&string_node, source);

        assert_eq!(vars.len(), 2);
        assert_eq!(vars[0].exp, "${first}");
        assert_eq!(vars[1].exp, "${last}");
    }
}
