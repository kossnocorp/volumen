use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn concat() {
    ParseTest::test(
        &ParseTestLang::go(indoc! {r#"
            // @prompt
            greeting := "Hello, " + name + "!"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.go",
                      enclosure: (0, 45),
                      span: SpanShape(
                        outer: (23, 45),
                        inner: (24, 44),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (24, 31),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (35, 39),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (43, 44),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (32, 42),
                            inner: (35, 39),
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
                    "enclosure": "// @prompt\ngreeting := \"Hello, \" + name + \"!\"",
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
fn concat_with_objects() {
    ParseTest::test(
        &ParseTestLang::go(indoc! {r#"
            // @prompt
            message := "Items: " + []int{1, 2, 3} + "!"
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
        &ParseTestLang::go(indoc! {r#"
            // @prompt
            message := "Result: " + struct{}{} + "!"
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
        &ParseTestLang::go(indoc! {r#"
            // @prompt
            message := "Hello " + format(name) + " world"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.go",
                      enclosure: (0, 56),
                      span: SpanShape(
                        outer: (22, 56),
                        inner: (23, 55),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (23, 29),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (33, 45),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (49, 55),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (30, 48),
                            inner: (33, 45),
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
                    "enclosure": "// @prompt\nmessage := \"Hello \" + format(name) + \" world\"",
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
