use volumen_benchmarks::count_tokens;

// Go samples
const SMALL_GO: &str = include_str!("../../benches/go_parsers.rs");
const MEDIUM_GO: &str = include_str!("../../benches/go_parsers.rs");
const LARGE_GO: &str = include_str!("../../benches/go_parsers.rs");

// Java samples
const SMALL_JAVA: &str = include_str!("../../benches/java_parsers.rs");
const MEDIUM_JAVA: &str = include_str!("../../benches/java_parsers.rs");
const LARGE_JAVA: &str = include_str!("../../benches/java_parsers.rs");

// C# samples
const SMALL_CSHARP: &str = include_str!("../../benches/csharp_parsers.rs");
const MEDIUM_CSHARP: &str = include_str!("../../benches/csharp_parsers.rs");
const LARGE_CSHARP: &str = include_str!("../../benches/csharp_parsers.rs");

// PHP samples
const SMALL_PHP: &str = include_str!("../../benches/php_parsers.rs");
const MEDIUM_PHP: &str = include_str!("../../benches/php_parsers.rs");
const LARGE_PHP: &str = include_str!("../../benches/php_parsers.rs");

// Ruby samples
const SMALL_RUBY: &str = include_str!("../../benches/ruby_parsers.rs");
const MEDIUM_RUBY: &str = include_str!("../../benches/ruby_parsers.rs");
const LARGE_RUBY: &str = include_str!("../../benches/ruby_parsers.rs");

// Python samples
const SMALL_PYTHON: &str = include_str!("../../benches/python_parsers.rs");
const MEDIUM_PYTHON: &str = include_str!("../../benches/python_parsers.rs");
const LARGE_PYTHON: &str = include_str!("../../benches/python_parsers.rs");

// TypeScript samples
const SMALL_TYPESCRIPT: &str = include_str!("../../benches/typescript_parsers.rs");
const MEDIUM_TYPESCRIPT: &str = include_str!("../../benches/typescript_parsers.rs");
const LARGE_TYPESCRIPT: &str = include_str!("../../benches/typescript_parsers.rs");

fn extract_code_sample(file_content: &str, const_name: &str) -> Option<String> {
    // Find the const declaration
    let pattern = format!("const {}: &str = r#\"", const_name);
    let start_idx = file_content.find(&pattern)?;
    let code_start = start_idx + pattern.len();

    // Find the closing \"#;
    let remaining = &file_content[code_start..];
    let end_idx = remaining.find("\"#;")?;

    Some(remaining[..end_idx].to_string())
}

fn main() {
    println!("{{");

    // Go
    println!("  \"go_parsers\": {{");
    println!("    \"Tree-sitter\": {{");
    if let Some(code) = extract_code_sample(SMALL_GO, "SMALL_GO") {
        if let Ok(tokens) = count_tokens(&code, "go") {
            println!("      \"small\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(MEDIUM_GO, "MEDIUM_GO") {
        if let Ok(tokens) = count_tokens(&code, "go") {
            println!("      \"medium\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(LARGE_GO, "LARGE_GO") {
        if let Ok(tokens) = count_tokens(&code, "go") {
            println!("      \"large\": {}", tokens);
        }
    }
    println!("    }}");
    println!("  }},");

    // Java
    println!("  \"java_parsers\": {{");
    println!("    \"Tree-sitter\": {{");
    if let Some(code) = extract_code_sample(SMALL_JAVA, "SMALL_JAVA") {
        if let Ok(tokens) = count_tokens(&code, "java") {
            println!("      \"small\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(MEDIUM_JAVA, "MEDIUM_JAVA") {
        if let Ok(tokens) = count_tokens(&code, "java") {
            println!("      \"medium\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(LARGE_JAVA, "LARGE_JAVA") {
        if let Ok(tokens) = count_tokens(&code, "java") {
            println!("      \"large\": {}", tokens);
        }
    }
    println!("    }}");
    println!("  }},");

    // C#
    println!("  \"csharp_parsers\": {{");
    println!("    \"Tree-sitter\": {{");
    if let Some(code) = extract_code_sample(SMALL_CSHARP, "SMALL_CSHARP") {
        if let Ok(tokens) = count_tokens(&code, "csharp") {
            println!("      \"small\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(MEDIUM_CSHARP, "MEDIUM_CSHARP") {
        if let Ok(tokens) = count_tokens(&code, "csharp") {
            println!("      \"medium\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(LARGE_CSHARP, "LARGE_CSHARP") {
        if let Ok(tokens) = count_tokens(&code, "csharp") {
            println!("      \"large\": {}", tokens);
        }
    }
    println!("    }}");
    println!("  }},");

    // PHP
    println!("  \"php_parsers\": {{");
    println!("    \"Tree-sitter\": {{");
    if let Some(code) = extract_code_sample(SMALL_PHP, "SMALL_PHP") {
        if let Ok(tokens) = count_tokens(&code, "php") {
            println!("      \"small\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(MEDIUM_PHP, "MEDIUM_PHP") {
        if let Ok(tokens) = count_tokens(&code, "php") {
            println!("      \"medium\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(LARGE_PHP, "LARGE_PHP") {
        if let Ok(tokens) = count_tokens(&code, "php") {
            println!("      \"large\": {}", tokens);
        }
    }
    println!("    }}");
    println!("  }},");

    // Ruby
    println!("  \"ruby_parsers\": {{");
    println!("    \"Tree-sitter\": {{");
    if let Some(code) = extract_code_sample(SMALL_RUBY, "SMALL_RUBY") {
        if let Ok(tokens) = count_tokens(&code, "ruby") {
            println!("      \"small\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(MEDIUM_RUBY, "MEDIUM_RUBY") {
        if let Ok(tokens) = count_tokens(&code, "ruby") {
            println!("      \"medium\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(LARGE_RUBY, "LARGE_RUBY") {
        if let Ok(tokens) = count_tokens(&code, "ruby") {
            println!("      \"large\": {}", tokens);
        }
    }
    println!("    }}");
    println!("  }},");

    // Python (3 parsers - all parse the same code)
    println!("  \"python_parsers\": {{");
    println!("    \"Ruff\": {{");
    if let Some(code) = extract_code_sample(SMALL_PYTHON, "SMALL_PYTHON") {
        if let Ok(tokens) = count_tokens(&code, "python") {
            println!("      \"small\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(MEDIUM_PYTHON, "MEDIUM_PYTHON") {
        if let Ok(tokens) = count_tokens(&code, "python") {
            println!("      \"medium\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(LARGE_PYTHON, "LARGE_PYTHON") {
        if let Ok(tokens) = count_tokens(&code, "python") {
            println!("      \"large\": {}", tokens);
        }
    }
    println!("    }},");
    println!("    \"RustPython\": {{");
    if let Some(code) = extract_code_sample(SMALL_PYTHON, "SMALL_PYTHON") {
        if let Ok(tokens) = count_tokens(&code, "python") {
            println!("      \"small\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(MEDIUM_PYTHON, "MEDIUM_PYTHON") {
        if let Ok(tokens) = count_tokens(&code, "python") {
            println!("      \"medium\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(LARGE_PYTHON, "LARGE_PYTHON") {
        if let Ok(tokens) = count_tokens(&code, "python") {
            println!("      \"large\": {}", tokens);
        }
    }
    println!("    }},");
    println!("    \"Tree-sitter\": {{");
    if let Some(code) = extract_code_sample(SMALL_PYTHON, "SMALL_PYTHON") {
        if let Ok(tokens) = count_tokens(&code, "python") {
            println!("      \"small\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(MEDIUM_PYTHON, "MEDIUM_PYTHON") {
        if let Ok(tokens) = count_tokens(&code, "python") {
            println!("      \"medium\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(LARGE_PYTHON, "LARGE_PYTHON") {
        if let Ok(tokens) = count_tokens(&code, "python") {
            println!("      \"large\": {}", tokens);
        }
    }
    println!("    }}");
    println!("  }},");

    // TypeScript (2 parsers - both parse the same code)
    println!("  \"typescript_parsers\": {{");
    println!("    \"Oxc\": {{");
    if let Some(code) = extract_code_sample(SMALL_TYPESCRIPT, "SMALL_TYPESCRIPT") {
        if let Ok(tokens) = count_tokens(&code, "typescript") {
            println!("      \"small\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(MEDIUM_TYPESCRIPT, "MEDIUM_TYPESCRIPT") {
        if let Ok(tokens) = count_tokens(&code, "typescript") {
            println!("      \"medium\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(LARGE_TYPESCRIPT, "LARGE_TYPESCRIPT") {
        if let Ok(tokens) = count_tokens(&code, "typescript") {
            println!("      \"large\": {}", tokens);
        }
    }
    println!("    }},");
    println!("    \"Tree-sitter\": {{");
    if let Some(code) = extract_code_sample(SMALL_TYPESCRIPT, "SMALL_TYPESCRIPT") {
        if let Ok(tokens) = count_tokens(&code, "typescript") {
            println!("      \"small\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(MEDIUM_TYPESCRIPT, "MEDIUM_TYPESCRIPT") {
        if let Ok(tokens) = count_tokens(&code, "typescript") {
            println!("      \"medium\": {},", tokens);
        }
    }
    if let Some(code) = extract_code_sample(LARGE_TYPESCRIPT, "LARGE_TYPESCRIPT") {
        if let Ok(tokens) = count_tokens(&code, "typescript") {
            println!("      \"large\": {}", tokens);
        }
    }
    println!("    }}");
    println!("  }}");

    println!("}}");
}
