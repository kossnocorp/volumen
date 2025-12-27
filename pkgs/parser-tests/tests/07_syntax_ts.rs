use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn invalid() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            const invalid = `unclosed template
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultError(
                  state: "error",
                  error: "<error>",
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @"[]");
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @"[]");
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @"[]");
            }),
        },
    );
}

#[test]
fn jsx() {
    ParseTest::test(
        &ParseTestLang::ts_named(
            indoc! {r#"
                const prompt = /* @prompt */ `Hello, ${world}!`;
                const element = <div>{prompt}</div>;
            "#},
            "prompts.tsx",
        ),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.tsx",
                      enclosure: (0, 48),
                      span: SpanShape(
                        outer: (29, 47),
                        inner: (30, 46),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (30, 37),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (37, 45),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (45, 46),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (37, 45),
                            inner: (39, 44),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (15, 28),
                              inner: (17, 26),
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
                    "enclosure": "const prompt = /* @prompt */ `Hello, ${world}!`;",
                    "outer": "`Hello, ${world}!`",
                    "inner": "Hello, ${world}!",
                    "vars": [
                      {
                        "outer": "${world}",
                        "inner": "world"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, {0}!"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    [
                      {
                        "outer": "/* @prompt */",
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

#[test]
fn ts() {
    ParseTest::test(
        &ParseTestLang::ts_named(
            indoc! {r#"
              const prompt: string = /* @prompt */ `Hello ${world}!`;
            "#},
            "prompts.ts",
        ),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.ts",
                      enclosure: (0, 55),
                      span: SpanShape(
                        outer: (37, 54),
                        inner: (38, 53),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (38, 44),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (44, 52),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (52, 53),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (44, 52),
                            inner: (46, 51),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (23, 36),
                              inner: (25, 34),
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
                    "enclosure": "const prompt: string = /* @prompt */ `Hello ${world}!`;",
                    "outer": "`Hello ${world}!`",
                    "inner": "Hello ${world}!",
                    "vars": [
                      {
                        "outer": "${world}",
                        "inner": "world"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello {0}!"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    [
                      {
                        "outer": "/* @prompt */",
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

#[test]
fn tsx() {
    ParseTest::test(
        &ParseTestLang::ts_named(
            indoc! {r#"
              const prompt: string = /* @prompt */ `Hello ${world}!`;
              const element = <div>{prompt}</div>;
            "#},
            "prompts.tsx",
        ),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.tsx",
                      enclosure: (0, 55),
                      span: SpanShape(
                        outer: (37, 54),
                        inner: (38, 53),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (38, 44),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (44, 52),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (52, 53),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (44, 52),
                            inner: (46, 51),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (23, 36),
                              inner: (25, 34),
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
                    "enclosure": "const prompt: string = /* @prompt */ `Hello ${world}!`;",
                    "outer": "`Hello ${world}!`",
                    "inner": "Hello ${world}!",
                    "vars": [
                      {
                        "outer": "${world}",
                        "inner": "world"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello {0}!"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    [
                      {
                        "outer": "/* @prompt */",
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

#[test]
fn multiline() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const user = `Hello, ${name}!
            How is the weather today in ${city}?
            `;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 80),
                      span: SpanShape(
                        outer: (24, 79),
                        inner: (25, 78),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (25, 32),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (32, 39),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (39, 69),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (69, 76),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (76, 78),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (32, 39),
                            inner: (34, 38),
                          ),
                        ),
                        PromptVar(
                          span: SpanShape(
                            outer: (69, 76),
                            inner: (71, 75),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (0, 10),
                              inner: (2, 10),
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
                    "enclosure": "// @prompt\nconst user = `Hello, ${name}!\nHow is the weather today in ${city}?\n`;",
                    "outer": "`Hello, ${name}!\nHow is the weather today in ${city}?\n`",
                    "inner": "Hello, ${name}!\nHow is the weather today in ${city}?\n",
                    "vars": [
                      {
                        "outer": "${name}",
                        "inner": "name"
                      },
                      {
                        "outer": "${city}",
                        "inner": "city"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, {0}!\nHow is the weather today in {1}?\n"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
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
