use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn join_method() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const prompt = ["Hello", user, "!"].join("\n");
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 58),
                      span: SpanShape(
                        outer: (26, 57),
                        inner: (27, 45),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (28, 33),
                        ),
                        PromptContentTokenJoint(
                          type: "joint",
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (36, 40),
                          index: 0,
                        ),
                        PromptContentTokenJoint(
                          type: "joint",
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (43, 44),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (52, 56),
                        inner: (53, 55),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (36, 40),
                            inner: (36, 40),
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
                    "enclosure": "// @prompt\nconst prompt = [\"Hello\", user, \"!\"].join(\"\\n\");",
                    "outer": "[\"Hello\", user, \"!\"].join(\"\\n\")",
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
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello\\n{0}\\n!"
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

#[test]
fn array_simple() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const prompt = ["Hello ", user, "!"];
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 48),
                      span: SpanShape(
                        outer: (26, 47),
                        inner: (27, 46),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (28, 34),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (37, 41),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (44, 45),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (37, 41),
                            inner: (37, 41),
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
                    "enclosure": "// @prompt\nconst prompt = [\"Hello \", user, \"!\"];",
                    "outer": "[\"Hello \", user, \"!\"]",
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
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello {0}!"
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
