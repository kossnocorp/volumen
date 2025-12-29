use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn string_format() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            string formatted = String.Format("Hello {0}", name);
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
                      enclosure: (0, 63),
                      span: SpanShape(
                        outer: (30, 62),
                        inner: (45, 54),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (45, 51),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (51, 54),
                          index: 0,
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (57, 61),
                            inner: (57, 61),
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
                    "enclosure": "// @prompt\nstring formatted = String.Format(\"Hello {0}\", name);",
                    "outer": "String.Format(\"Hello {0}\", name)",
                    "inner": "Hello {0}",
                    "vars": [
                      {
                        "outer": "name",
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
                  "Hello {0}"
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
