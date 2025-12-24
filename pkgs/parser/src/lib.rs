use volumen_parser_core::VolumenParser;
use volumen_parser_cs::ParserCs;
use volumen_parser_go::ParserGo;
use volumen_parser_java::ParserJava;
use volumen_parser_php::ParserPhp;
use volumen_parser_py::ParserPy;
use volumen_parser_rb::ParserRb;
use volumen_parser_ts::ParserTs;
use volumen_types::*;

pub struct Parser {}

impl Parser {
    pub fn parse(source: &str, filename: &str) -> ParseResult {
        let lower = filename.to_ascii_lowercase();
        let ext = lower.rsplit('.').next().unwrap_or("");
        match ext {
            "js" | "jsx" | "mjs" | "mjsx" | "cjs" | "cjsx" | "ts" | "tsx" => {
                ParserTs::parse(source, filename)
            }

            "py" | "pyi" => ParserPy::parse(source, filename),

            "rb" | "ruby" => ParserRb::parse(source, filename),

            "php" => ParserPhp::parse(source, filename),

            "cs" => ParserCs::parse(source, filename),

            "go" => ParserGo::parse(source, filename),

            "java" => ParserJava::parse(source, filename),

            _ => ParseResult::ParseResultError(ParseResultError {
                state: ParseResultErrorStateError,
                error: format!("Unsupported file extension for file: {}", filename),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use volumen_parser_test::*;

    #[test]
    fn parse_js() {
        let js_source = indoc! { r#"
          // This is a comment
          const prompt = "Hello, {name}!";
        "# };
        let js_result = Parser::parse(js_source, "example.js");
        assert_prompts_size(js_result, 1);
    }

    #[test]
    fn parse_jsx() {
        let jsx_source = indoc! { r#"
          // This is a comment
          const prompt = "Hello, {name}!";
          const jsx = <div>{prompt}</div>;
        "# };
        let jsx_result = Parser::parse(jsx_source, "example.jsx");
        assert_prompts_size(jsx_result, 1);
    }

    #[test]
    fn parse_ts() {
        let ts_source = indoc! { r#"
          // This is a comment
          const prompt = "Hello, {name}!";
        "# };
        let ts_result = Parser::parse(ts_source, "example.ts");
        assert_prompts_size(ts_result, 1);
    }

    #[test]
    fn parse_tsx() {
        let tsx_source = indoc! { r#"
          // This is a comment
          const prompt = "Hello, {name}!";
          const tsx = <div>{prompt}</div>;
        "# };
        let tsx_result = Parser::parse(tsx_source, "example.tsx");
        assert_prompts_size(tsx_result, 1);
    }

    #[test]
    fn parse_mjs() {
        let mjs_source = r#"
// This is a comment
const prompt = "Hello, {name}!";
"#;
        let mjs_result = Parser::parse(mjs_source, "example.mjs");
        assert_prompts_size(mjs_result, 1);
    }

    #[test]
    fn parse_cjs() {
        let cjs_source = indoc! { r#"
          // This is a comment
          const prompt = "Hello, {name}!";
        "# };
        let cjs_result = Parser::parse(cjs_source, "example.cjs");
        assert_prompts_size(cjs_result, 1);
    }

    #[test]
    fn parse_py() {
        let py_source = indoc! { r#"
          # This is a comment
          prompt = "Hello, {name}!"
        "# };
        let py_result = Parser::parse(py_source, "example.py");
        assert_prompts_size(py_result, 1);
    }

    #[test]
    fn parse_pyi() {
        let pyi_source = indoc! { r#"
          # This is a comment
          prompt: str = "Hello, {name}!"
        "# };
        let pyi_result = Parser::parse(pyi_source, "example.pyi");
        assert_prompts_size(pyi_result, 1);
    }

    #[test]
    fn parse_mjsx() {
        let mjsx_source = indoc! { r#"
          // This is a comment
          const prompt = "Hello, {name}!";
        "# };
        let mjsx_result = Parser::parse(mjsx_source, "example.mjsx");
        assert_prompts_size(mjsx_result, 1);
    }

    #[test]
    fn parse_cjsx() {
        let cjsx_source = indoc! { r#"
          // This is a comment
          const prompt = "Hello, {name}!";
        "# };
        let cjsx_result = Parser::parse(cjsx_source, "example.cjsx");
        assert_prompts_size(cjsx_result, 1);
    }

    #[test]
    fn parse_rb_and_ruby() {
        let ruby_source = indoc! { r#"
          prompt = "Hello, {name}!"
        "# };

        for ext in ["rb", "ruby"] {
            let filename = format!("example.{ext}");
            let ruby_result = Parser::parse(ruby_source, &filename);
            assert_prompts_size(ruby_result, 1);
        }
    }

    #[test]
    fn parse_php() {
        let php_source = indoc! { r#"
          <?php
          $prompt = "Hello, {name}!";
        "# };
        let php_result = Parser::parse(php_source, "example.php");
        assert_prompts_size(php_result, 1);
    }

    #[test]
    fn parse_cs() {
        let cs_source = indoc! { r#"
          using System;

          public class Example {
              string prompt = "Hello, {name}!";
          }
        "# };
        let cs_result = Parser::parse(cs_source, "example.cs");
        assert_prompts_size(cs_result, 0);
    }

    #[test]
    fn parse_go() {
        let go_source = indoc! { r#"
          package main

          var prompt = "Hello, {name}!"
        "# };
        let go_result = Parser::parse(go_source, "example.go");
        assert_prompts_size(go_result, 0);
    }

    #[test]
    #[ignore = "Wasm tests fail, fix them first"]
    fn parse_java() {
        let java_source = indoc! { r#"
          class Example {
            String prompt = "Hello, {name}!";
          }
        "# };
        let java_result = Parser::parse(java_source, "Example.java");
        assert_prompts_size(java_result, 1);
    }

    #[test]
    fn unsupported_extension_returns_error() {
        let result = Parser::parse("prompt = \"Hello, {name}!\"", "example.txt");

        match result {
            ParseResult::ParseResultError(ParseResultError { state, error }) => {
                assert_eq!(state, ParseResultErrorStateError);
                assert_eq!(error, "Unsupported file extension for file: example.txt");
            }
            _ => panic!("Expected ParseResultError for unsupported extension"),
        }
    }
}

