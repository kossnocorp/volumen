use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn format_method() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            formatted = "Hello {}".format(name)
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (0, 45),
                      span: SpanShape(
                        outer: (22, 45),
                        inner: (23, 31),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (23, 29),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (29, 31),
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
                            outer: (40, 44),
                            inner: (40, 44),
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
                    "enclosure": "# @prompt\nformatted = \"Hello {}\".format(name)",
                    "outer": "\"Hello {}\".format(name)",
                    "inner": "Hello {}",
                    "vars": [
                      {
                        "outer": "name",
                        "inner": "name"
                      }
                    ]
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello {0}"
                ]
                "#);
            }),
            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
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
