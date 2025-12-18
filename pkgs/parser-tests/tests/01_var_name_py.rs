use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
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
        &ParseTestLang::py(indoc! {r#"
            class Hello:
                def world(self):
                    def fn():
                        hello_prompt = f"Hello, {name}!"

                        # @prompt
                        also_prmpt = "Hi!"
                    return fn
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 79,
                          end: 96,
                        ),
                        inner: Span(
                          start: 81,
                          end: 95,
                        ),
                      ),
                      enclosure: Span(
                        start: 64,
                        end: 96,
                      ),
                      exp: "f\"Hello, {name}!\"",
                      vars: [
                        PromptVar(
                          exp: "{name}",
                          span: SpanShape(
                            outer: Span(
                              start: 88,
                              end: 94,
                            ),
                            inner: Span(
                              start: 89,
                              end: 93,
                            ),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 145,
                          end: 150,
                        ),
                        inner: Span(
                          start: 146,
                          end: 149,
                        ),
                      ),
                      enclosure: Span(
                        start: 110,
                        end: 150,
                      ),
                      exp: "\"Hi!\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 110,
                            end: 119,
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
                    "enclosure": "hello_prompt = f\"Hello, {name}!\"",
                    "outer": "f\"Hello, {name}!\"",
                    "inner": "Hello, {name}!",
                    "vars": [
                      {
                        "outer": "{name}",
                        "inner": "name"
                      }
                    ]
                  },
                  {
                    "enclosure": "# @prompt\n            also_prmpt = \"Hi!\"",
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
