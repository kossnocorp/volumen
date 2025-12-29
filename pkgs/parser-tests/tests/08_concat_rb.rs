use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn concat() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            greeting = "Hello, " + name + "!"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (0, 43),
                      span: SpanShape(
                        outer: (21, 43),
                        inner: (22, 42),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (22, 29),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (33, 37),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (41, 42),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (30, 40),
                            inner: (33, 37),
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
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\ngreeting = \"Hello, \" + name + \"!\"",
                    "outer": "\"Hello, \" + name + \"!\"",
                    "inner": "Hello, \" + name + \"!",
                    "vars": [
                      {
                        "outer": " + name + ",
                        "inner": "name"
                      }
                    ]
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello, {0}!"
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
fn concat_with_function_calls() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            greeting = "Hello " + format(name) + " world"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (0, 55),
                      span: SpanShape(
                        outer: (21, 55),
                        inner: (22, 54),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (22, 28),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (32, 44),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (48, 54),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (29, 47),
                            inner: (32, 44),
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
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\ngreeting = \"Hello \" + format(name) + \" world\"",
                    "outer": "\"Hello \" + format(name) + \" world\"",
                    "inner": "Hello \" + format(name) + \" world",
                    "vars": [
                      {
                        "outer": " + format(name) + ",
                        "inner": "format(name)"
                      }
                    ]
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello {0} world"
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

    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            message = "User: " + user.get_name() + "!"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (0, 52),
                      span: SpanShape(
                        outer: (20, 52),
                        inner: (21, 51),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (21, 27),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (31, 46),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (50, 51),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (28, 49),
                            inner: (31, 46),
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
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\nmessage = \"User: \" + user.get_name() + \"!\"",
                    "outer": "\"User: \" + user.get_name() + \"!\"",
                    "inner": "User: \" + user.get_name() + \"!",
                    "vars": [
                      {
                        "outer": " + user.get_name() + ",
                        "inner": "user.get_name()"
                      }
                    ]
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "User: {0}!"
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
fn concat_with_objects() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            message = "Items: " + [1, 2, 3] + "!"
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
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            message = "Result: " + {key: "value"} + "!"
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
