use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            // @prompt
            $system = "You are a helpful assistant.";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (6, 57),
                      span: SpanShape(
                        outer: (27, 57),
                        inner: (28, 56),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (28, 56),
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
                              outer: (6, 16),
                              inner: (8, 16),
                            ),
                          ],
                          exp: "// @prompt",
                        ),
                      ],
                      exp: "\"You are a helpful assistant.\"",
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "// @prompt\n$system = \"You are a helpful assistant.\"",
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

#[ignore]
#[test]
fn inline() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            /* @prompt */
            $hello = "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[ignore]
#[test]
fn doc() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            /**
             * @prompt
             */
            $hello = "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[test]
fn assigned() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            // @prompt
            $assigned = "Assigned {$value}";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (6, 48),
                      span: SpanShape(
                        outer: (29, 48),
                        inner: (30, 47),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (30, 39),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (39, 47),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (39, 47),
                            inner: (40, 46),
                          ),
                          exp: "{$value}",
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (6, 16),
                              inner: (8, 16),
                            ),
                          ],
                          exp: "// @prompt",
                        ),
                      ],
                      exp: "\"Assigned {$value}\"",
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "// @prompt\n$assigned = \"Assigned {$value}\"",
                    "outer": "\"Assigned {$value}\"",
                    "inner": "Assigned {$value}",
                    "vars": [
                      {
                        "outer": "{$value}",
                        "inner": "$value"
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
        &ParseTestLang::php(indoc! {r#"
            <?php
            $assigned = "Assigned";
            // @prompt
            $assigned = "Assigned again";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (30, 69),
                      span: SpanShape(
                        outer: (53, 69),
                        inner: (54, 68),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (54, 68),
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
                              outer: (30, 40),
                              inner: (32, 40),
                            ),
                          ],
                          exp: "// @prompt",
                        ),
                      ],
                      exp: "\"Assigned again\"",
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "// @prompt\n$assigned = \"Assigned again\"",
                    "outer": "\"Assigned again\"",
                    "inner": "Assigned again",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Assigned again"
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
        &ParseTestLang::php(indoc! {r#"
            <?php
            // @prompt
            $reassigned = "First";
            $reassigned = "Second";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (6, 38),
                      span: SpanShape(
                        outer: (31, 38),
                        inner: (32, 37),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (32, 37),
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
                              outer: (6, 16),
                              inner: (8, 16),
                            ),
                          ],
                          exp: "// @prompt",
                        ),
                      ],
                      exp: "\"First\"",
                    ),
                    Prompt(
                      file: "prompts.php",
                      enclosure: (40, 62),
                      span: SpanShape(
                        outer: (54, 62),
                        inner: (55, 61),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (55, 61),
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
                              outer: (6, 16),
                              inner: (8, 16),
                            ),
                          ],
                          exp: "// @prompt",
                        ),
                      ],
                      exp: "\"Second\"",
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "// @prompt\n$reassigned = \"First\"",
                    "outer": "\"First\"",
                    "inner": "First",
                    "vars": []
                  },
                  {
                    "enclosure": "$reassigned = \"Second\"",
                    "outer": "\"Second\"",
                    "inner": "Second",
                    "vars": []
                  }
                ]
                "#);
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

#[ignore]
#[test]
fn spaced() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            // @prompt


            $hello = "Hello, world!";

            // @prompt
            nope();

            $world = "Hello!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[ignore]
#[test]
fn mixed() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            // @prompt
            $number = 42;
            $hello = "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[ignore]
#[test]
fn dirty() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            // @prompt system
            $system = "You are a helpful assistant.";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[ignore]
#[test]
fn inexact() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            // @prompting
            $hello = "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      span: SpanShape(
                        outer: Span(
                          start: 35,
                          end: 49,
                        ),
                        inner: Span(
                          start: 36,
                          end: 48,
                        ),
                      ),
                      enclosure: Span(
                        start: 6,
                        end: 49,
                      ),
                      exp: "\"Exact prompt\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 6,
                            end: 16,
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
                    "enclosure": "// @prompt\n$inexact_prompt = \"Exact prompt\"",
                    "outer": "\"Exact prompt\"",
                    "inner": "Exact prompt",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Exact prompt"
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

#[ignore]
#[test]
fn mixed_assign() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            // @prompt def
            $hello = null;
            // @prompt fresh
            $hello = "Hi";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[ignore]
#[test]
fn mixed_none() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            $regular_string = "This is not special";
            $normal_string = "This is not special";
            $regular = "Regular string";
            $message = "Just a message";
            // @prompt
            $number = 1;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[ignore]
#[test]
fn mixed_nested() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            class Hello {
                function world() {
                    // @prompt
                    $hello = 42;

                    // @prompt
                    $hi = 42;

                    $hi = "Hi!";
                }
            }

            $hello = "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[test]
fn destructuring() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            // @prompt
            list($hello1, $world1) = ["Hello", "World"];
            // @prompt
            [$hello2, $world2] = ["Hello", "World"];
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (6, 60),
                      span: SpanShape(
                        outer: (43, 50),
                        inner: (44, 49),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (44, 49),
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
                              outer: (6, 16),
                              inner: (8, 16),
                            ),
                          ],
                          exp: "// @prompt",
                        ),
                      ],
                      exp: "\"Hello\"",
                    ),
                    Prompt(
                      file: "prompts.php",
                      enclosure: (6, 60),
                      span: SpanShape(
                        outer: (52, 59),
                        inner: (53, 58),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (53, 58),
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
                              outer: (6, 16),
                              inner: (8, 16),
                            ),
                          ],
                          exp: "// @prompt",
                        ),
                      ],
                      exp: "\"World\"",
                    ),
                    Prompt(
                      file: "prompts.php",
                      enclosure: (62, 112),
                      span: SpanShape(
                        outer: (95, 102),
                        inner: (96, 101),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (96, 101),
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
                              outer: (62, 72),
                              inner: (64, 72),
                            ),
                          ],
                          exp: "// @prompt",
                        ),
                      ],
                      exp: "\"Hello\"",
                    ),
                    Prompt(
                      file: "prompts.php",
                      enclosure: (62, 112),
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
                              outer: (62, 72),
                              inner: (64, 72),
                            ),
                          ],
                          exp: "// @prompt",
                        ),
                      ],
                      exp: "\"World\"",
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "// @prompt\nlist($hello1, $world1) = [\"Hello\", \"World\"]",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\nlist($hello1, $world1) = [\"Hello\", \"World\"]",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\n[$hello2, $world2] = [\"Hello\", \"World\"]",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\n[$hello2, $world2] = [\"Hello\", \"World\"]",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  }
                ]
                "#);
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
                  ]
                ]
                "#);
            }),
        },
    );
}

#[ignore]
#[test]
fn chained() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            // @prompt
            $hello = $world = "Hi";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      span: SpanShape(
                        outer: Span(
                          start: 35,
                          end: 39,
                        ),
                        inner: Span(
                          start: 36,
                          end: 38,
                        ),
                      ),
                      enclosure: Span(
                        start: 6,
                        end: 39,
                      ),
                      exp: "\"Hi\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 6,
                            end: 16,
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
                    "enclosure": "// @prompt\n$hello = $world = \"Hi\"",
                    "outer": "\"Hi\"",
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
                    "// @prompt"
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
        &ParseTestLang::php(indoc! {r#"
            <?php
            // @prompt def
            $hello = 123;
            $hello = 456;
            // @prompting
            $hello = "Hi";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (49, 76),
                      span: SpanShape(
                        outer: (72, 76),
                        inner: (73, 75),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (73, 75),
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
                              outer: (6, 20),
                              inner: (8, 20),
                            ),
                          ],
                          exp: "// @prompt def",
                        ),
                      ],
                      exp: "\"Hi\"",
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "// @prompting\n$hello = \"Hi\"",
                    "outer": "\"Hi\"",
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

#[ignore]
#[test]
fn multi() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            // @prompt
            [$hello, $world] = ["Hello", "World"];
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
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
                        start: 6,
                        end: 54,
                      ),
                      exp: "\"Hello\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 6,
                            end: 16,
                          ),
                          exp: "// @prompt",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.php",
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
                        start: 6,
                        end: 54,
                      ),
                      exp: "\"World\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 6,
                            end: 16,
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
                    "enclosure": "// @prompt\n[$hello, $world] = [\"Hello\", \"World\"]",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\n[$hello, $world] = [\"Hello\", \"World\"]",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  }
                ]
                "#);
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
