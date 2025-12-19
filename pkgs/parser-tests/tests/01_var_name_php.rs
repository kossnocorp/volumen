use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            $user_prompt = "You are a helpful assistant.";
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
                          start: 21,
                          end: 51,
                        ),
                        inner: Span(
                          start: 22,
                          end: 50,
                        ),
                      ),
                      enclosure: Span(
                        start: 6,
                        end: 51,
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
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "$user_prompt = \"You are a helpful assistant.\"",
                    "outer": "\"You are a helpful assistant.\"",
                    "inner": "You are a helpful assistant.",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "You are a helpful assistant."
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r"
                [
                  []
                ]
                ");
            }),
        },
    );
}

#[test]
fn nested() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            class Hello {
              function world() {
                $hello_prompt = "Hello, {$name}!";

                // @prompt
                $also_prompt = "Hi!";
              }
            }
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
                          start: 61,
                          end: 78,
                        ),
                        inner: Span(
                          start: 62,
                          end: 77,
                        ),
                      ),
                      enclosure: Span(
                        start: 45,
                        end: 78,
                      ),
                      exp: "\"Hello, {$name}!\"",
                      vars: [
                        PromptVar(
                          exp: "{$name}",
                          span: SpanShape(
                            outer: Span(
                              start: 69,
                              end: 76,
                            ),
                            inner: Span(
                              start: 70,
                              end: 75,
                            ),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                    Prompt(
                      file: "prompts.php",
                      span: SpanShape(
                        outer: Span(
                          start: 115,
                          end: 120,
                        ),
                        inner: Span(
                          start: 116,
                          end: 119,
                        ),
                      ),
                      enclosure: Span(
                        start: 85,
                        end: 120,
                      ),
                      exp: "\"Hi!\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 85,
                            end: 95,
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
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "$hello_prompt = \"Hello, {$name}!\"",
                    "outer": "\"Hello, {$name}!\"",
                    "inner": "Hello, {$name}!",
                    "vars": [
                      {
                        "outer": "{$name}",
                        "inner": "$name"
                      }
                    ]
                  },
                  {
                    "enclosure": "// @prompt\n    $also_prompt = \"Hi!\"",
                    "outer": "\"Hi!\"",
                    "inner": "Hi!",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, {0}!",
                  "Hi!"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [],
                  [
                    "// @prompt"
                  ]
                ]
                "#);
            }),
        },
    );
}
