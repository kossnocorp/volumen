use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
          const greeting = /* @prompt */ `Welcome, ${user}!`
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
                          start: 31,
                          end: 50,
                        ),
                        inner: Span(
                          start: 32,
                          end: 49,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 50,
                      ),
                      exp: "`Welcome, ${user}!`",
                      vars: [
                        PromptVar(
                          exp: "${user}",
                          span: SpanShape(
                            outer: Span(
                              start: 41,
                              end: 48,
                            ),
                            inner: Span(
                              start: 43,
                              end: 47,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 17,
                            end: 30,
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
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "const greeting = /* @prompt */ `Welcome, ${user}!`",
                    "outer": "`Welcome, ${user}!`",
                    "inner": "Welcome, ${user}!",
                    "vars": [
                      {
                        "outer": "${user}",
                        "inner": "user"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Welcome, {0}!"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    "/* @prompt */"
                  ]
                ]
                "#);
            }),
        },
    );
}

#[test]
fn jsdoc() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
          const hello = /** @prompt */ "Hello, world!";
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
                          start: 29,
                          end: 44,
                        ),
                        inner: Span(
                          start: 30,
                          end: 43,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 45,
                      ),
                      exp: "\"Hello, world!\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 14,
                            end: 28,
                          ),
                          exp: "/** @prompt */",
                        ),
                      ],
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "const hello = /** @prompt */ \"Hello, world!\";",
                    "outer": "\"Hello, world!\"",
                    "inner": "Hello, world!",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, world!"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    "/** @prompt */"
                  ]
                ]
                "#);
            }),
        },
    );
}

#[test]
fn inexact() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
          const greeting = /* @prompting */ `Welcome, ${user}!`;
          const whatever = /* wrong@prompt */ "That's not it!";
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
                assert_json_snapshot!(prompt_source_cuts, @"[]");
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @"[]");
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @"[]");
            }),
        },
    );
}

#[test]
fn dirty() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
          const greeting = /* @prompt greeting */ `Welcome, ${user}!`;
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
                          start: 40,
                          end: 59,
                        ),
                        inner: Span(
                          start: 41,
                          end: 58,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 60,
                      ),
                      exp: "`Welcome, ${user}!`",
                      vars: [
                        PromptVar(
                          exp: "${user}",
                          span: SpanShape(
                            outer: Span(
                              start: 50,
                              end: 57,
                            ),
                            inner: Span(
                              start: 52,
                              end: 56,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 17,
                            end: 39,
                          ),
                          exp: "/* @prompt greeting */",
                        ),
                      ],
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "const greeting = /* @prompt greeting */ `Welcome, ${user}!`;",
                    "outer": "`Welcome, ${user}!`",
                    "inner": "Welcome, ${user}!",
                    "vars": [
                      {
                        "outer": "${user}",
                        "inner": "user"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Welcome, {0}!"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    "/* @prompt greeting */"
                  ]
                ]
                "#);
            }),
        },
    );
}
