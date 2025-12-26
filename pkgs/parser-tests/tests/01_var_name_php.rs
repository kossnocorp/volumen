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
                      enclosure: (6, 51),
                      span: SpanShape(
                        outer: (21, 51),
                        inner: (22, 50),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (22, 50),
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
                      enclosure: (45, 78),
                      span: SpanShape(
                        outer: (61, 78),
                        inner: (62, 77),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (62, 77),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (69, 76),
                            inner: (70, 75),
                          ),
                          exp: "{$name}",
                        ),
                      ],
                      annotations: [],
                      exp: "\"Hello, {$name}!\"",
                    ),
                    Prompt(
                      file: "prompts.php",
                      enclosure: (85, 120),
                      span: SpanShape(
                        outer: (115, 120),
                        inner: (116, 119),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (116, 119),
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
                              outer: (85, 95),
                              inner: (87, 95),
                            ),
                          ],
                          exp: "// @prompt",
                        ),
                      ],
                      exp: "\"Hi!\"",
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
                    [
                      {
                        "outer": "// @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ]
                ]
                "#);
            }),
        },
    );
}
