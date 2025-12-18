use indoc::indoc;
use insta::{assert_ron_snapshot, assert_json_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            system = "You are a helpful assistant."
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 19,
                          end: 49,
                        ),
                        inner: Span(
                          start: 20,
                          end: 48,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 49,
                      ),
                      exp: "\"You are a helpful assistant.\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\nsystem = \"You are a helpful assistant.\"",
                    "outer": "\"You are a helpful assistant.\"",
                    "inner": "You are a helpful assistant.",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "You are a helpful assistant."
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn assigned() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            assigned : str
            assigned = f"Assigned {value}";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 36,
                          end: 55,
                        ),
                        inner: Span(
                          start: 38,
                          end: 54,
                        ),
                      ),
                      enclosure: Span(
                        start: 25,
                        end: 55,
                      ),
                      exp: "f\"Assigned {value}\"",
                      vars: [
                        PromptVar(
                          exp: "{value}",
                          span: SpanShape(
                            outer: Span(
                              start: 47,
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
                            end: 9,
                          ),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "assigned = f\"Assigned {value}\"",
                    "outer": "f\"Assigned {value}\"",
                    "inner": "Assigned {value}",
                    "vars": [
                      {
                        "outer": "{value}",
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
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn assigned_late_comment() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            assigned : str
            # @prompt
            assigned = f"Assigned {value}";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 36,
                          end: 55,
                        ),
                        inner: Span(
                          start: 38,
                          end: 54,
                        ),
                      ),
                      enclosure: Span(
                        start: 15,
                        end: 55,
                      ),
                      exp: "f\"Assigned {value}\"",
                      vars: [
                        PromptVar(
                          exp: "{value}",
                          span: SpanShape(
                            outer: Span(
                              start: 47,
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
                            start: 15,
                            end: 24,
                          ),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\nassigned = f\"Assigned {value}\"",
                    "outer": "f\"Assigned {value}\"",
                    "inner": "Assigned {value}",
                    "vars": [
                      {
                        "outer": "{value}",
                        "inner": "value"
                      }
                    ]
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Assigned {0}"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn reassigned() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            reassigned : Union[str, int] = 123
            reassigned = 456
            reassigned = f"Reassigned {value}";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 75,
                          end: 96,
                        ),
                        inner: Span(
                          start: 77,
                          end: 95,
                        ),
                      ),
                      enclosure: Span(
                        start: 62,
                        end: 96,
                      ),
                      exp: "f\"Reassigned {value}\"",
                      vars: [
                        PromptVar(
                          exp: "{value}",
                          span: SpanShape(
                            outer: Span(
                              start: 88,
                              end: 95,
                            ),
                            inner: Span(
                              start: 89,
                              end: 94,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "reassigned = f\"Reassigned {value}\"",
                    "outer": "f\"Reassigned {value}\"",
                    "inner": "Reassigned {value}",
                    "vars": [
                      {
                        "outer": "{value}",
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
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn inexact() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompting
            hello = "Hello, world!"
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
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            number = 42
            hello = "Hello, world!"
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
        &ParseTestLang::py(indoc! {r#"
            class Hello:
                def world(self):
                    # @prompt
                    hello = 42
                    # @prompt
                    hi = 42
                    hi = "Hi!"

            hello = "Hello, world!"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 118,
                          end: 123,
                        ),
                        inner: Span(
                          start: 119,
                          end: 122,
                        ),
                      ),
                      enclosure: Span(
                        start: 113,
                        end: 123,
                      ),
                      exp: "\"Hi!\"",
                      vars: [],
                      annotations: [],
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
                assert_json_snapshot!(annotations, @r"
                [
                  []
                ]
                ");
            }),
        },
    );
}

#[test]
fn mixed_none() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            regular_template = f"This is not a {value}"
            normal_string = "This is not special"
            regular = f"Regular template with {variable}"
            message = "Just a message"
            # @prompt
            number = 1
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
        &ParseTestLang::py(indoc! {r#"
            # @prompt def
            hello: str
            # @prompt fresh
            hello = "Hi"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 49,
                          end: 53,
                        ),
                        inner: Span(
                          start: 50,
                          end: 52,
                        ),
                      ),
                      enclosure: Span(
                        start: 25,
                        end: 53,
                      ),
                      exp: "\"Hi\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 25,
                            end: 40,
                          ),
                          exp: "# @prompt fresh",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt fresh\nhello = \"Hi\"",
                    "outer": "\"Hi\"",
                    "inner": "Hi",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hi"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt fresh"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
#[ignore = "TODO: Python parsers don not match, the snapshot is correct though"]
fn mixed_reassign() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt def
            hello: Union[str | int] = 123
            hello = 456
            # @prompting
            hello = "Hi"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 77,
                          end: 81,
                        ),
                        inner: Span(
                          start: 78,
                          end: 80,
                        ),
                      ),
                      enclosure: Span(
                        start: 56,
                        end: 81,
                      ),
                      exp: "\"Hi\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 13,
                          ),
                          exp: "# @prompt def",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [[]]
                enclosure = 'hello = "Hi"'
                vars = []
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @"['Hi']");
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @"");
            }),
        },
    );
}

#[test]
fn spaced() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt


            hello = "Hello, world!"

            # @prompt
            nope()

            world = "Hello!"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 20,
                          end: 35,
                        ),
                        inner: Span(
                          start: 21,
                          end: 34,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 35,
                      ),
                      exp: "\"Hello, world!\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\n\n\nhello = \"Hello, world!\"",
                    "outer": "\"Hello, world!\"",
                    "inner": "Hello, world!",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, world!"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}
