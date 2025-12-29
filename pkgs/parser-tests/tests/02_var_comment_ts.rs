use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const system = "You are a helpful assistant.";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 57),
                      span: SpanShape(
                        outer: (26, 56),
                        inner: (27, 55),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (27, 55),
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
                    "enclosure": "// @prompt\nconst system = \"You are a helpful assistant.\";",
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

#[test]
fn inline() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            /* @prompt */
            const hello = `Hello, world!`;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 44),
                      span: SpanShape(
                        outer: (28, 43),
                        inner: (29, 42),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (29, 42),
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
                              outer: (0, 13),
                              inner: (2, 11),
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
                    "enclosure": "/* @prompt */\nconst hello = `Hello, world!`;",
                    "outer": "`Hello, world!`",
                    "inner": "Hello, world!",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, world!"
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
fn doc() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            /**
             * @prompt
             */
            const hello = `Hello, world!`;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 49),
                      span: SpanShape(
                        outer: (33, 48),
                        inner: (34, 47),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (34, 47),
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
                              outer: (0, 18),
                              inner: (3, 16),
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
                    "enclosure": "/**\n * @prompt\n */\nconst hello = `Hello, world!`;",
                    "outer": "`Hello, world!`",
                    "inner": "Hello, world!",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, world!"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    [
                      {
                        "outer": "/**\n * @prompt\n */",
                        "inner": "\n * @prompt\n "
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
fn assigned() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            let assigned;
            assigned = `Assigned ${value}`;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (25, 56),
                      span: SpanShape(
                        outer: (36, 55),
                        inner: (37, 54),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (37, 46),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (46, 54),
                          index: 0,
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (46, 54),
                            inner: (48, 53),
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
                    "enclosure": "assigned = `Assigned ${value}`;",
                    "outer": "`Assigned ${value}`",
                    "inner": "Assigned ${value}",
                    "vars": [
                      {
                        "outer": "${value}",
                        "inner": "value"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Assigned {0}"
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

#[test]
fn assigned_late_comment() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            let assigned;
            // @prompt
            assigned = `Assigned ${value}`;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (14, 56),
                      span: SpanShape(
                        outer: (36, 55),
                        inner: (37, 54),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (37, 46),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (46, 54),
                          index: 0,
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (46, 54),
                            inner: (48, 53),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (14, 24),
                              inner: (16, 24),
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
                    "enclosure": "// @prompt\nassigned = `Assigned ${value}`;",
                    "outer": "`Assigned ${value}`",
                    "inner": "Assigned ${value}",
                    "vars": [
                      {
                        "outer": "${value}",
                        "inner": "value"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Assigned {0}"
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

#[test]
fn reassigned() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            let reassigned = 123;
            reassigned = 456;
            reassigned = `Reassigned ${value}`;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (51, 86),
                      span: SpanShape(
                        outer: (64, 85),
                        inner: (65, 84),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (65, 76),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (76, 84),
                          index: 0,
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (76, 84),
                            inner: (78, 83),
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
                    "enclosure": "reassigned = `Reassigned ${value}`;",
                    "outer": "`Reassigned ${value}`",
                    "inner": "Reassigned ${value}",
                    "vars": [
                      {
                        "outer": "${value}",
                        "inner": "value"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Reassigned {0}"
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

#[test]
fn inexact() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompting
            const hello = "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [],
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
fn mixed() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const number = 42;
            const hello = "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [],
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
fn mixed_nested() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            class Hello {
                world(self) {
                    // @prompt
                    let hello = 42;

                    // @prompt
                    let hi = 42;

                    hi = "Hi!"
                }
            }

            hello = "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (125, 135),
                      span: SpanShape(
                        outer: (130, 135),
                        inner: (131, 134),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (131, 134),
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
                              outer: (84, 94),
                              inner: (86, 94),
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
                    "enclosure": "hi = \"Hi!\"",
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
                  "Hi!"
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

#[test]
fn mixed_none() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            const regularTemplate = `This is not a ${value}`;
            const normalString = "This is not special";
            const regular = `Regular template with ${variable}`;
            const message = "Just a message";
            // @prompt
            const number = 1;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [],
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
fn mixed_assign() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt def
            let hello;
            // @prompt fresh
            hello = `Hi`;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (26, 56),
                      span: SpanShape(
                        outer: (51, 55),
                        inner: (52, 54),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (52, 54),
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
                              outer: (26, 42),
                              inner: (28, 42),
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
                    "enclosure": "// @prompt fresh\nhello = `Hi`;",
                    "outer": "`Hi`",
                    "inner": "Hi",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hi"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    [
                      {
                        "outer": "// @prompt fresh",
                        "inner": " @prompt fresh"
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
#[ignore = "TODO: Fix annotation collection for reassignments with non-@prompt line comments"]
fn mixed_reassign() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt def
            let hello = 123;
            hello = 456;
            // @prompting
            hello = `Hi`;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (59, 72),
                      span: SpanShape(
                        outer: (67, 71),
                        inner: (68, 70),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (68, 70),
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
                              outer: (0, 14),
                              inner: (2, 14),
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
                    "enclosure": "hello = `Hi`;",
                    "outer": "`Hi`",
                    "inner": "Hi",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hi"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    [
                      {
                        "outer": "// @prompt def",
                        "inner": " @prompt def"
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
#[ignore = "TODO: Fix annotation collection for inline @prompt with non-@prompt leading comments"]
fn mixed_reassign_inline() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt def
            let hello = 123;
            // hello
            hello = /* @prompt fresh */ `Hi`;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (41, 74),
                      span: SpanShape(
                        outer: (69, 73),
                        inner: (70, 72),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (70, 72),
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
                              outer: (49, 68),
                              inner: (51, 66),
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
                    "enclosure": "hello = /* @prompt fresh */ `Hi`;",
                    "outer": "`Hi`",
                    "inner": "Hi",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hi"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    [
                      {
                        "outer": "/* @prompt fresh */",
                        "inner": " @prompt fresh "
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
fn spaced() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt


            const hello = `Hello, world!`;

            // @prompt
            nope()

            const world = "Hello!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 43),
                      span: SpanShape(
                        outer: (27, 42),
                        inner: (28, 41),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (28, 41),
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
                    "enclosure": "// @prompt\n\n\nconst hello = `Hello, world!`;",
                    "outer": "`Hello, world!`",
                    "inner": "Hello, world!",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, world!"
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

#[test]
fn dirty() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt system
            const system = "You are a helpful assistant.";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 64),
                      span: SpanShape(
                        outer: (33, 63),
                        inner: (34, 62),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (34, 62),
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
                              outer: (0, 17),
                              inner: (2, 17),
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
                    "enclosure": "// @prompt system\nconst system = \"You are a helpful assistant.\";",
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
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    [
                      {
                        "outer": "// @prompt system",
                        "inner": " @prompt system"
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
fn multi() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const hello = "Hello", world = "World";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 50),
                      span: SpanShape(
                        outer: (25, 32),
                        inner: (26, 31),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (26, 31),
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
                              outer: (0, 10),
                              inner: (2, 10),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 50),
                      span: SpanShape(
                        outer: (42, 49),
                        inner: (43, 48),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (43, 48),
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
                assert_json_snapshot!(prompt_source_cuts,  @r#"
                [
                  {
                    "enclosure": "// @prompt\nconst hello = \"Hello\", world = \"World\";",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\nconst hello = \"Hello\", world = \"World\";",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations,  @r#"
                [
                  "Hello",
                  "World"
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
                  ],
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

#[test]
fn destructuring() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const [hello1, world1] = ["Hello", "World"];
            // @prompt
            const { hello2, world2 } = { hello2: "Hello", world2: "World" };
            // @prompt
            const [{ hello3, world3 }] = [{ hello3: "Hello", world3: "World" }];
            // @prompt
            const { hi: [[hello4], { world4 }] } = { hi: [["Hello"], { world3: "World" }] };
            // @prompt
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 55),
                      span: SpanShape(
                        outer: (37, 44),
                        inner: (38, 43),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (38, 43),
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
                              outer: (0, 10),
                              inner: (2, 10),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 55),
                      span: SpanShape(
                        outer: (46, 53),
                        inner: (47, 52),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (47, 52),
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
                              outer: (0, 10),
                              inner: (2, 10),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      enclosure: (56, 131),
                      span: SpanShape(
                        outer: (104, 111),
                        inner: (105, 110),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (105, 110),
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
                              outer: (56, 66),
                              inner: (58, 66),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      enclosure: (56, 131),
                      span: SpanShape(
                        outer: (121, 128),
                        inner: (122, 127),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (122, 127),
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
                              outer: (56, 66),
                              inner: (58, 66),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      enclosure: (132, 211),
                      span: SpanShape(
                        outer: (183, 190),
                        inner: (184, 189),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (184, 189),
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
                              outer: (132, 142),
                              inner: (134, 142),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      enclosure: (132, 211),
                      span: SpanShape(
                        outer: (200, 207),
                        inner: (201, 206),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (201, 206),
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
                              outer: (132, 142),
                              inner: (134, 142),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      enclosure: (212, 303),
                      span: SpanShape(
                        outer: (270, 277),
                        inner: (271, 276),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (271, 276),
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
                              outer: (212, 222),
                              inner: (214, 222),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      enclosure: (212, 303),
                      span: SpanShape(
                        outer: (290, 297),
                        inner: (291, 296),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (291, 296),
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
                              outer: (212, 222),
                              inner: (214, 222),
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
                assert_json_snapshot!(prompt_source_cuts,  @r#"
                [
                  {
                    "enclosure": "// @prompt\nconst [hello1, world1] = [\"Hello\", \"World\"];",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\nconst [hello1, world1] = [\"Hello\", \"World\"];",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\nconst { hello2, world2 } = { hello2: \"Hello\", world2: \"World\" };",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\nconst { hello2, world2 } = { hello2: \"Hello\", world2: \"World\" };",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\nconst [{ hello3, world3 }] = [{ hello3: \"Hello\", world3: \"World\" }];",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\nconst [{ hello3, world3 }] = [{ hello3: \"Hello\", world3: \"World\" }];",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\nconst { hi: [[hello4], { world4 }] } = { hi: [[\"Hello\"], { world3: \"World\" }] };",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\nconst { hi: [[hello4], { world4 }] } = { hi: [[\"Hello\"], { world3: \"World\" }] };",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations,  @r#"
                [
                  "Hello",
                  "World",
                  "Hello",
                  "World",
                  "Hello",
                  "World",
                  "Hello",
                  "World"
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
                        "outer": "// @prompt",
                        "inner": " @prompt"
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
                        "outer": "// @prompt",
                        "inner": " @prompt"
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
                        "outer": "// @prompt",
                        "inner": " @prompt"
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
                  ]
                ]
                "#);
            }),
        },
    );
}

#[test]
fn chained() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
	          let world;
            // @prompt
            const hello = world = "Hi"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (12, 50),
                      span: SpanShape(
                        outer: (46, 50),
                        inner: (47, 49),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (47, 49),
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
                              outer: (12, 22),
                              inner: (14, 22),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      enclosure: (12, 50),
                      span: SpanShape(
                        outer: (46, 50),
                        inner: (47, 49),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (47, 49),
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
                              outer: (12, 22),
                              inner: (14, 22),
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
                assert_json_snapshot!(prompt_source_cuts,  @r#"
                [
                  {
                    "enclosure": "// @prompt\n const hello = world = \"Hi\"",
                    "outer": "\"Hi\"",
                    "inner": "Hi",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\n const hello = world = \"Hi\"",
                    "outer": "\"Hi\"",
                    "inner": "Hi",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations,  @r#"
                [
                  "Hi",
                  "Hi"
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
                  ],
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
