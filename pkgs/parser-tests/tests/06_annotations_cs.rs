use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[ignore]
#[test]
fn multiple() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // Hello, world
            string hello = /* @prompt */ "asd";
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
fn multiline() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            /*
             Multi
             Line
             Block
            */
            string hello = /* @prompt */ "x";
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
fn multiline_nested() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            void Fn() {
                // Hello
                // @prompt
                // world
                string msg = "Hello";
            }
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
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
                    "enclosure": "// Hello\n    // @prompt\n    // world\n    string msg = \"Hello\";",
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
