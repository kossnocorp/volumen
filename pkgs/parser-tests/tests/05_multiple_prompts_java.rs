use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

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
                      enclosure: (0, 35),
                      span: SpanShape(
                        outer: (20, 34),
                        inner: (21, 33),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (21, 33),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [],
                      annotations: [],
                    ),
                    Prompt(
                      file: "Prompts.java",
                      enclosure: (36, 79),
                      span: SpanShape(
                        outer: (68, 78),
                        inner: (69, 77),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (69, 77),
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
                              outer: (54, 67),
                              inner: (56, 65),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "Prompts.java",
                      enclosure: (80, 120),
                      span: SpanShape(
                        outer: (109, 119),
                        inner: (110, 118),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (110, 118),
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
                              outer: (80, 90),
                              inner: (82, 90),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "Prompts.java",
                      enclosure: (121, 178),
                      span: SpanShape(
                        outer: (152, 177),
                        inner: (153, 176),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (153, 176),
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
                              outer: (121, 135),
                              inner: (124, 133),
                            ),
                          ],
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
                    [
                      {
                        "outer": "/* @prompt */",
                        "inner": " @prompt "
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "// @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "/** @prompt */",
                        "inner": " @prompt "
                      }
                    ]
                  ]
                ]
                "#);
            }),
        },
    );
}
