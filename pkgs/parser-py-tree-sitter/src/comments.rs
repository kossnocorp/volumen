use tree_sitter::{Node, Tree};
use volumen_parser_core::parse_annotation;
use volumen_types::{PromptAnnotation, Span};

/// Recursively extract all comment nodes from the tree.
fn extract_comments_recursive(node: &Node, source: &str, comments: &mut Vec<CommentNode>) {
    if node.kind() == "comment" {
        comments.push(CommentNode {
            start: node.start_byte() as u32,
            end: node.end_byte() as u32,
            text: node
                .utf8_text(source.as_bytes())
                .unwrap_or("")
                .to_string(),
        });
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            extract_comments_recursive(&child, source, comments);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Represents a comment node extracted from the syntax tree.
#[derive(Clone, Debug)]
pub struct CommentNode {
    pub start: u32,
    pub end: u32,
    pub text: String,
}

/// Tracks and manages comments from the parsed tree.
pub struct CommentTracker<'a> {
    source: &'a str,
    comments: Vec<CommentNode>,
}

impl<'a> CommentTracker<'a> {
    /// Create a new CommentTracker by extracting all comments from the tree.
    pub fn new(tree: &Tree, source: &'a str) -> Self {
        let root = tree.root_node();
        let mut comments = Vec::new();

        // Traverse the tree to find all comment nodes
        extract_comments_recursive(&root, source, &mut comments);

        // Sort comments by start position for efficient searching
        comments.sort_by_key(|c| c.start);

        Self { source, comments }
    }

    /// Collect adjacent leading comments for a statement.
    /// Finds contiguous comment blocks immediately before the statement (only whitespace between).
    pub fn collect_adjacent_leading(&self, stmt_start: u32) -> Vec<PromptAnnotation> {
        let mut block_ranges: Vec<&CommentNode> = Vec::new();

        // Find comments that end before the statement starts
        for comment in self.comments.iter().rev() {
            if comment.end <= stmt_start {
                // Check if there's only whitespace between comment and statement
                let start = comment.end as usize;
                let end = stmt_start as usize;

                let between = if start <= end && end <= self.source.len() {
                    &self.source[start..end]
                } else {
                    ""
                };

                if between.trim().is_empty() {
                    // Found a comment adjacent to the statement
                    // Now collect the entire contiguous block
                    let mut last = stmt_start;
                    for c in self.comments.iter().rev() {
                        if c.end <= last {
                            let s = c.end as usize;
                            let e = last as usize;
                            let between2 = if s <= e && e <= self.source.len() {
                                &self.source[s..e]
                            } else {
                                ""
                            };

                            if between2.trim().is_empty() {
                                block_ranges.push(c);
                                last = c.start;
                            } else {
                                break;
                            }
                        }
                    }
                    block_ranges.reverse();
                }
                break;
            }
        }

        if block_ranges.is_empty() {
            return Vec::new();
        }

        // Check if any comment in the block contains @prompt
        let has_prompt = block_ranges.iter().any(|c| parse_annotation(&c.text).unwrap_or(false));
        if !has_prompt {
            return Vec::new();
        }

        // Merge the entire block into a single annotation
        let first = block_ranges.first().unwrap();
        let last = block_ranges.last().unwrap();
        let start = first.start;
        let end = last.end;
        let block_text = &self.source[start as usize..end as usize];

        vec![PromptAnnotation {
            span: Span { start, end },
            exp: block_text.to_string(),
        }]
    }

    /// Collect inline @prompt comments within a statement's range.
    pub fn collect_inline_prompt(&self, stmt_start: u32, stmt_end: u32) -> Vec<PromptAnnotation> {
        self.comments
            .iter()
            .filter(|c| c.start >= stmt_start && c.start < stmt_end)
            .filter(|c| parse_annotation(&c.text).unwrap_or(false))
            .map(|c| PromptAnnotation {
                span: Span {
                    start: c.start,
                    end: c.end,
                },
                exp: c.text.clone(),
            })
            .collect()
    }

    /// Get the start position of the first leading comment, if any.
    pub fn get_leading_start(&self, stmt_start: u32) -> Option<u32> {
        let annotations = self.collect_adjacent_leading(stmt_start);
        annotations.first().map(|a| a.span.start)
    }

    /// Get the start position of any adjacent leading comment, regardless of whether it's valid.
    /// Used for enclosure calculation when we want to include all leading comments.
    pub fn get_any_leading_start(&self, stmt_start: u32) -> Option<u32> {
        // Find comments that end before the statement starts
        for comment in self.comments.iter().rev() {
            if comment.end <= stmt_start {
                // Check if there's only whitespace between comment and statement
                let start = comment.end as usize;
                let end = stmt_start as usize;

                let between = if start <= end && end <= self.source.len() {
                    &self.source[start..end]
                } else {
                    ""
                };

                if between.trim().is_empty() {
                    // Found a comment adjacent to the statement
                    // Now find the start of the entire contiguous block
                    let mut block_start = comment.start;
                    let mut last = stmt_start;
                    for c in self.comments.iter().rev() {
                        if c.end <= last {
                            let s = c.end as usize;
                            let e = last as usize;
                            let between2 = if s <= e && e <= self.source.len() {
                                &self.source[s..e]
                            } else {
                                ""
                            };

                            if between2.trim().is_empty() {
                                block_start = c.start;
                                last = c.start;
                            } else {
                                break;
                            }
                        }
                    }
                    return Some(block_start);
                }
                break;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    #[test]
    fn test_extract_comments() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .unwrap();

        let source = r#"# Comment 1
# Comment 2
x = 1"#;

        let tree = parser.parse(source, None).unwrap();
        let tracker = CommentTracker::new(&tree, source);

        assert_eq!(tracker.comments.len(), 2);
        assert_eq!(tracker.comments[0].text, "# Comment 1");
        assert_eq!(tracker.comments[1].text, "# Comment 2");
    }

    #[test]
    fn test_collect_adjacent_leading() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .unwrap();

        let source = r#"# @prompt
# This is a prompt
x = "hello""#;

        let tree = parser.parse(source, None).unwrap();
        let tracker = CommentTracker::new(&tree, source);

        // Find the statement start - need to skip comment nodes
        let root = tree.root_node();
        let mut cursor = root.walk();
        cursor.goto_first_child();

        // Find the expression_statement (skip comments)
        let mut expr_stmt = None;
        loop {
            let node = cursor.node();
            if node.kind() == "expression_statement" {
                expr_stmt = Some(node);
                break;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }

        let assignment = expr_stmt.unwrap();
        let stmt_start = assignment.start_byte() as u32;

        let annotations = tracker.collect_adjacent_leading(stmt_start);

        assert_eq!(annotations.len(), 1);
        assert!(annotations[0].exp.contains("@prompt"));
        assert!(annotations[0].exp.contains("This is a prompt"));
    }

    #[test]
    fn test_collect_inline_prompt() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .unwrap();

        let source = r#"x = "hello"  # @prompt"#;

        let tree = parser.parse(source, None).unwrap();
        let tracker = CommentTracker::new(&tree, source);

        // Use the line start/end instead of the statement byte range
        // because inline comments appear after the statement
        let stmt_start = 0;
        let stmt_end = source.len() as u32;

        let annotations = tracker.collect_inline_prompt(stmt_start, stmt_end);

        assert_eq!(annotations.len(), 1);
        assert_eq!(annotations[0].exp, "# @prompt");
    }

    #[test]
    fn test_no_adjacent_with_gap() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .unwrap();

        let source = r#"# Comment

x = "hello""#;

        let tree = parser.parse(source, None).unwrap();
        let tracker = CommentTracker::new(&tree, source);

        let root = tree.root_node();
        let assignment = root.child(0).unwrap();
        let stmt_start = assignment.start_byte() as u32;

        let annotations = tracker.collect_adjacent_leading(stmt_start);

        // Should not find the comment because there's a blank line
        assert_eq!(annotations.len(), 0);
    }
}
