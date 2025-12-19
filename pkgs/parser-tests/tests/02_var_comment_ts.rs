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
                      span: SpanShape(
                        outer: Span(
                          start: 26,
                          end: 56,
                        ),
                        inner: Span(
                          start: 27,
                          end: 55,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 57,
                      ),
                      exp: "\"You are a helpful assistant.\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 10,
                          ),
                          exp: "// @prompt",
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
                    "// @prompt"
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
                      span: SpanShape(
                        outer: Span(
                          start: 28,
                          end: 43,
                        ),
                        inner: Span(
                          start: 29,
                          end: 42,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 44,
                      ),
                      exp: "`Hello, world!`",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 13,
                          ),
                          exp: "/* @prompt */",
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
                    "/* @prompt */"
                  ]
                ]
                "#);
            }),
        },
    );
}

#[test]
fn jsdoc() {
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
                      span: SpanShape(
                        outer: Span(
                          start: 33,
                          end: 48,
                        ),
                        inner: Span(
                          start: 34,
                          end: 47,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 49,
                      ),
                      exp: "`Hello, world!`",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 18,
                          ),
                          exp: "/**\n * @prompt\n */",
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
                    "/**\n * @prompt\n */"
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
                      span: SpanShape(
                        outer: Span(
                          start: 36,
                          end: 55,
                        ),
                        inner: Span(
                          start: 37,
                          end: 54,
                        ),
                      ),
                      enclosure: Span(
                        start: 25,
                        end: 56,
                      ),
                      exp: "`Assigned ${value}`",
                      vars: [
                        PromptVar(
                          exp: "${value}",
                          span: SpanShape(
                            outer: Span(
                              start: 46,
                              end: 54,
                            ),
                            inner: Span(
                              start: 48,
                              end: 53,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 10,
                          ),
                          exp: "// @prompt",
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
                    "// @prompt"
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
                      span: SpanShape(
                        outer: Span(
                          start: 36,
                          end: 55,
                        ),
                        inner: Span(
                          start: 37,
                          end: 54,
                        ),
                      ),
                      enclosure: Span(
                        start: 14,
                        end: 56,
                      ),
                      exp: "`Assigned ${value}`",
                      vars: [
                        PromptVar(
                          exp: "${value}",
                          span: SpanShape(
                            outer: Span(
                              start: 46,
                              end: 54,
                            ),
                            inner: Span(
                              start: 48,
                              end: 53,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 14,
                            end: 24,
                          ),
                          exp: "// @prompt",
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
                    "// @prompt"
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
                      span: SpanShape(
                        outer: Span(
                          start: 64,
                          end: 85,
                        ),
                        inner: Span(
                          start: 65,
                          end: 84,
                        ),
                      ),
                      enclosure: Span(
                        start: 51,
                        end: 86,
                      ),
                      exp: "`Reassigned ${value}`",
                      vars: [
                        PromptVar(
                          exp: "${value}",
                          span: SpanShape(
                            outer: Span(
                              start: 76,
                              end: 84,
                            ),
                            inner: Span(
                              start: 78,
                              end: 83,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 10,
                          ),
                          exp: "// @prompt",
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
                    "// @prompt"
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
                      span: SpanShape(
                        outer: Span(
                          start: 130,
                          end: 135,
                        ),
                        inner: Span(
                          start: 131,
                          end: 134,
                        ),
                      ),
                      enclosure: Span(
                        start: 125,
                        end: 135,
                      ),
                      exp: "\"Hi!\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 84,
                            end: 94,
                          ),
                          exp: "// @prompt",
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
                    "// @prompt"
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
                      span: SpanShape(
                        outer: Span(
                          start: 51,
                          end: 55,
                        ),
                        inner: Span(
                          start: 52,
                          end: 54,
                        ),
                      ),
                      enclosure: Span(
                        start: 26,
                        end: 56,
                      ),
                      exp: "`Hi`",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 26,
                            end: 42,
                          ),
                          exp: "// @prompt fresh",
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
                    "// @prompt fresh"
                  ]
                ]
                "#);
            }),
        },
    );
}

#[test]
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
                      span: SpanShape(
                        outer: Span(
                          start: 67,
                          end: 71,
                        ),
                        inner: Span(
                          start: 68,
                          end: 70,
                        ),
                      ),
                      enclosure: Span(
                        start: 45,
                        end: 72,
                      ),
                      exp: "`Hi`",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 14,
                          ),
                          exp: "// @prompt def",
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
                    "enclosure": "// @prompting\nhello = `Hi`;",
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
                    "// @prompt def"
                  ]
                ]
                "#);
            }),
        },
    );
}

#[test]
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
                      span: SpanShape(
                        outer: Span(
                          start: 69,
                          end: 73,
                        ),
                        inner: Span(
                          start: 70,
                          end: 72,
                        ),
                      ),
                      enclosure: Span(
                        start: 32,
                        end: 74,
                      ),
                      exp: "`Hi`",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 32,
                            end: 40,
                          ),
                          exp: "// hello",
                        ),
                        PromptAnnotation(
                          span: Span(
                            start: 49,
                            end: 68,
                          ),
                          exp: "/* @prompt fresh */",
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
                    "enclosure": "// hello\nhello = /* @prompt fresh */ `Hi`;",
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
                    "// hello",
                    "/* @prompt fresh */"
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
                      span: SpanShape(
                        outer: Span(
                          start: 27,
                          end: 42,
                        ),
                        inner: Span(
                          start: 28,
                          end: 41,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 43,
                      ),
                      exp: "`Hello, world!`",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 10,
                          ),
                          exp: "// @prompt",
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
                    "// @prompt"
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
                      span: SpanShape(
                        outer: Span(
                          start: 33,
                          end: 63,
                        ),
                        inner: Span(
                          start: 34,
                          end: 62,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 64,
                      ),
                      exp: "\"You are a helpful assistant.\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 17,
                          ),
                          exp: "// @prompt system",
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
                    "// @prompt system"
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
                      span: SpanShape(
                        outer: Span(
                          start: 25,
                          end: 32,
                        ),
                        inner: Span(
                          start: 26,
                          end: 31,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 50,
                      ),
                      exp: "\"Hello\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 10,
                          ),
                          exp: "// @prompt",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 42,
                          end: 49,
                        ),
                        inner: Span(
                          start: 43,
                          end: 48,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 50,
                      ),
                      exp: "\"World\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 10,
                          ),
                          exp: "// @prompt",
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
                    "// @prompt"
                  ],
                  [
                    "// @prompt"
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
                      span: SpanShape(
                        outer: Span(
                          start: 37,
                          end: 44,
                        ),
                        inner: Span(
                          start: 38,
                          end: 43,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 55,
                      ),
                      exp: "\"Hello\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 10,
                          ),
                          exp: "// @prompt",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 46,
                          end: 53,
                        ),
                        inner: Span(
                          start: 47,
                          end: 52,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 55,
                      ),
                      exp: "\"World\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 10,
                          ),
                          exp: "// @prompt",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 104,
                          end: 111,
                        ),
                        inner: Span(
                          start: 105,
                          end: 110,
                        ),
                      ),
                      enclosure: Span(
                        start: 56,
                        end: 131,
                      ),
                      exp: "\"Hello\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 56,
                            end: 66,
                          ),
                          exp: "// @prompt",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 121,
                          end: 128,
                        ),
                        inner: Span(
                          start: 122,
                          end: 127,
                        ),
                      ),
                      enclosure: Span(
                        start: 56,
                        end: 131,
                      ),
                      exp: "\"World\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 56,
                            end: 66,
                          ),
                          exp: "// @prompt",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 183,
                          end: 190,
                        ),
                        inner: Span(
                          start: 184,
                          end: 189,
                        ),
                      ),
                      enclosure: Span(
                        start: 132,
                        end: 211,
                      ),
                      exp: "\"Hello\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 132,
                            end: 142,
                          ),
                          exp: "// @prompt",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 200,
                          end: 207,
                        ),
                        inner: Span(
                          start: 201,
                          end: 206,
                        ),
                      ),
                      enclosure: Span(
                        start: 132,
                        end: 211,
                      ),
                      exp: "\"World\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 132,
                            end: 142,
                          ),
                          exp: "// @prompt",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 270,
                          end: 277,
                        ),
                        inner: Span(
                          start: 271,
                          end: 276,
                        ),
                      ),
                      enclosure: Span(
                        start: 212,
                        end: 303,
                      ),
                      exp: "\"Hello\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 212,
                            end: 222,
                          ),
                          exp: "// @prompt",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 290,
                          end: 297,
                        ),
                        inner: Span(
                          start: 291,
                          end: 296,
                        ),
                      ),
                      enclosure: Span(
                        start: 212,
                        end: 303,
                      ),
                      exp: "\"World\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 212,
                            end: 222,
                          ),
                          exp: "// @prompt",
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
                    "// @prompt"
                  ],
                  [
                    "// @prompt"
                  ],
                  [
                    "// @prompt"
                  ],
                  [
                    "// @prompt"
                  ],
                  [
                    "// @prompt"
                  ],
                  [
                    "// @prompt"
                  ],
                  [
                    "// @prompt"
                  ],
                  [
                    "// @prompt"
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
                      span: SpanShape(
                        outer: Span(
                          start: 62,
                          end: 66,
                        ),
                        inner: Span(
                          start: 63,
                          end: 65,
                        ),
                      ),
                      enclosure: Span(
                        start: 20,
                        end: 66,
                      ),
                      exp: "\"Hi\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 20,
                            end: 30,
                          ),
                          exp: "// @prompt",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 62,
                          end: 66,
                        ),
                        inner: Span(
                          start: 63,
                          end: 65,
                        ),
                      ),
                      enclosure: Span(
                        start: 20,
                        end: 66,
                      ),
                      exp: "\"Hi\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 20,
                            end: 30,
                          ),
                          exp: "// @prompt",
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
                    "enclosure": "// @prompt\n         const hello = world = \"Hi\"",
                    "outer": "\"Hi\"",
                    "inner": "Hi",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\n         const hello = world = \"Hi\"",
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
                    "// @prompt"
                  ],
                  [
                    "// @prompt"
                  ]
                ]
                "#);
            }),
        },
    );
}
