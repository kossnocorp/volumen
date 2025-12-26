use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
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
                      file: "prompts.rb",
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
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"You are a helpful assistant.\"",
                    ),
                  ],
                )
                "##);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r##"
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
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "You are a helpful assistant."
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r##"
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
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            assigned;
            assigned = "Assigned #{value}"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (20, 50),
                      span: SpanShape(
                        outer: (31, 50),
                        inner: (32, 49),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (32, 41),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (41, 49),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (41, 49),
                            inner: (43, 48),
                          ),
                          exp: "#{value}",
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
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"Assigned #{value}\"",
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "assigned = \"Assigned #{value}\"",
                    "outer": "\"Assigned #{value}\"",
                    "inner": "Assigned #{value}",
                    "vars": [
                      {
                        "outer": "#{value}",
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
fn assigned_late_comment() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            assigned;
            # @prompt
            assigned = "Assigned ${value}"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (10, 50),
                      span: SpanShape(
                        outer: (31, 50),
                        inner: (32, 49),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (32, 49),
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
                              outer: (10, 19),
                              inner: (11, 19),
                            ),
                          ],
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"Assigned ${value}\"",
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\nassigned = \"Assigned ${value}\"",
                    "outer": "\"Assigned ${value}\"",
                    "inner": "Assigned ${value}",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Assigned ${value}"
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
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            reassigned = 123
            reassigned = 456
            reassigned = "Reassigned ${value}"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (44, 78),
                      span: SpanShape(
                        outer: (57, 78),
                        inner: (58, 77),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (58, 77),
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
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"Reassigned ${value}\"",
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "reassigned = \"Reassigned ${value}\"",
                    "outer": "\"Reassigned ${value}\"",
                    "inner": "Reassigned ${value}",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Reassigned ${value}"
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
        &ParseTestLang::rb(indoc! {r#"
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
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"[]");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"[]");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"[]");
            }),
        },
    );
}

#[test]
fn mixed() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
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
        &ParseTestLang::rb(indoc! {r#"
            class Hello
              def world
                # @prompt
                hello = 42

                # @prompt
                hi = 42

                hi = "Hi!"
              end
            end

            hello = "Hello, world!"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (85, 95),
                      span: SpanShape(
                        outer: (90, 95),
                        inner: (91, 94),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (91, 94),
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
                              outer: (58, 67),
                              inner: (59, 67),
                            ),
                          ],
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"Hi!\"",
                    ),
                  ],
                )
                "##);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
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
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hi!"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r##"
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
fn mixed_none() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            regular_string = "This is not special"
            normal_string = "This is not special"
            regular = "Regular string"
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
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"[]");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"[]");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"[]");
            }),
        },
    );
}

#[test]
fn mixed_assign() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt def
            hello = nil
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
                      file: "prompts.rb",
                      enclosure: (26, 54),
                      span: SpanShape(
                        outer: (50, 54),
                        inner: (51, 53),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (51, 53),
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
                              outer: (26, 41),
                              inner: (27, 41),
                            ),
                          ],
                          exp: "# @prompt fresh",
                        ),
                      ],
                      exp: "\"Hi\"",
                    ),
                  ],
                )
                "##);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r##"
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
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hi"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r##"
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
fn mixed_reassign() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt def
            hello = 123
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
                      file: "prompts.rb",
                      enclosure: (38, 63),
                      span: SpanShape(
                        outer: (59, 63),
                        inner: (60, 62),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (60, 62),
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
                              inner: (1, 13),
                            ),
                          ],
                          exp: "# @prompt def",
                        ),
                      ],
                      exp: "\"Hi\"",
                    ),
                  ],
                )
                "##);
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
        &ParseTestLang::rb(indoc! {r#"
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
                      file: "prompts.rb",
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
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"Hello, world!\"",
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
        &ParseTestLang::rb(indoc! {r#"
            # @prompt system
            system = "You are a helpful assistant."
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
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
                          exp: "# @prompt system",
                        ),
                      ],
                      exp: "\"You are a helpful assistant.\"",
                    ),
                  ],
                )
                "##);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r##"
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
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "You are a helpful assistant."
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r##"
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
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            hello, world = "Hello", "World"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
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
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"Hello\"",
                    ),
                    Prompt(
                      file: "prompts.rb",
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
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"World\"",
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
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
                assert_json_snapshot!(interpolations, @r#"
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
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            hello1, world1 = ["Hello", "World"]
            # @prompt
            hello2, world2 = "Hello", "World"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
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
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"Hello\"",
                    ),
                    Prompt(
                      file: "prompts.rb",
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
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"World\"",
                    ),
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (46, 89),
                      span: SpanShape(
                        outer: (73, 80),
                        inner: (74, 79),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (74, 79),
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
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"Hello\"",
                    ),
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (46, 89),
                      span: SpanShape(
                        outer: (82, 89),
                        inner: (83, 88),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (83, 88),
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
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"World\"",
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\nhello1, world1 = [\"Hello\", \"World\"]",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\nhello1, world1 = [\"Hello\", \"World\"]",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\nhello2, world2 = \"Hello\", \"World\"",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "# @prompt\nhello2, world2 = \"Hello\", \"World\"",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
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
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            hello = world = "Hi"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
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
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"Hi\"",
                    ),
                    Prompt(
                      file: "prompts.rb",
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
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"Hi\"",
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
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
                assert_json_snapshot!(interpolations, @r#"
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
