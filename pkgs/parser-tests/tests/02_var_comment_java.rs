use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String system = "You are a helpful assistant.";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
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
                    "enclosure": "// @prompt\nString system = \"You are a helpful assistant.\";",
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
        &ParseTestLang::java(indoc! {r#"
            /* @prompt */
            String hello = "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
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
                    "enclosure": "/* @prompt */\nString hello = \"Hello, world!\";",
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
        &ParseTestLang::java(indoc! {r#"
            /**
             * @prompt
             */
            String hello = "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
                      enclosure: (0, 50),
                      span: SpanShape(
                        outer: (34, 49),
                        inner: (35, 48),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (35, 48),
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
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "/**\n * @prompt\n */\nString hello = \"Hello, world!\";",
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
#[ignore = "TODO: `Hello` is not detected as prompt."]
fn assigned() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String assigned;
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
#[ignore = "TODO: `Assigned again` is not detected as prompt."]
fn assigned_late_comment() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            String assigned = "Assigned";
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
#[ignore = "TODO: Both `First` and `Second` must be detected as prompts."]
fn reassigned() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String reassigned = "First";
            reassigned = "Second";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
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
                    "enclosure": "// @prompt\nString reassigned = \"First\";",
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
        &ParseTestLang::java(indoc! {r#"
            // @prompting
            String hello = "Hello, world!";
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
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            int number = 42;
            String hello = "Hello, world!";
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
#[ignore = "TODO: `Hi!` must be detected as prompt."]
fn mixed_nested() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            class Hello {
                void world() {
                    // @prompt
                    int hello = 42;

                    // @prompt
                    int hi = 42;

                    hi = "Hi!";
                }
            }

            String hello = "Hello, world!";
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
fn mixed_none() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            String regularString = "This is not special";
            String normalString = "This is not special";
            String regular = "Regular string";
            String message = "Just a message";
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
#[ignore = "TODO: `Hi` must be detected as prompt."]
fn mixed_assign() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            // @prompt def
            String hello;
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
        &ParseTestLang::java(indoc! {r#"
            // @prompt


            String hello = "Hello, world!";

            // @prompt
            nope();

            String world = "Hello!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
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
                    "enclosure": "// @prompt\n\n\nString hello = \"Hello, world!\";",
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
        &ParseTestLang::java(indoc! {r#"
            // @prompt system
            String system = "You are a helpful assistant.";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
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
                    "enclosure": "// @prompt system\nString system = \"You are a helpful assistant.\";",
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
fn multi() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String hello = "Hello", world = "World";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
                      enclosure: (0, 51),
                      span: SpanShape(
                        outer: (26, 33),
                        inner: (27, 32),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (27, 32),
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
                      file: "Prompts.java",
                      enclosure: (0, 51),
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
                    "enclosure": "// @prompt\nString hello = \"Hello\", world = \"World\";",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  },
                  {
                    "enclosure": "// @prompt\nString hello = \"Hello\", world = \"World\";",
                    "outer": "\"World\"",
                    "inner": "World",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello",
                  "World"
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
