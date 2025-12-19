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
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
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
                          start: 21,
                          end: 40,
                        ),
                        inner: Span(
                          start: 22,
                          end: 39,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 40,
                      ),
                      exp: "\"Assigned #{value}\"",
                      vars: [
                        PromptVar(
                          exp: "#{value}",
                          span: SpanShape(
                            outer: Span(
                              start: 31,
                              end: 39,
                            ),
                            inner: Span(
                              start: 33,
                              end: 38,
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
                    "enclosure": "# @prompt\nassigned = \"Assigned #{value}\"",
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
            assigned = "Assigned"
            # @prompt
            assigned = "Assigned again"
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
                          start: 43,
                          end: 59,
                        ),
                        inner: Span(
                          start: 44,
                          end: 58,
                        ),
                      ),
                      enclosure: Span(
                        start: 22,
                        end: 59,
                      ),
                      exp: "\"Assigned again\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 22,
                            end: 31,
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
                    "enclosure": "# @prompt\nassigned = \"Assigned again\"",
                    "outer": "\"Assigned again\"",
                    "inner": "Assigned again",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Assigned again"
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
            reassigned = "First"
            reassigned = "Second"
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
                          start: 23,
                          end: 30,
                        ),
                        inner: Span(
                          start: 24,
                          end: 29,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 30,
                      ),
                      exp: "\"First\"",
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
                          start: 44,
                          end: 52,
                        ),
                        inner: Span(
                          start: 45,
                          end: 51,
                        ),
                      ),
                      enclosure: Span(
                        start: 31,
                        end: 52,
                      ),
                      exp: "\"Second\"",
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
                    "enclosure": "# @prompt\nreassigned = \"First\"",
                    "outer": "\"First\"",
                    "inner": "First",
                    "vars": []
                  },
                  {
                    "enclosure": "reassigned = \"Second\"",
                    "outer": "\"Second\"",
                    "inner": "Second",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "First",
                  "Second"
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
fn spaced() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # This is a comment
            # @prompt
            # This is another comment
            spaced = "Spaced"
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
                          start: 65,
                          end: 73,
                        ),
                        inner: Span(
                          start: 66,
                          end: 72,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 73,
                      ),
                      exp: "\"Spaced\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 55,
                          ),
                          exp: "# This is a comment\n# @prompt\n# This is another comment",
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
                    "enclosure": "# This is a comment\n# @prompt\n# This is another comment\nspaced = \"Spaced\"",
                    "outer": "\"Spaced\"",
                    "inner": "Spaced",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Spaced"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# This is a comment\n# @prompt\n# This is another comment"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn mixed() {
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
                    "# @prompt system"
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
            dirty = 123
            # @prompt
            dirty = "Dirty"
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
                          start: 30,
                          end: 37,
                        ),
                        inner: Span(
                          start: 31,
                          end: 36,
                        ),
                      ),
                      enclosure: Span(
                        start: 12,
                        end: 37,
                      ),
                      exp: "\"Dirty\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 12,
                            end: 21,
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
                    "enclosure": "# @prompt\ndirty = \"Dirty\"",
                    "outer": "\"Dirty\"",
                    "inner": "Dirty",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Dirty"
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
            # @prompt
            inexact_prompt = "Exact prompt"
            inexact = "Inexact"
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
                          start: 27,
                          end: 41,
                        ),
                        inner: Span(
                          start: 28,
                          end: 40,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 41,
                      ),
                      exp: "\"Exact prompt\"",
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
                    "enclosure": "# @prompt\ninexact_prompt = \"Exact prompt\"",
                    "outer": "\"Exact prompt\"",
                    "inner": "Exact prompt",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Exact prompt"
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
fn mixed_assign() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            mixed_prompt = "First"
            mixed = "Second"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: Span(
                          start: 15,
                          end: 22,
                        ),
                        inner: Span(
                          start: 16,
                          end: 21,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 22,
                      ),
                      exp: "\"First\"",
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
                    "enclosure": "mixed_prompt = \"First\"",
                    "outer": "\"First\"",
                    "inner": "First",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "First"
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
        &ParseTestLang::rb(indoc! {r#"
            prompt = "First"
            second = "Second"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: Span(
                          start: 9,
                          end: 16,
                        ),
                        inner: Span(
                          start: 10,
                          end: 15,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 16,
                      ),
                      exp: "\"First\"",
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
                    "enclosure": "prompt = \"First\"",
                    "outer": "\"First\"",
                    "inner": "First",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "First"
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
fn mixed_nested() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            def my_function
              prompt = "First"
              second = "Second"
            end
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: Span(
                          start: 27,
                          end: 34,
                        ),
                        inner: Span(
                          start: 28,
                          end: 33,
                        ),
                      ),
                      enclosure: Span(
                        start: 18,
                        end: 34,
                      ),
                      exp: "\"First\"",
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
                    "enclosure": "prompt = \"First\"",
                    "outer": "\"First\"",
                    "inner": "First",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "First"
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
