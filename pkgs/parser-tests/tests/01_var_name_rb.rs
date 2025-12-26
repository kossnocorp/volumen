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
                      enclosure: (0, 44),
                      span: SpanShape(
                        outer: (14, 44),
                        inner: (15, 43),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (15, 43),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [],
                      annotations: [],
                      exp: "\"You are a helpful assistant.\"",
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
                      enclosure: (41, 73),
                      span: SpanShape(
                        outer: (56, 73),
                        inner: (57, 72),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (57, 64),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (64, 71),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (71, 72),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (64, 71),
                            inner: (66, 70),
                          ),
                          exp: "#{name}",
                        ),
                      ],
                      annotations: [],
                      exp: "\"Hello, #{name}!\"",
                    ),
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (81, 116),
                      span: SpanShape(
                        outer: (111, 116),
                        inner: (112, 115),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (112, 115),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (81, 90),
                              inner: (82, 90),
                            ),
                          ],
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"Hi!\"",
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
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ]
                ]
                "##);
            }),
        },
    );
}
