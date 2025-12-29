use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn format_method() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            formatted = "Hello %s" % name
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (0, 39),
                      span: SpanShape(
                        outer: (22, 39),
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
                            outer: (35, 39),
                            inner: (35, 39),
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
                    "enclosure": "# @prompt\nformatted = \"Hello %s\" % name",
                    "outer": "\"Hello %s\" % name",
                    "inner": "Hello %s",
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
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello {0}"
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
