use tree_sitter::{Node, Tree};
use volumen_parser_core::parse_annotation;
use volumen_types::{PromptAnnotation, Span};

#[derive(Debug, Clone)]
pub struct CommentNode {
    pub start: u32,
    pub end: u32,
    pub text: String,
}

pub struct CommentTracker {
    pub comments: Vec<CommentNode>,
    pub source: String,
}

impl CommentTracker {
    pub fn new(tree: &Tree, source: &str) -> Self {
        let mut comments = Vec::new();
        extract_comments_recursive(&tree.root_node(), source, &mut comments);

        Self {
            comments,
            source: source.to_string(),
        }
    }

    /// Finds contiguous comment blocks immediately before the statement (only whitespace between).
    /// Returns both individual comments and merged block if contains @prompt.
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

                // Check if there's only whitespace (blank lines are allowed in TypeScript)
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

    /// Collects adjacent leading comments regardless of @prompt.
    /// Used when there's an inline @prompt to also collect non-@prompt leading comments.
    pub fn collect_all_adjacent_leading(&self, stmt_start: u32) -> Vec<PromptAnnotation> {
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

                // Check if there's only whitespace (blank lines allowed in TypeScript)
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

        // Merge the entire block into a single annotation (even if no @prompt)
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
}

/// Recursively extract all comments from the tree.
fn extract_comments_recursive(node: &Node, source: &str, comments: &mut Vec<CommentNode>) {
    if node.kind() == "comment" {
        let text = node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
        comments.push(CommentNode {
            start: node.start_byte() as u32,
            end: node.end_byte() as u32,
            text,
        });
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            extract_comments_recursive(&cursor.node(), source, comments);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
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
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();

        let source = r#"// Comment 1
/* Comment 2 */
const x = 1;"#;

        let tree = parser.parse(source, None).unwrap();
        let tracker = CommentTracker::new(&tree, source);

        assert_eq!(tracker.comments.len(), 2);
        assert_eq!(tracker.comments[0].text, "// Comment 1");
        assert_eq!(tracker.comments[1].text, "/* Comment 2 */");
    }

    #[test]
    fn test_collect_adjacent_leading() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();

        let source = r#"// @prompt
// This is a prompt
const x = "hello";"#;

        let tree = parser.parse(source, None).unwrap();
        let tracker = CommentTracker::new(&tree, source);

        // Find the statement start - need to skip comment nodes
        let root = tree.root_node();
        let mut cursor = root.walk();
        cursor.goto_first_child();

        // Find the lexical_declaration (skip comments)
        let mut decl_stmt = None;
        loop {
            let node = cursor.node();
            if node.kind() == "lexical_declaration" {
                decl_stmt = Some(node);
                break;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }

        let declaration = decl_stmt.unwrap();
        let stmt_start = declaration.start_byte() as u32;

        let annotations = tracker.collect_adjacent_leading(stmt_start);

        assert_eq!(annotations.len(), 1);
        assert!(annotations[0].exp.contains("@prompt"));
        assert!(annotations[0].exp.contains("This is a prompt"));
    }

    #[test]
    fn test_collect_inline_prompt() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();

        let source = r#"const x = /* @prompt */ "hello";"#;

        let tree = parser.parse(source, None).unwrap();
        let tracker = CommentTracker::new(&tree, source);

        let root = tree.root_node();
        let mut cursor = root.walk();
        cursor.goto_first_child();

        // Find the lexical_declaration
        let mut decl_stmt = None;
        loop {
            let node = cursor.node();
            if node.kind() == "lexical_declaration" {
                decl_stmt = Some(node);
                break;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }

        let declaration = decl_stmt.unwrap();
        let stmt_start = declaration.start_byte() as u32;
        let stmt_end = declaration.end_byte() as u32;

        let annotations = tracker.collect_inline_prompt(stmt_start, stmt_end);

        assert_eq!(annotations.len(), 1);
        assert_eq!(annotations[0].exp, "/* @prompt */");
    }

    #[test]
    fn test_adjacent_with_gap() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();

        let source = r#"// @prompt

const x = "hello";"#;

        let tree = parser.parse(source, None).unwrap();
        let tracker = CommentTracker::new(&tree, source);

        let root = tree.root_node();
        let mut cursor = root.walk();
        cursor.goto_first_child();

        // Find the lexical_declaration
        let mut decl_stmt = None;
        loop {
            let node = cursor.node();
            if node.kind() == "lexical_declaration" {
                decl_stmt = Some(node);
                break;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }

        let declaration = decl_stmt.unwrap();
        let stmt_start = declaration.start_byte() as u32;

        let annotations = tracker.collect_adjacent_leading(stmt_start);

        // In TypeScript, blank lines don't break adjacency (unlike Python)
        assert_eq!(annotations.len(), 1);
    }
}
