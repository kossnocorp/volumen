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
                      span: SpanShape(
                        outer: Span(
                          start: 20,
                          end: 50,
                        ),
                        inner: Span(
                          start: 21,
                          end: 49,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
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
                      span: SpanShape(
                        outer: Span(
                          start: 62,
                          end: 77,
                        ),
                        inner: Span(
                          start: 63,
                          end: 76,
                        ),
                      ),
                      enclosure: Span(
                        start: 41,
                        end: 78,
                      ),
                      exp: "\"Hello, world!\"",
                      vars: [],
                      annotations: [],
                    ),
                    Prompt(
                      file: "Prompts.java",
                      span: SpanShape(
                        outer: Span(
                          start: 127,
                          end: 132,
                        ),
                        inner: Span(
                          start: 128,
                          end: 131,
                        ),
                      ),
                      enclosure: Span(
                        start: 88,
                        end: 133,
                      ),
                      exp: "\"Hi!\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 88,
                            end: 98,
                          ),
                          exp: "// @prompt",
                        ),
                      ],
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
                    "// @prompt"
                  ]
                ]
                "#);
            }),
        },
    );
}
