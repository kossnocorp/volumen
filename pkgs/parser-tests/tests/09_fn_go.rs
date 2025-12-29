use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn sprintf_fn() {
    ParseTest::test(
        &ParseTestLang::go(indoc! {r#"
            // @prompt
            formatted := fmt.Sprintf("Hello %s", name)
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.go",
                      enclosure: (0, 53),
                      span: SpanShape(
                        outer: (24, 53),
                        inner: (37, 45),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (37, 43),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (43, 45),
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
                            outer: (48, 52),
                            inner: (48, 52),
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
                    "enclosure": "// @prompt\nformatted := fmt.Sprintf(\"Hello %s\", name)",
                    "outer": "fmt.Sprintf(\"Hello %s\", name)",
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
