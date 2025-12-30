use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            string system = "You are a helpful assistant.";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
                      enclosure: (0, 58),
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
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "// @prompt\nstring system = \"You are a helpful assistant.\";",
                    "outer": "\"You are a helpful assistant.\"",
                    "inner": "You are a helpful assistant.",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "You are a helpful assistant."
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
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
        &ParseTestLang::cs(indoc! {r#"
            /* @prompt */
            string hello = "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
                      enclosure: (0, 45),
                      span: SpanShape(
                        outer: (29, 44),
                        inner: (30, 43),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (30, 43),
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
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "/* @prompt */\nstring hello = \"Hello, world!\";",
                    "outer": "\"Hello, world!\"",
                    "inner": "Hello, world!",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello, world!"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
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
        &ParseTestLang::cs(indoc! {r#"
            /// @prompt
            string hello = "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
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
                              outer: (0, 11),
                              inner: (3, 11),
                            ),
                          ],
                        ),
                      ],
                    ),
                  ],
                )
                "#);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "/// @prompt\nstring hello = \"Hello, world!\";",
                    "outer": "\"Hello, world!\"",
                    "inner": "Hello, world!",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello, world!"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
                [
                  [
                    [
                      {
                        "outer": "/// @prompt",
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
#[ignore = "TODO: `Assigned value` must detect as a prompt as the var declaration has the `// @prompt` annotation."]
fn assigned() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            string assigned;
            assigned = "Assigned value";
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
#[ignore = "TODO: `Assigned value` must detect as a prompt as the var declaration has the `// @prompt` annotation."]
fn assigned_late_comment() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            string assigned = "Assigned";
            // @prompt
            assigned = "Assigned again";
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
#[ignore = "TODO: It must detect both prompts, the second one is not being detected. Also, add this test to `py`, `rb`, `php`, `go`, and `java`."]
fn reassigned_strings() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            string reassigned = "First";
            reassigned = "Second";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
                      enclosure: (0, 39),
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
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "// @prompt\nstring reassigned = \"First\";",
                    "outer": "\"First\"",
                    "inner": "First",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "First"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
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
        &ParseTestLang::cs(indoc! {r#"
            // @prompting
            string hello = "Hello, world!";
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
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            int number = 42;
            string hello = "Hello, world!";
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
fn mixed_nested() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            class Hello {
                void World() {
                    // @prompt
                    int hello = 42;

                    // @prompt
                    int hi = 42;

                    hi = "Hi!";
                }
            }

            string hello = "Hello, world!";
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
#[ignore = "Both \"42\" and \"Hi!\" must be detected as prompts since the variable `hi` has the `// @prompt` annotation. Also, add this test to `py`, `rb`, `php`, `go`, and `java`."]
fn mixed_nested_strings() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            class Hello {
                void World() {
                    // @prompt
                    int hello = 42;

                    // @prompt
                    string hi = "42";

                    hi = "Hi!";
                }
            }

            string hello = "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
                      enclosure: (85, 121),
                      span: SpanShape(
                        outer: (116, 120),
                        inner: (117, 119),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (117, 119),
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
                              outer: (85, 95),
                              inner: (87, 95),
                            ),
                          ],
                        ),
                      ],
                    ),
                  ],
                )
                "#);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "// @prompt\n        string hi = \"42\";",
                    "outer": "\"42\"",
                    "inner": "42",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "42"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
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
        &ParseTestLang::cs(indoc! {r#"
            string regularString = "This is not special";
            string normalString = "This is not special";
            string regular = "Regular string";
            string message = "Just a message";
            // @prompt
            int number = 1;
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
#[ignore = "TODO: `Hi` must be detected as a prompt since the variable declaration has the `// @prompt` annotation."]
fn mixed_assign() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt def
            string hello;
            // @prompt fresh
            hello = "Hi";
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
fn spaced() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt


            string hello = "Hello, world!";

            // @prompt
            nope();

            string world = "Hello!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
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
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "// @prompt\n\n\nstring hello = \"Hello, world!\";",
                    "outer": "\"Hello, world!\"",
                    "inner": "Hello, world!",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello, world!"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
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
        &ParseTestLang::cs(indoc! {r#"
            // @prompt system
            string system = "You are a helpful assistant.";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
                      enclosure: (0, 65),
                      span: SpanShape(
                        outer: (34, 64),
                        inner: (35, 63),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (35, 63),
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
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "// @prompt system\nstring system = \"You are a helpful assistant.\";",
                    "outer": "\"You are a helpful assistant.\"",
                    "inner": "You are a helpful assistant.",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "You are a helpful assistant."
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
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
#[ignore = "TODO: This is not multi test, see pkgs/parser-tests/tests/02_var_comment_ts.rs. It is destructuring (same, use TS as reference). Also, chained is missing. Is it correct?"]
fn multi() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            (string hello, string world) = ("Hello", "World");
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
