use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

// Java doesn't have native string interpolation like JavaScript template literals,
// so these tests won't extract variables. They're included for completeness.

#[test]
fn single_var() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            String userPrompt = "Welcome, user!";
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
                          start: 20,
                          end: 36,
                        ),
                        inner: Span(
                          start: 21,
                          end: 35,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 37,
                      ),
                      exp: "\"Welcome, user!\"",
                      vars: [],
                      annotations: [],
                    ),
                  ],
                )
                "#);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "String userPrompt = \"Welcome, user!\";",
                    "outer": "\"Welcome, user!\"",
                    "inner": "Welcome, user!",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Welcome, user!"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r"
                [
                  []
                ]
                ");
            }),
        },
    );
}

#[test]
fn multiple_vars() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            String userPrompt = "Hello, name! How is the weather today in city?";
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
                          start: 20,
                          end: 68,
                        ),
                        inner: Span(
                          start: 21,
                          end: 67,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 69,
                      ),
                      exp: "\"Hello, name! How is the weather today in city?\"",
                      vars: [],
                      annotations: [],
                    ),
                  ],
                )
                "#);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "String userPrompt = \"Hello, name! How is the weather today in city?\";",
                    "outer": "\"Hello, name! How is the weather today in city?\"",
                    "inner": "Hello, name! How is the weather today in city?",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello, name! How is the weather today in city?"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r"
                [
                  []
                ]
                ");
            }),
        },
    );
}
