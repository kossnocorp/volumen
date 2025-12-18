/// Tree-sitter query patterns for Python parser.
/// These queries use the tree-sitter query syntax to pattern-match against the AST.

/// Query to find regular assignment statements.
/// Matches: variable = value
pub const ASSIGNMENT_QUERY: &str = r#"
(assignment
  left: (identifier) @var_name
  right: [
    (string) @value
    (concatenated_string) @value
  ]) @assignment
"#;

/// Query to find annotated assignment statements (type hints).
/// Matches: variable: Type = value
pub const ANNOTATED_ASSIGNMENT_QUERY: &str = r#"
(assignment
  left: (identifier) @var_name
  type: (_)
  right: [
    (string) @value
    (concatenated_string) @value
  ]?) @assignment
"#;

/// Query to find tuple/list unpacking assignments.
/// Matches: a, b = values or [a, b] = values
pub const MULTI_ASSIGNMENT_QUERY: &str = r#"
(assignment
  left: [
    (pattern_list) @targets
    (tuple_pattern) @targets
  ]
  right: (_) @values) @assignment
"#;

/// Query to find function definitions (scope boundaries).
pub const FUNCTION_DEF_QUERY: &str = r#"
[
  (function_definition) @function
  (decorated_definition
    (function_definition) @function)
] @scope
"#;

/// Query to find class definitions (scope boundaries).
pub const CLASS_DEF_QUERY: &str = r#"
[
  (class_definition) @class
  (decorated_definition
    (class_definition) @class)
] @scope
"#;

/// Combined scope query for all scope boundaries.
pub const SCOPE_QUERY: &str = r#"
[
  (function_definition) @scope
  (class_definition) @scope
  (decorated_definition) @scope
]
"#;

/// Query to find all assignments (including chained assignments).
/// Matches: a = b = value
pub const ALL_ASSIGNMENTS_QUERY: &str = r#"
(assignment) @assignment
"#;

/// Query to find expression statements (for detecting assignment expressions).
pub const EXPRESSION_STATEMENT_QUERY: &str = r#"
(expression_statement) @expr_stmt
"#;
