use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn concat() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            /* @prompt */
            $greeting = "Hello, " . $name . "!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (6, 55),
                      span: SpanShape(
                        outer: (32, 55),
                        inner: (33, 54),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (33, 40),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (44, 49),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (53, 54),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (41, 52),
                            inner: (44, 49),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (6, 19),
                              inner: (8, 17),
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
                    "enclosure": "/* @prompt */\n$greeting = \"Hello, \" . $name . \"!\"",
                    "outer": "\"Hello, \" . $name . \"!\"",
                    "inner": "Hello, \" . $name . \"!",
                    "vars": [
                      {
                        "outer": " . $name . ",
                        "inner": "$name"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, {0}!"
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
fn concat_with_primitives() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            // @prompt
            $message = "Count: " . 42 . ", Active: " . true;
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
fn concat_with_function_calls() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            /* @prompt */
            $greeting = "Hello " . format($name) . " world";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (6, 67),
                      span: SpanShape(
                        outer: (32, 67),
                        inner: (33, 66),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (33, 39),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (43, 56),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (60, 66),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (40, 59),
                            inner: (43, 56),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (6, 19),
                              inner: (8, 17),
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
                    "enclosure": "/* @prompt */\n$greeting = \"Hello \" . format($name) . \" world\"",
                    "outer": "\"Hello \" . format($name) . \" world\"",
                    "inner": "Hello \" . format($name) . \" world",
                    "vars": [
                      {
                        "outer": " . format($name) . ",
                        "inner": "format($name)"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello {0} world"
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

    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            /* @prompt */
            $message = "User: " . $user->getName() . "!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (6, 64),
                      span: SpanShape(
                        outer: (31, 64),
                        inner: (32, 63),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (32, 38),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (42, 58),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (62, 63),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (39, 61),
                            inner: (42, 58),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (6, 19),
                              inner: (8, 17),
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
                    "enclosure": "/* @prompt */\n$message = \"User: \" . $user->getName() . \"!\"",
                    "outer": "\"User: \" . $user->getName() . \"!\"",
                    "inner": "User: \" . $user->getName() . \"!",
                    "vars": [
                      {
                        "outer": " . $user->getName() . ",
                        "inner": "$user->getName()"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "User: {0}!"
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
fn concat_with_objects() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            // @prompt
            $message = "Items: " . [1, 2, 3] . "!";
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

    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            // @prompt
            $message = "Result: " . new stdClass() . "!";
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
