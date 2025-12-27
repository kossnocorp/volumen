use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

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
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (0, 49),
                      span: SpanShape(
                        outer: (19, 49),
                        inner: (20, 48),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (20, 48),
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
                              outer: (0, 9),
                              inner: (1, 9),
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
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
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
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (25, 55),
                      span: SpanShape(
                        outer: (36, 55),
                        inner: (38, 54),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (38, 47),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (47, 54),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (47, 54),
                            inner: (48, 53),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (0, 9),
                              inner: (1, 9),
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
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
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
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (15, 55),
                      span: SpanShape(
                        outer: (36, 55),
                        inner: (38, 54),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (38, 47),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (47, 54),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (47, 54),
                            inner: (48, 53),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (15, 24),
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
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
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
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (62, 96),
                      span: SpanShape(
                        outer: (75, 96),
                        inner: (77, 95),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (77, 88),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (88, 95),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (88, 95),
                            inner: (89, 94),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (0, 9),
                              inner: (1, 9),
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
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
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
                      enclosure: (113, 123),
                      span: SpanShape(
                        outer: (118, 123),
                        inner: (119, 122),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (119, 122),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
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
                assert_json_snapshot!(annotations, @"
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
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (25, 53),
                      span: SpanShape(
                        outer: (49, 53),
                        inner: (50, 52),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (50, 52),
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
                              outer: (25, 40),
                              inner: (26, 40),
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
                    [
                      {
                        "outer": "# @prompt fresh",
                        "inner": " @prompt fresh"
                      }
                    ]
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
#[ignore = "TODO: Fix annotation collection for reassignments with non-@prompt line comments"]
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
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (56, 81),
                      span: SpanShape(
                        outer: (77, 81),
                        inner: (78, 80),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (78, 80),
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
                              outer: (56, 68),
                              inner: (57, 68),
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
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompting\nhello = \"Hi\"",
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
                    [
                      {
                        "outer": "# @prompt def",
                        "inner": " @prompt def"
                      }
                    ]
                  ]
                ]
                "##);
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
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (0, 35),
                      span: SpanShape(
                        outer: (20, 35),
                        inner: (21, 34),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (21, 34),
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
                              outer: (0, 9),
                              inner: (1, 9),
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
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn dirty() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt system
            system = "You are a helpful assistant."
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (0, 56),
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
                              outer: (0, 16),
                              inner: (1, 16),
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
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt system\nsystem = \"You are a helpful assistant.\"",
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
                    [
                      {
                        "outer": "# @prompt system",
                        "inner": " @prompt system"
                      }
                    ]
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn multi() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            hello, world = "Hello", "World"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (0, 41),
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
                              outer: (0, 9),
                              inner: (1, 9),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (0, 41),
                      span: SpanShape(
                        outer: (34, 41),
                        inner: (35, 40),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (35, 40),
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
                              outer: (0, 9),
                              inner: (1, 9),
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
                assert_json_snapshot!(prompt_source_cuts,  @r##"
                [
                  {
                    "enclosure": "# @prompt\nhello, world = \"Hello\", \"World\"",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\nhello, world = \"Hello\", \"World\"",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  }
                ]
                "##);
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
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn destructuring() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            hello1, world1 = ("Hello", "World")
            # @prompt
            [hello2, world2] = ("Hello", "World")
            # @prompt
            (hello3, world3) = ("Hello", "World")
            # @prompt
            hello4, world4 = ["Hello", "World"]
            # @prompt
            [hello5, world5] = ["Hello", "World"]
            # @prompt
            (hello6, world6) = ["Hello", "World"]
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (0, 45),
                      span: SpanShape(
                        outer: (28, 35),
                        inner: (29, 34),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (29, 34),
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
                              outer: (0, 9),
                              inner: (1, 9),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (0, 45),
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
                              outer: (0, 9),
                              inner: (1, 9),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (46, 93),
                      span: SpanShape(
                        outer: (76, 83),
                        inner: (77, 82),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (77, 82),
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
                              outer: (46, 55),
                              inner: (47, 55),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (46, 93),
                      span: SpanShape(
                        outer: (85, 92),
                        inner: (86, 91),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (86, 91),
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
                              outer: (46, 55),
                              inner: (47, 55),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (94, 141),
                      span: SpanShape(
                        outer: (124, 131),
                        inner: (125, 130),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (125, 130),
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
                              outer: (94, 103),
                              inner: (95, 103),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (94, 141),
                      span: SpanShape(
                        outer: (133, 140),
                        inner: (134, 139),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (134, 139),
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
                              outer: (94, 103),
                              inner: (95, 103),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (142, 187),
                      span: SpanShape(
                        outer: (170, 177),
                        inner: (171, 176),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (171, 176),
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
                              outer: (142, 151),
                              inner: (143, 151),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (142, 187),
                      span: SpanShape(
                        outer: (179, 186),
                        inner: (180, 185),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (180, 185),
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
                              outer: (142, 151),
                              inner: (143, 151),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (188, 235),
                      span: SpanShape(
                        outer: (218, 225),
                        inner: (219, 224),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (219, 224),
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
                              outer: (188, 197),
                              inner: (189, 197),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (188, 235),
                      span: SpanShape(
                        outer: (227, 234),
                        inner: (228, 233),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (228, 233),
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
                              outer: (188, 197),
                              inner: (189, 197),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (236, 283),
                      span: SpanShape(
                        outer: (266, 273),
                        inner: (267, 272),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (267, 272),
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
                              outer: (236, 245),
                              inner: (237, 245),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (236, 283),
                      span: SpanShape(
                        outer: (275, 282),
                        inner: (276, 281),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (276, 281),
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
                              outer: (236, 245),
                              inner: (237, 245),
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
                assert_json_snapshot!(prompt_source_cuts,  @r##"
                [
                  {
                    "enclosure": "# @prompt\nhello1, world1 = (\"Hello\", \"World\")",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\nhello1, world1 = (\"Hello\", \"World\")",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\n[hello2, world2] = (\"Hello\", \"World\")",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\n[hello2, world2] = (\"Hello\", \"World\")",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\n(hello3, world3) = (\"Hello\", \"World\")",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\n(hello3, world3) = (\"Hello\", \"World\")",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\nhello4, world4 = [\"Hello\", \"World\"]",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\nhello4, world4 = [\"Hello\", \"World\"]",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\n[hello5, world5] = [\"Hello\", \"World\"]",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\n[hello5, world5] = [\"Hello\", \"World\"]",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\n(hello6, world6) = [\"Hello\", \"World\"]",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\n(hello6, world6) = [\"Hello\", \"World\"]",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  }
                ]
                "##);
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
                  "World",
                  "Hello",
                  "World",
                  "Hello",
                  "World"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn chained() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            hello = world = "Hi"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (0, 30),
                      span: SpanShape(
                        outer: (26, 30),
                        inner: (27, 29),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (27, 29),
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
                              outer: (0, 9),
                              inner: (1, 9),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (0, 30),
                      span: SpanShape(
                        outer: (26, 30),
                        inner: (27, 29),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (27, 29),
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
                              outer: (0, 9),
                              inner: (1, 9),
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
                assert_json_snapshot!(prompt_source_cuts,  @r##"
                [
                  {
                    "enclosure": "# @prompt\nhello = world = \"Hi\"",
                    "outer": "\"Hi\"",
                    "inner": "Hi",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\nhello = world = \"Hi\"",
                    "outer": "\"Hi\"",
                    "inner": "Hi",
                    "vars": []
                  }
                ]
                "##);
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
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ]
                ]
                "##);
            }),
        },
    );
}
