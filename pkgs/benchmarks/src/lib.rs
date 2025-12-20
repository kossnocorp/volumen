use tree_sitter::{Node, Parser};

/// Recursively count all nodes in a tree-sitter syntax tree
pub fn count_tree_nodes(node: &Node) -> u64 {
    let mut count = 1u64; // Count current node
    let mut cursor = node.walk();

    if cursor.goto_first_child() {
        loop {
            count += count_tree_nodes(&cursor.node());
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    count
}

/// Count tokens in a code sample for a specific language
pub fn count_tokens(code: &str, lang: &str) -> Result<u64, String> {
    let mut parser = Parser::new();

    let language = match lang {
        "go" => tree_sitter_go::LANGUAGE,
        "java" => tree_sitter_java::LANGUAGE,
        "csharp" => tree_sitter_c_sharp::LANGUAGE,
        "php" => tree_sitter_php::LANGUAGE_PHP,
        "ruby" => tree_sitter_ruby::LANGUAGE,
        "python" => tree_sitter_python::LANGUAGE,
        "typescript" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
        _ => return Err(format!("Unknown language: {}", lang)),
    };

    parser
        .set_language(&language.into())
        .map_err(|e| format!("Failed to set language: {:?}", e))?;

    let tree = parser
        .parse(code, None)
        .ok_or_else(|| "Failed to parse code".to_string())?;

    Ok(count_tree_nodes(&tree.root_node()))
}
