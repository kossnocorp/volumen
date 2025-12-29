use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn multiple() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            const userPrompt = `Hello, ${name}!`;
            const greeting = /* @prompt */ `Welcome ${user}!`;
            // @prompt
            const farewell = `Goodbye ${user.name}!`;
            /** @prompt */
            const system = "You are an AI assistant";
            const regular = `Not a prompt ${value}`;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 37),
                      span: SpanShape(
                        outer: (19, 36),
                        inner: (20, 35),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (20, 27),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (27, 34),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (34, 35),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (27, 34),
                            inner: (29, 33),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                    Prompt(
                      file: "prompts.js",
                      enclosure: (38, 88),
                      span: SpanShape(
                        outer: (69, 87),
                        inner: (70, 86),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (70, 78),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (78, 85),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (85, 86),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (78, 85),
                            inner: (80, 84),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (55, 68),
                              inner: (57, 66),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      enclosure: (89, 141),
                      span: SpanShape(
                        outer: (117, 140),
                        inner: (118, 139),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (118, 126),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (126, 138),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (138, 139),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (126, 138),
                            inner: (128, 137),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (89, 99),
                              inner: (91, 99),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      enclosure: (142, 198),
                      span: SpanShape(
                        outer: (172, 197),
                        inner: (173, 196),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (173, 196),
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
                              outer: (142, 156),
                              inner: (145, 154),
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
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "const userPrompt = `Hello, ${name}!`;",
                    "outer": "`Hello, ${name}!`",
                    "inner": "Hello, ${name}!",
                    "vars": [
                      {
                        "outer": "${name}",
                        "inner": "name"
                      }
                    ]
                  },
                  {
                    "enclosure": "const greeting = /* @prompt */ `Welcome ${user}!`;",
                    "outer": "`Welcome ${user}!`",
                    "inner": "Welcome ${user}!",
                    "vars": [
                      {
                        "outer": "${user}",
                        "inner": "user"
                      }
                    ]
                  },
                  {
                    "enclosure": "// @prompt\nconst farewell = `Goodbye ${user.name}!`;",
                    "outer": "`Goodbye ${user.name}!`",
                    "inner": "Goodbye ${user.name}!",
                    "vars": [
                      {
                        "outer": "${user.name}",
                        "inner": "user.name"
                      }
                    ]
                  },
                  {
                    "enclosure": "/** @prompt */\nconst system = \"You are an AI assistant\";",
                    "outer": "\"You are an AI assistant\"",
                    "inner": "You are an AI assistant",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, {0}!",
                  "Welcome {0}!",
                  "Goodbye {0}!",
                  "You are an AI assistant"
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
