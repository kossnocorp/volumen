use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            String userPrompt = "You are a helpful assistant.";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
                      enclosure: (0, 51),
                      span: SpanShape(
                        outer: (20, 50),
                        inner: (21, 49),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (21, 49),
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
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "String userPrompt = \"You are a helpful assistant.\";",
                    "outer": "\"You are a helpful assistant.\"",
                    "inner": "You are a helpful assistant.",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "You are a helpful assistant."
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"
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
        &ParseTestLang::java(indoc! {r#"
            class Hello {
                void world() {
                    String helloPrompt = "Hello, world!";

                    // @prompt
                    String alsoPrompt = "Hi!";
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
                      file: "Prompts.java",
                      enclosure: (41, 78),
                      span: SpanShape(
                        outer: (62, 77),
                        inner: (63, 76),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (63, 76),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [],
                      annotations: [],
                      exp: "\"Hello, world!\"",
                    ),
                    Prompt(
                      file: "Prompts.java",
                      enclosure: (88, 133),
                      span: SpanShape(
                        outer: (127, 132),
                        inner: (128, 131),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (128, 131),
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
                              outer: (88, 98),
                              inner: (90, 98),
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
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "String helloPrompt = \"Hello, world!\";",
                    "outer": "\"Hello, world!\"",
                    "inner": "Hello, world!",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\n        String alsoPrompt = \"Hi!\";",
                    "outer": "\"Hi!\"",
                    "inner": "Hi!",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello, world!",
                  "Hi!"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
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
