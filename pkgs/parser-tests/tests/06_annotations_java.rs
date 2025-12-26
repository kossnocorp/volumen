use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[ignore]
#[test]
fn multiple() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            // Hello, world
            String hello = /* @prompt */ "asd";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
                      span: SpanShape(
                        outer: Span(
                          start: 45,
                          end: 50,
                        ),
                        inner: Span(
                          start: 46,
                          end: 49,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 51,
                      ),
                      exp: "\"asd\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 31,
                            end: 44,
                          ),
                          exp: "/* @prompt */",
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
                    "enclosure": "// Hello, world\nString hello = /* @prompt */ \"asd\";",
                    "outer": "\"asd\"",
                    "inner": "asd",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "asd"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
                [
                  [
                    "/* @prompt */"
                  ]
                ]
                "#);
            }),
        },
    );
}

#[ignore]
#[test]
fn multiline() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            /*
             Multi
             Line
             Block
            */
            String hello = /* @prompt */ "x";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
                      span: SpanShape(
                        outer: Span(
                          start: 55,
                          end: 58,
                        ),
                        inner: Span(
                          start: 56,
                          end: 57,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 59,
                      ),
                      exp: "\"x\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 41,
                            end: 54,
                          ),
                          exp: "/* @prompt */",
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
                    "enclosure": "/*\n Multi\n Line\n Block\n*/\nString hello = /* @prompt */ \"x\";",
                    "outer": "\"x\"",
                    "inner": "x",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "x"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
                [
                  [
                    "/* @prompt */"
                  ]
                ]
                "#);
            }),
        },
    );
}

#[test]
fn multiline_nested() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            void fn() {
                // Hello
                // @prompt
                // world
                String msg = "Hello";
            }
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
                      enclosure: (16, 78),
                      span: SpanShape(
                        outer: (70, 77),
                        inner: (71, 76),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (71, 76),
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
                              outer: (16, 24),
                              inner: (18, 24),
                            ),
                            SpanShape(
                              outer: (29, 39),
                              inner: (31, 39),
                            ),
                            SpanShape(
                              outer: (44, 52),
                              inner: (46, 52),
                            ),
                          ],
                          exp: "// Hello\n    // @prompt\n    // world",
                        ),
                      ],
                      exp: "\"Hello\"",
                    ),
                  ],
                )
                "#);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "// Hello\n    // @prompt\n    // world\n    String msg = \"Hello\";",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
                [
                  [
                    [
                      {
                        "outer": "// Hello",
                        "inner": " Hello"
                      },
                      {
                        "outer": "// @prompt",
                        "inner": " @prompt"
                      },
                      {
                        "outer": "// world",
                        "inner": " world"
                      }
                    ]
                  ]
                ]
                "#);
            }),
        },
    );
}
