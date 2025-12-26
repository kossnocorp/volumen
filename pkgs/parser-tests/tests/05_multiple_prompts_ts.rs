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
                      span: SpanShape(
                        outer: (19, 36),
                        inner: (20, 35),
                      ),
                      enclosure: (0, 37),
                      exp: "`Hello, ${name}!`",
                      vars: [
                        PromptVar(
                          exp: "${name}",
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
                      span: SpanShape(
                        outer: (69, 87),
                        inner: (70, 86),
                      ),
                      enclosure: (38, 88),
                      exp: "`Welcome ${user}!`",
                      vars: [
                        PromptVar(
                          exp: "${user}",
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
                          exp: "/* @prompt */",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: (117, 140),
                        inner: (118, 139),
                      ),
                      enclosure: (89, 141),
                      exp: "`Goodbye ${user.name}!`",
                      vars: [
                        PromptVar(
                          exp: "${user.name}",
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
                          exp: "// @prompt",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: (172, 197),
                        inner: (173, 196),
                      ),
                      enclosure: (142, 198),
                      exp: "\"You are an AI assistant\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (142, 156),
                              inner: (145, 154),
                            ),
                          ],
                          exp: "/** @prompt */",
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
