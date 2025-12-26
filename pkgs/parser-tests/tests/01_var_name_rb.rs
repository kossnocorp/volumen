use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            user_prompt = "You are a helpful assistant."
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
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
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "user_prompt = \"You are a helpful assistant.\"",
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
                assert_json_snapshot!(annotations, @"
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
        &ParseTestLang::rb(indoc! {r#"
            class Hello
              def world
                def fn
                  hello_prompt = "Hello, #{name}!"

                  # @prompt
                  also_prompt = "Hi!"
                end
                return fn
              end
            end
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: Span(
                          start: 56,
                          end: 73,
                        ),
                        inner: Span(
                          start: 57,
                          end: 72,
                        ),
                      ),
                      enclosure: Span(
                        start: 41,
                        end: 73,
                      ),
                      exp: "\"Hello, #{name}!\"",
                      vars: [
                        PromptVar(
                          exp: "#{name}",
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
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: Span(
                          start: 111,
                          end: 116,
                        ),
                        inner: Span(
                          start: 112,
                          end: 115,
                        ),
                      ),
                      enclosure: Span(
                        start: 81,
                        end: 116,
                      ),
                      exp: "\"Hi!\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 81,
                            end: 90,
                          ),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "hello_prompt = \"Hello, #{name}!\"",
                    "outer": "\"Hello, #{name}!\"",
                    "inner": "Hello, #{name}!",
                    "vars": [
                      {
                        "outer": "#{name}",
                        "inner": "name"
                      }
                    ]
                  },
                  {
                    "enclosure": "# @prompt\n      also_prompt = \"Hi!\"",
                    "outer": "\"Hi!\"",
                    "inner": "Hi!",
                    "vars": []
                  }
                ]
                "##);
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
                assert_json_snapshot!(annotations, @r##"
                [
                  [],
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}
