use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            $greeting = /* @prompt */ "Welcome, {$user}!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      span: SpanShape(
                        outer: Span(
                          start: 32,
                          end: 51,
                        ),
                        inner: Span(
                          start: 33,
                          end: 50,
                        ),
                      ),
                      enclosure: Span(
                        start: 6,
                        end: 51,
                      ),
                      exp: "\"Welcome, {$user}!\"",
                      vars: [
                        PromptVar(
                          exp: "{$user}",
                          span: SpanShape(
                            outer: Span(
                              start: 42,
                              end: 49,
                            ),
                            inner: Span(
                              start: 43,
                              end: 48,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 18,
                            end: 31,
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
                    "enclosure": "$greeting = /* @prompt */ \"Welcome, {$user}!\"",
                    "outer": "\"Welcome, {$user}!\"",
                    "inner": "Welcome, {$user}!",
                    "vars": [
                      {
                        "outer": "{$user}",
                        "inner": "$user"
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
fn phpdoc() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            $hello = /** @prompt */ "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      span: SpanShape(
                        outer: Span(
                          start: 30,
                          end: 45,
                        ),
                        inner: Span(
                          start: 31,
                          end: 44,
                        ),
                      ),
                      enclosure: Span(
                        start: 6,
                        end: 45,
                      ),
                      exp: "\"Hello, world!\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 15,
                            end: 29,
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
                    "enclosure": "$hello = /** @prompt */ \"Hello, world!\"",
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
        &ParseTestLang::php(indoc! {r#"
            <?php
            $greeting = /* @prompting */ "Welcome, {$user}!";
            $whatever = /* wrong@prompt */ "That's not it!";
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
        &ParseTestLang::php(indoc! {r#"
            <?php
            $greeting = /* @prompt greeting */ "Welcome, {$user}!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      span: SpanShape(
                        outer: Span(
                          start: 41,
                          end: 60,
                        ),
                        inner: Span(
                          start: 42,
                          end: 59,
                        ),
                      ),
                      enclosure: Span(
                        start: 6,
                        end: 60,
                      ),
                      exp: "\"Welcome, {$user}!\"",
                      vars: [
                        PromptVar(
                          exp: "{$user}",
                          span: SpanShape(
                            outer: Span(
                              start: 51,
                              end: 58,
                            ),
                            inner: Span(
                              start: 52,
                              end: 57,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 18,
                            end: 40,
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
                    "enclosure": "$greeting = /* @prompt greeting */ \"Welcome, {$user}!\"",
                    "outer": "\"Welcome, {$user}!\"",
                    "inner": "Welcome, {$user}!",
                    "vars": [
                      {
                        "outer": "{$user}",
                        "inner": "$user"
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
