use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn multiple() {
    ParseTest::test(
        &ParseTestLang::go(indoc! {r#"
            userPrompt := "Hello, name!"
            greeting := /* @prompt */ "Welcome!"
            // @prompt
            farewell := "Goodbye!"
            /** @prompt */
            system := "You are an AI assistant"
            regular := "Not a prompt"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.go",
                      enclosure: (0, 28),
                      span: SpanShape(
                        outer: (14, 28),
                        inner: (15, 27),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (15, 27),
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
                      file: "prompts.go",
                      enclosure: (29, 65),
                      span: SpanShape(
                        outer: (55, 65),
                        inner: (56, 64),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (56, 64),
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
                              outer: (41, 54),
                              inner: (43, 52),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.go",
                      enclosure: (66, 99),
                      span: SpanShape(
                        outer: (89, 99),
                        inner: (90, 98),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (90, 98),
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
                              outer: (66, 76),
                              inner: (68, 76),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.go",
                      enclosure: (100, 150),
                      span: SpanShape(
                        outer: (125, 150),
                        inner: (126, 149),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (126, 149),
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
                              outer: (100, 114),
                              inner: (103, 112),
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
                    "enclosure": "userPrompt := \"Hello, name!\"",
                    "outer": "\"Hello, name!\"",
                    "inner": "Hello, name!",
                    "vars": []
                  },
                  {
                    "enclosure": "greeting := /* @prompt */ \"Welcome!\"",
                    "outer": "\"Welcome!\"",
                    "inner": "Welcome!",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\nfarewell := \"Goodbye!\"",
                    "outer": "\"Goodbye!\"",
                    "inner": "Goodbye!",
                    "vars": []
                  },
                  {
                    "enclosure": "/** @prompt */\nsystem := \"You are an AI assistant\"",
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
