use indoc::indoc;
use insta::{assert_ron_snapshot, assert_snapshot, assert_toml_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    // const

    ParseTest::test(
        Some("const"),
        &ParseTestLang::ts(indoc! {r#"
            const userPrompt = "You are a helpful assistant.";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 19,
                          end: 49,
                        ),
                        inner: Span(
                          start: 20,
                          end: 48,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 50,
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
                enclosure = 'const userPrompt = "You are a helpful assistant.";'
                outer = '"You are a helpful assistant."'
                inner = 'You are a helpful assistant.'
                vars = []
                "#);
            }),

            interpolate: None,
        },
    );

    // var

    ParseTest::test(
        Some("var"),
        &ParseTestLang::ts(indoc! {r#"
            var userPrompt = "You are a helpful assistant.";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 17,
                          end: 47,
                        ),
                        inner: Span(
                          start: 18,
                          end: 46,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 48,
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
                enclosure = 'var userPrompt = "You are a helpful assistant.";'
                outer = '"You are a helpful assistant."'
                inner = 'You are a helpful assistant.'
                vars = []
                "#);
            }),

            interpolate: None,
        },
    );

    // let

    ParseTest::test(
        Some("let"),
        &ParseTestLang::ts(indoc! {r#"
            let userPrompt = "You are a helpful assistant.";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 17,
                          end: 47,
                        ),
                        inner: Span(
                          start: 18,
                          end: 46,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 48,
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
                enclosure = 'let userPrompt = "You are a helpful assistant.";'
                outer = '"You are a helpful assistant."'
                inner = 'You are a helpful assistant.'
                vars = []
                "#);
            }),

            interpolate: None,
        },
    );
}

#[test]
fn literal() {
    ParseTest::test(
        None,
        &ParseTestLang::ts(indoc! {r#"
            const userPrompt = `Welcome, ${user}!`;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 19,
                          end: 38,
                        ),
                        inner: Span(
                          start: 20,
                          end: 37,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 39,
                      ),
                      exp: "`Welcome, ${user}!`",
                      vars: [
                        PromptVar(
                          exp: "${user}",
                          span: SpanShape(
                            outer: Span(
                              start: 29,
                              end: 36,
                            ),
                            inner: Span(
                              start: 31,
                              end: 35,
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
                assert_toml_snapshot!(prompt_source_cuts, @r"
                [[]]
                enclosure = 'const userPrompt = `Welcome, ${user}!`;'
                outer = '`Welcome, ${user}!`'
                inner = 'Welcome, ${user}!'

                [[vars]]
                outer = '${user}'
                inner = 'user'
                ");
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
        &ParseTestLang::ts(indoc! {r#"
            const userPrompt = `Hello, ${name}! How is the weather today in ${city}?`;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 19,
                          end: 73,
                        ),
                        inner: Span(
                          start: 20,
                          end: 72,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 74,
                      ),
                      exp: "`Hello, ${name}! How is the weather today in ${city}?`",
                      vars: [
                        PromptVar(
                          exp: "${name}",
                          span: SpanShape(
                            outer: Span(
                              start: 27,
                              end: 34,
                            ),
                            inner: Span(
                              start: 29,
                              end: 33,
                            ),
                          ),
                        ),
                        PromptVar(
                          exp: "${city}",
                          span: SpanShape(
                            outer: Span(
                              start: 64,
                              end: 71,
                            ),
                            inner: Span(
                              start: 66,
                              end: 70,
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
                assert_toml_snapshot!(prompt_source_cuts, @r"
                [[]]
                enclosure = 'const userPrompt = `Hello, ${name}! How is the weather today in ${city}?`;'
                outer = '`Hello, ${name}! How is the weather today in ${city}?`'
                inner = 'Hello, ${name}! How is the weather today in ${city}?'

                [[vars]]
                outer = '${name}'
                inner = 'name'

                [[vars]]
                outer = '${city}'
                inner = 'city'
                ");
            }),

            interpolate: Some(Box::new(|interpolated| {
                assert_toml_snapshot!(interpolated, @"['Hello, {0}! How is the weather today in {1}?']");
            })),
        },
    );
}
