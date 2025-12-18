use indoc::indoc;
use insta::{assert_ron_snapshot, assert_snapshot, assert_toml_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        Some("str"),
        &ParseTestLang::py(indoc! {r#"
            user_prompt = "You are a helpful assistant."
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 14,
                          end: 44,
                        ),
                        inner: Span(
                          start: 15,
                          end: 43,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 44,
                      ),
                      exp: "\"You are a helpful assistant.\"",
                      vars: [],
                      annotations: [],
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @r#"
                [[]]
                enclosure = 'user_prompt = "You are a helpful assistant."'
                outer = '"You are a helpful assistant."'
                inner = 'You are a helpful assistant.'
                vars = []
                "#);
            }),

            interpolate: None,
        },
    );
}