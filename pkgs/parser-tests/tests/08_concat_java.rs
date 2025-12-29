use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn concat() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String greeting = "Hello, " + name + "!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
                      enclosure: (0, 52),
                      span: SpanShape(
                        outer: (29, 51),
                        inner: (30, 50),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (30, 37),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (41, 45),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (49, 50),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (38, 48),
                            inner: (41, 45),
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
                    "enclosure": "// @prompt\nString greeting = \"Hello, \" + name + \"!\";",
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
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello, {0}!"
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
fn concat_with_primitives() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String message = "Count: " + 42 + ", Active: " + true;
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
                        outer: (28, 64),
                        inner: (29, 64),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (29, 36),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (40, 42),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (46, 56),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (60, 64),
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
                    "enclosure": "// @prompt\nString message = \"Count: \" + 42 + \", Active: \" + true;",
                    "outer": "\"Count: \" + 42 + \", Active: \" + true",
                    "inner": "Count: \" + 42 + \", Active: \" + true",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Count: 42, Active: true"
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
fn concat_with_objects() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String message = "Items: " + new int[] {1, 2, 3} + "!";
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
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String message = "Result: " + new Object() + "!";
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
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String message = "Hello " + format(name) + " world";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
                      enclosure: (0, 63),
                      span: SpanShape(
                        outer: (28, 62),
                        inner: (29, 61),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (29, 35),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (39, 51),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (55, 61),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (36, 54),
                            inner: (39, 51),
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
                    "enclosure": "// @prompt\nString message = \"Hello \" + format(name) + \" world\";",
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
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello {0} world"
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
