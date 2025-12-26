use tree_sitter::{Node, Tree};
use volumen_parser_core::parse_annotation;
use volumen_types::{PromptAnnotation, Span};

/// Recursively extract all comment nodes from the tree.
fn extract_comments_recursive(node: &Node, source: &str, comments: &mut Vec<CommentNode>) {
    if node.kind() == "comment" {
        comments.push(CommentNode {
            start: node.start_byte() as u32,
            end: node.end_byte() as u32,
            text: node.utf8_text(source.as_bytes()).unwrap_or("").to_string(),
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
pub struct CommentTracker {
    source: String,
    comments: Vec<CommentNode>,
}

impl CommentTracker {
    /// Create a new CommentTracker by extracting all comments from the tree.
    pub fn new(tree: &Tree, source: &str) -> Self {
        let root = tree.root_node();
        let mut comments = Vec::new();

        // Traverse the tree to find all comment nodes
        extract_comments_recursive(&root, source, &mut comments);

        // Sort comments by start position for efficient searching
        comments.sort_by_key(|c| c.start);

        Self {
            source: source.to_string(),
            comments,
        }
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
        let has_prompt = block_ranges
            .iter()
            .any(|c| parse_annotation(&c.text).unwrap_or(false));
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
            span: (start, end),
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
                span: (c.start, c.end),
                exp: c.text.clone(),
            })
            .collect()
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
