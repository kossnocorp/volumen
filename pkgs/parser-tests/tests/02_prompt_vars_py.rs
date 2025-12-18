use indoc::indoc;
use insta::{assert_ron_snapshot, assert_snapshot, assert_toml_snapshot};

mod utils;
use utils::*;

#[test]
fn single_var() {
    ParseTest::test(
        Some("fstr"),
        &ParseTestLang::py(indoc! {r#"
            greeting_prompt = f"Welcome, {user}!"
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
                          start: 18,
                          end: 37,
                        ),
                        inner: Span(
                          start: 20,
                          end: 36,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 37,
                      ),
                      exp: "f\"Welcome, {user}!\"",
                      vars: [
                        PromptVar(
                          exp: "{user}",
                          span: SpanShape(
                            outer: Span(
                              start: 29,
                              end: 35,
                            ),
                            inner: Span(
                              start: 30,
                              end: 34,
                            ),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @r#"
                [[]]
                enclosure = 'greeting_prompt = f"Welcome, {user}!"'
                outer = 'f"Welcome, {user}!"'
                inner = 'Welcome, {user}!'

                [[vars]]
                outer = '{user}'
                inner = 'user'
                "#);
            }),

            interpolate: Some(Box::new(|interpolated| {
                assert_toml_snapshot!(interpolated, @"['Welcome, {0}!']");
            })),
        },
    );
}

#[test]
fn multiple_vars() {
    ParseTest::test(
        None,
        &ParseTestLang::py(indoc! {r#"
            user_prompt = f"Hello, {name}! How is the weather today in {city}?"
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
                          end: 67,
                        ),
                        inner: Span(
                          start: 16,
                          end: 66,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 67,
                      ),
                      exp: "f\"Hello, {name}! How is the weather today in {city}?\"",
                      vars: [
                        PromptVar(
                          exp: "{name}",
                          span: SpanShape(
                            outer: Span(
                              start: 23,
                              end: 29,
                            ),
                            inner: Span(
                              start: 24,
                              end: 28,
                            ),
                          ),
                        ),
                        PromptVar(
                          exp: "{city}",
                          span: SpanShape(
                            outer: Span(
                              start: 59,
                              end: 65,
                            ),
                            inner: Span(
                              start: 60,
                              end: 64,
                            ),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @r#"
                [[]]
                enclosure = 'user_prompt = f"Hello, {name}! How is the weather today in {city}?"'
                outer = 'f"Hello, {name}! How is the weather today in {city}?"'
                inner = 'Hello, {name}! How is the weather today in {city}?'

                [[vars]]
                outer = '{name}'
                inner = 'name'

                [[vars]]
                outer = '{city}'
                inner = 'city'
                "#);
            }),

            interpolate: Some(Box::new(|interpolated| {
                assert_toml_snapshot!(interpolated, @"['Hello, {0}! How is the weather today in {1}?']");
            })),
        },
    );
}
