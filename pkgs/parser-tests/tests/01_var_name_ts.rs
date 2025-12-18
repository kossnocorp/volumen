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
fn inline() {
    ParseTest::test(
        Some("const"),
        &ParseTestLang::ts(indoc! {r#"
            /* @prompt */
            const hello = `Hello, world!`;
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
                          start: 28,
                          end: 43,
                        ),
                        inner: Span(
                          start: 29,
                          end: 42,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 44,
                      ),
                      exp: "`Hello, world!`",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 13,
                          ),
                          exp: "/* @prompt */",
                        ),
                      ],
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @r"
                [[]]
                enclosure = '''
                /* @prompt */
                const hello = `Hello, world!`;'''
                outer = '`Hello, world!`'
                inner = 'Hello, world!'
                vars = []
                ");
            }),

            interpolate: None,
        },
    );
}

#[test]
fn jsdoc() {
    ParseTest::test(
        Some("const"),
        &ParseTestLang::ts(indoc! {r#"
            /**
             * @prompt
             */
            const hello = `Hello, world!`;
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
                          start: 33,
                          end: 48,
                        ),
                        inner: Span(
                          start: 34,
                          end: 47,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 49,
                      ),
                      exp: "`Hello, world!`",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 18,
                          ),
                          exp: "/**\n * @prompt\n */",
                        ),
                      ],
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @r"
                [[]]
                enclosure = '''
                /**
                 * @prompt
                 */
                const hello = `Hello, world!`;'''
                outer = '`Hello, world!`'
                inner = 'Hello, world!'
                vars = []
                ");
            }),

            interpolate: None,
        },
    );
}
