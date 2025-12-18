use indoc::indoc;
use insta::{assert_ron_snapshot, assert_snapshot, assert_toml_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        None,
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const system = "You are a helpful assistant.";
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
                          start: 26,
                          end: 56,
                        ),
                        inner: Span(
                          start: 27,
                          end: 55,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 57,
                      ),
                      exp: "\"You are a helpful assistant.\"",
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

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @r#"
                [[]]
                enclosure = '''
                // @prompt
                const system = "You are a helpful assistant.";'''
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
fn assigned() {
    ParseTest::test(
        None,
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            let assigned;
            assigned = `Assigned ${value}`;
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
                          start: 36,
                          end: 55,
                        ),
                        inner: Span(
                          start: 37,
                          end: 54,
                        ),
                      ),
                      enclosure: Span(
                        start: 25,
                        end: 56,
                      ),
                      exp: "`Assigned ${value}`",
                      vars: [
                        PromptVar(
                          exp: "${value}",
                          span: SpanShape(
                            outer: Span(
                              start: 46,
                              end: 54,
                            ),
                            inner: Span(
                              start: 48,
                              end: 53,
                            ),
                          ),
                        ),
                      ],
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

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @r"
                [[]]
                enclosure = 'assigned = `Assigned ${value}`;'
                outer = '`Assigned ${value}`'
                inner = 'Assigned ${value}'

                [[vars]]
                outer = '${value}'
                inner = 'value'
                ");
            }),

            interpolate: Some(Box::new(|_interpolations| {
                assert_snapshot!(true, @"true");
            })),
        },
    );
}

#[test]
fn reassigned() {
    ParseTest::test(
        None,
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            let reassigned = 123;
            reassigned = 456;
            reassigned = `Reassigned ${value}`;
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
                          start: 64,
                          end: 85,
                        ),
                        inner: Span(
                          start: 65,
                          end: 84,
                        ),
                      ),
                      enclosure: Span(
                        start: 51,
                        end: 86,
                      ),
                      exp: "`Reassigned ${value}`",
                      vars: [
                        PromptVar(
                          exp: "${value}",
                          span: SpanShape(
                            outer: Span(
                              start: 76,
                              end: 84,
                            ),
                            inner: Span(
                              start: 78,
                              end: 83,
                            ),
                          ),
                        ),
                      ],
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

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @r"
                [[]]
                enclosure = 'reassigned = `Reassigned ${value}`;'
                outer = '`Reassigned ${value}`'
                inner = 'Reassigned ${value}'

                [[vars]]
                outer = '${value}'
                inner = 'value'
                ");
            }),

            interpolate: Some(Box::new(|_interpolations| {
                assert_snapshot!(true, @"true");
            })),
        },
    );
}

#[test]
fn not_exact() {
    ParseTest::test(
        None,
        &ParseTestLang::ts(indoc! {r#"
            // @prompting
            const hello = "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @"[]");
            }),

            interpolate: None,
        },
    );
}

#[test]
fn mixed() {
    ParseTest::test(
        None,
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const number = 42;
            const hello = "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @"[]");
            }),

            interpolate: None,
        },
    );
}

#[test]
fn mixed_nested() {
    ParseTest::test(
        None,
        &ParseTestLang::ts(indoc! {r#"
            class Hello {
                world(self) {
                    // @prompt
                    let hello = 42;

                    // @prompt
                    let hi = 42;

                    hi = "Hi!"
                }
            }

            hello = "Hello, world!";
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
                          start: 130,
                          end: 135,
                        ),
                        inner: Span(
                          start: 131,
                          end: 134,
                        ),
                      ),
                      enclosure: Span(
                        start: 125,
                        end: 135,
                      ),
                      exp: "\"Hi!\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 84,
                            end: 94,
                          ),
                          exp: "// @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @r#"
                [[]]
                enclosure = 'hi = "Hi!"'
                outer = '"Hi!"'
                inner = 'Hi!'
                vars = []
                "#);
            }),

            interpolate: Some(Box::new(|interpolated| {
                assert_toml_snapshot!(interpolated, @"['Hi!']");
            })),
        },
    );
}

#[test]
fn mixed_none() {
    ParseTest::test(
        None,
        &ParseTestLang::ts(indoc! {r#"
            const regularTemplate = `This is not a ${value}`;
            const normalString = "This is not special";
            const regular = `Regular template with ${variable}`;
            const message = "Just a message";
            // @prompt
            const number = 1;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @"[]");
            }),

            interpolate: None,
        },
    );
}
