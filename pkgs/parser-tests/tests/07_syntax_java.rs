use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn invalid() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            String invalid = "unclosed string
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultError(
                  state: "error",
                  error: "<error>",
                )
                "#);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"[]");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"[]");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"[]");
            }),
        },
    );
}

#[test]
fn text_block() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String system = """
                You are a helpful assistant.
                """;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
                      span: SpanShape(
                        outer: Span(
                          start: 27,
                          end: 71,
                        ),
                        inner: Span(
                          start: 28,
                          end: 70,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 72,
                      ),
                      exp: "\"\"\"\n    You are a helpful assistant.\n    \"\"\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 10,
                          ),
                          exp: "// @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "#);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "// @prompt\nString system = \"\"\"\n    You are a helpful assistant.\n    \"\"\";",
                    "outer": "\"\"\"\n    You are a helpful assistant.\n    \"\"\"",
                    "inner": "\"\"\n    You are a helpful assistant.\n    \"\"",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "\"\"\n    You are a helpful assistant.\n    \"\""
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
                [
                  [
                    "// @prompt"
                  ]
                ]
                "#);
            }),
        },
    );
}
