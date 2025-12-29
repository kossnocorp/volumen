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
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (64, 96),
                      span: SpanShape(
                        outer: (79, 96),
                        inner: (81, 95),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (81, 88),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (88, 94),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (94, 95),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (88, 94),
                            inner: (89, 93),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (110, 150),
                      span: SpanShape(
                        outer: (145, 150),
                        inner: (146, 149),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (146, 149),
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
                              outer: (110, 119),
                              inner: (111, 119),
                            ),
                          ],
                        ),
                      ],
                    ),
                  ],
                )
                "#);
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
