use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn string_format() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String formatted = String.format("Hello %s", name);
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
                      enclosure: (0, 62),
                      span: SpanShape(
                        outer: (30, 61),
                        inner: (45, 53),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (45, 51),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (51, 53),
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
                            outer: (56, 60),
                            inner: (56, 60),
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
                    "enclosure": "// @prompt\nString formatted = String.format(\"Hello %s\", name);",
                    "outer": "String.format(\"Hello %s\", name)",
                    "inner": "Hello %s",
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
