use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn invalid() {
    ParseTest::test(
        &ParseTestLang::go(indoc! {r#"
            invalid := "unclosed string
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultError(
                  state: "error",
                  error: "<error>",
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
fn raw_string() {
    ParseTest::test(
        &ParseTestLang::go(indoc! {r#"
            // @prompt
            system := `You are a helpful assistant.
            You can help with various tasks.`
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.go",
                      enclosure: (0, 84),
                      span: SpanShape(
                        outer: (21, 84),
                        inner: (22, 83),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (22, 83),
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
                    "enclosure": "// @prompt\nsystem := `You are a helpful assistant.\nYou can help with various tasks.`",
                    "outer": "`You are a helpful assistant.\nYou can help with various tasks.`",
                    "inner": "You are a helpful assistant.\nYou can help with various tasks.",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "You are a helpful assistant.\nYou can help with various tasks."
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
