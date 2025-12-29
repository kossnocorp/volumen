use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn sprintf_fn() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            // @prompt
            $formatted = sprintf("Hello %s", $name);
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (6, 56),
                      span: SpanShape(
                        outer: (30, 56),
                        inner: (39, 47),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (39, 45),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (45, 47),
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
                            outer: (50, 55),
                            inner: (50, 55),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (6, 16),
                              inner: (8, 16),
                            ),
                          ],
                        ),
                      ],
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "// @prompt\n$formatted = sprintf(\"Hello %s\", $name)",
                    "outer": "sprintf(\"Hello %s\", $name)",
                    "inner": "Hello %s",
                    "vars": [
                      {
                        "outer": "$name",
                        "inner": "$name"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello {0}"
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
