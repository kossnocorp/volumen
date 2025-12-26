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
                      span: SpanShape(
                        outer: Span(
                          start: 31,
                          end: 50,
                        ),
                        inner: Span(
                          start: 32,
                          end: 49,
                        ),
                      ),
                      enclosure: Span(
                        start: 20,
                        end: 50,
                      ),
                      exp: "\"Assigned #{value}\"",
                      vars: [
                        PromptVar(
                          exp: "#{value}",
                          span: SpanShape(
                            outer: Span(
                              start: 41,
                              end: 49,
                            ),
                            inner: Span(
                              start: 43,
                              end: 48,
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
                      span: SpanShape(
                        outer: Span(
                          start: 31,
                          end: 50,
                        ),
                        inner: Span(
                          start: 32,
                          end: 49,
                        ),
                      ),
                      enclosure: Span(
                        start: 10,
                        end: 50,
                      ),
                      exp: "\"Assigned ${value}\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 10,
                            end: 19,
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
                      span: SpanShape(
                        outer: Span(
                          start: 57,
                          end: 78,
                        ),
                        inner: Span(
                          start: 58,
                          end: 77,
                        ),
                      ),
                      enclosure: Span(
                        start: 44,
                        end: 78,
                      ),
                      exp: "\"Reassigned ${value}\"",
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
                      span: SpanShape(
                        outer: Span(
                          start: 90,
                          end: 95,
                        ),
                        inner: Span(
                          start: 91,
                          end: 94,
                        ),
                      ),
                      enclosure: Span(
                        start: 85,
                        end: 95,
                      ),
                      exp: "\"Hi!\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 58,
                            end: 67,
                          ),
                          exp: "# @prompt",
                        ),
                      ],
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
                    "# @prompt"
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
                      span: SpanShape(
                        outer: Span(
                          start: 50,
                          end: 54,
                        ),
                        inner: Span(
                          start: 51,
                          end: 53,
                        ),
                      ),
                      enclosure: Span(
                        start: 26,
                        end: 54,
                      ),
                      exp: "\"Hi\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 26,
                            end: 41,
                          ),
                          exp: "# @prompt fresh",
                        ),
                      ],
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
                    "# @prompt fresh"
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
                      span: SpanShape(
                        outer: Span(
                          start: 59,
                          end: 63,
                        ),
                        inner: Span(
                          start: 60,
                          end: 62,
                        ),
                      ),
                      enclosure: Span(
                        start: 38,
                        end: 63,
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
                    "# @prompt def"
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
                        end: 56,
                      ),
                      exp: "\"You are a helpful assistant.\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 16,
                          ),
                          exp: "# @prompt system",
                        ),
                      ],
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
                    "# @prompt system"
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
                        end: 41,
                      ),
                      exp: "\"Hello\"",
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
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: Span(
                          start: 34,
                          end: 41,
                        ),
                        inner: Span(
                          start: 35,
                          end: 40,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 41,
                      ),
                      exp: "\"World\"",
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
                    "# @prompt"
                  ],
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
                      span: SpanShape(
                        outer: Span(
                          start: 28,
                          end: 35,
                        ),
                        inner: Span(
                          start: 29,
                          end: 34,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 45,
                      ),
                      exp: "\"Hello\"",
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
                    Prompt(
                      file: "prompts.rb",
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
                        end: 45,
                      ),
                      exp: "\"World\"",
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
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: Span(
                          start: 73,
                          end: 80,
                        ),
                        inner: Span(
                          start: 74,
                          end: 79,
                        ),
                      ),
                      enclosure: Span(
                        start: 46,
                        end: 89,
                      ),
                      exp: "\"Hello\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 46,
                            end: 55,
                          ),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: Span(
                          start: 82,
                          end: 89,
                        ),
                        inner: Span(
                          start: 83,
                          end: 88,
                        ),
                      ),
                      enclosure: Span(
                        start: 46,
                        end: 89,
                      ),
                      exp: "\"World\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 46,
                            end: 55,
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
                    "# @prompt"
                  ],
                  [
                    "# @prompt"
                  ],
                  [
                    "# @prompt"
                  ],
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
                      span: SpanShape(
                        outer: Span(
                          start: 26,
                          end: 30,
                        ),
                        inner: Span(
                          start: 27,
                          end: 29,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 30,
                      ),
                      exp: "\"Hi\"",
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
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: Span(
                          start: 26,
                          end: 30,
                        ),
                        inner: Span(
                          start: 27,
                          end: 29,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 30,
                      ),
                      exp: "\"Hi\"",
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
                    "# @prompt"
                  ],
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}
