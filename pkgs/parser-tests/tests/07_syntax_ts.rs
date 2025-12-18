use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn invalid() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            const invalid = `unclosed template
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
fn jsx() {
    ParseTest::test(
        &ParseTestLang::ts_named(
            indoc! {r#"
                const prompt = /* @prompt */ `Hello, ${world}!`;
                const element = <div>{prompt}</div>;
            "#},
            "prompts.tsx",
        ),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.tsx",
                      span: SpanShape(
                        outer: Span(
                          start: 29,
                          end: 47,
                        ),
                        inner: Span(
                          start: 30,
                          end: 46,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 48,
                      ),
                      exp: "`Hello, ${world}!`",
                      vars: [
                        PromptVar(
                          exp: "${world}",
                          span: SpanShape(
                            outer: Span(
                              start: 37,
                              end: 45,
                            ),
                            inner: Span(
                              start: 39,
                              end: 44,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 15,
                            end: 28,
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
                    "enclosure": "const prompt = /* @prompt */ `Hello, ${world}!`;",
                    "outer": "`Hello, ${world}!`",
                    "inner": "Hello, ${world}!",
                    "vars": [
                      {
                        "outer": "${world}",
                        "inner": "world"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, {0}!"
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
fn ts() {
    ParseTest::test(
        &ParseTestLang::ts_named(
            indoc! {r#"
              const prompt: string = /* @prompt */ `Hello ${world}!`;
            "#},
            "prompts.ts",
        ),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.ts",
                      span: SpanShape(
                        outer: Span(
                          start: 37,
                          end: 54,
                        ),
                        inner: Span(
                          start: 38,
                          end: 53,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 55,
                      ),
                      exp: "`Hello ${world}!`",
                      vars: [
                        PromptVar(
                          exp: "${world}",
                          span: SpanShape(
                            outer: Span(
                              start: 44,
                              end: 52,
                            ),
                            inner: Span(
                              start: 46,
                              end: 51,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 23,
                            end: 36,
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
                    "enclosure": "const prompt: string = /* @prompt */ `Hello ${world}!`;",
                    "outer": "`Hello ${world}!`",
                    "inner": "Hello ${world}!",
                    "vars": [
                      {
                        "outer": "${world}",
                        "inner": "world"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello {0}!"
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
fn tsx() {
    ParseTest::test(
        &ParseTestLang::ts_named(
            indoc! {r#"
              const prompt: string = /* @prompt */ `Hello ${world}!`;
              const element = <div>{prompt}</div>;
            "#},
            "prompts.tsx",
        ),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.tsx",
                      span: SpanShape(
                        outer: Span(
                          start: 37,
                          end: 54,
                        ),
                        inner: Span(
                          start: 38,
                          end: 53,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 55,
                      ),
                      exp: "`Hello ${world}!`",
                      vars: [
                        PromptVar(
                          exp: "${world}",
                          span: SpanShape(
                            outer: Span(
                              start: 44,
                              end: 52,
                            ),
                            inner: Span(
                              start: 46,
                              end: 51,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 23,
                            end: 36,
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
                    "enclosure": "const prompt: string = /* @prompt */ `Hello ${world}!`;",
                    "outer": "`Hello ${world}!`",
                    "inner": "Hello ${world}!",
                    "vars": [
                      {
                        "outer": "${world}",
                        "inner": "world"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello {0}!"
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
