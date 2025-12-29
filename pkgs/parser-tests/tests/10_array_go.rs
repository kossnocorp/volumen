use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn join_method() {
    ParseTest::test(
        &ParseTestLang::go(indoc! {r#"
            // @prompt
            prompt := strings.Join([]string{"Hello", user, "!"}, "\n")
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.go",
                      enclosure: (0, 69),
                      span: SpanShape(
                        outer: (21, 69),
                        inner: (43, 61),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (44, 49),
                        ),
                        PromptContentTokenJoint(
                          type: "joint",
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (52, 56),
                          index: 0,
                        ),
                        PromptContentTokenJoint(
                          type: "joint",
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (59, 60),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (64, 68),
                        inner: (65, 67),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (52, 56),
                            inner: (52, 56),
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
                    "enclosure": "// @prompt\nprompt := strings.Join([]string{\"Hello\", user, \"!\"}, \"\\n\")",
                    "outer": "strings.Join([]string{\"Hello\", user, \"!\"}, \"\\n\")",
                    "inner": "\"Hello\", user, \"!\"",
                    "vars": [
                      {
                        "outer": "user",
                        "inner": "user"
                      }
                    ]
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello\\n{0}\\n!"
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
fn array_simple() {
    ParseTest::test(
        &ParseTestLang::go(indoc! {r#"
            // @prompt
            prompt := []string{"Hello ", user, "!"}
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.go",
                      enclosure: (0, 50),
                      span: SpanShape(
                        outer: (21, 50),
                        inner: (30, 49),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (31, 37),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (40, 44),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (47, 48),
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
                    "enclosure": "// @prompt\nprompt := []string{\"Hello \", user, \"!\"}",
                    "outer": "[]string{\"Hello \", user, \"!\"}",
                    "inner": "\"Hello \", user, \"!\"",
                    "vars": [
                      {
                        "outer": "user",
                        "inner": "user"
                      }
                    ]
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello {0}!"
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
