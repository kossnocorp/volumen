use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn concat() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const greeting = "Hello, " + name + "!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 51),
                      span: SpanShape(
                        outer: (28, 50),
                        inner: (29, 49),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (29, 36),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (40, 44),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (48, 49),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (37, 47),
                            inner: (40, 44),
                          ),
                        ),
                      ],
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
                    "enclosure": "// @prompt\nconst greeting = \"Hello, \" + name + \"!\";",
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
fn concat_with_primitives() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const message = "Count: " + 42 + ", Active: " + true;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 64),
                      span: SpanShape(
                        outer: (27, 63),
                        inner: (28, 63),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (28, 35),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (39, 41),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (45, 55),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (59, 63),
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
                    "enclosure": "// @prompt\nconst message = \"Count: \" + 42 + \", Active: \" + true;",
                    "outer": "\"Count: \" + 42 + \", Active: \" + true",
                    "inner": "Count: \" + 42 + \", Active: \" + true",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Count: 42, Active: true"
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
fn concat_with_objects() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const message = "Items: " + [1, 2, 3] + "!";
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
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @"[]");
            }),
            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @"[]");
            }),
        },
    );

    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const message = "Result: " + {key: "value"} + "!";
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
fn concat_with_function_calls() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const message = "Hello " + format(name) + " world";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 62),
                      span: SpanShape(
                        outer: (27, 61),
                        inner: (28, 60),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (28, 34),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (38, 50),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (54, 60),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (35, 53),
                            inner: (38, 50),
                          ),
                        ),
                      ],
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
                    "enclosure": "// @prompt\nconst message = \"Hello \" + format(name) + \" world\";",
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
