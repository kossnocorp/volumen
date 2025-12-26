use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[ignore]
#[test]
fn multiple() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            String userPrompt = "Hello, name!";
            String greeting = /* @prompt */ "Welcome!";
            // @prompt
            String farewell = "Goodbye!";
            /** @prompt */
            String system = "You are an AI assistant";
            String regular = "Not a prompt";
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
                          end: 34,
                        ),
                        inner: Span(
                          start: 21,
                          end: 33,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 35,
                      ),
                      exp: "\"Hello, name!\"",
                      vars: [],
                      annotations: [],
                    ),
                    Prompt(
                      file: "Prompts.java",
                      span: SpanShape(
                        outer: Span(
                          start: 68,
                          end: 78,
                        ),
                        inner: Span(
                          start: 69,
                          end: 77,
                        ),
                      ),
                      enclosure: Span(
                        start: 36,
                        end: 79,
                      ),
                      exp: "\"Welcome!\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 54,
                            end: 67,
                          ),
                          exp: "/* @prompt */",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "Prompts.java",
                      span: SpanShape(
                        outer: Span(
                          start: 109,
                          end: 119,
                        ),
                        inner: Span(
                          start: 110,
                          end: 118,
                        ),
                      ),
                      enclosure: Span(
                        start: 80,
                        end: 120,
                      ),
                      exp: "\"Goodbye!\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 80,
                            end: 90,
                          ),
                          exp: "// @prompt",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "Prompts.java",
                      span: SpanShape(
                        outer: Span(
                          start: 152,
                          end: 177,
                        ),
                        inner: Span(
                          start: 153,
                          end: 176,
                        ),
                      ),
                      enclosure: Span(
                        start: 121,
                        end: 178,
                      ),
                      exp: "\"You are an AI assistant\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 121,
                            end: 135,
                          ),
                          exp: "/** @prompt */",
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
                    "enclosure": "String userPrompt = \"Hello, name!\";",
                    "outer": "\"Hello, name!\"",
                    "inner": "Hello, name!",
                    "vars": []
                  },
                  {
                    "enclosure": "String greeting = /* @prompt */ \"Welcome!\";",
                    "outer": "\"Welcome!\"",
                    "inner": "Welcome!",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\nString farewell = \"Goodbye!\";",
                    "outer": "\"Goodbye!\"",
                    "inner": "Goodbye!",
                    "vars": []
                  },
                  {
                    "enclosure": "/** @prompt */\nString system = \"You are an AI assistant\";",
                    "outer": "\"You are an AI assistant\"",
                    "inner": "You are an AI assistant",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello, name!",
                  "Welcome!",
                  "Goodbye!",
                  "You are an AI assistant"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
                [
                  [],
                  [
                    "/* @prompt */"
                  ],
                  [
                    "// @prompt"
                  ],
                  [
                    "/** @prompt */"
                  ]
                ]
                "#);
            }),
        },
    );
}
