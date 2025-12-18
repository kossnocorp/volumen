use volumen_parser_core::VolumenParser;
use volumen_parser_py::ParserPy;
use volumen_parser_ts::ParserTs;
use volumen_types::*;

pub struct Parser {}

impl Parser {
    pub fn parse(source: &str, filename: &str) -> ParseResult {
        let lower = filename.to_ascii_lowercase();
        if lower.ends_with(".py") || lower.ends_with(".pyi") {
            ParserPy::parse(source, filename)
        } else {
            ParserTs::parse(source, filename)
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
}
