use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn join_method() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            prompt = "\n".join(["Hello", user, "!"])
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (0, 50),
                      span: SpanShape(
                        outer: (19, 50),
                        inner: (30, 48),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (31, 36),
                        ),
                        PromptContentTokenJoint(
                          type: "joint",
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (39, 43),
                          index: 0,
                        ),
                        PromptContentTokenJoint(
                          type: "joint",
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (46, 47),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (19, 23),
                        inner: (20, 22),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (39, 43),
                            inner: (39, 43),
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
                    "enclosure": "# @prompt\nprompt = \"\\n\".join([\"Hello\", user, \"!\"])",
                    "outer": "\"\\n\".join([\"Hello\", user, \"!\"])",
                    "inner": "\"Hello\", user, \"!\"",
                    "vars": [
                      {
                        "outer": "user",
                        "inner": "user"
                      }
                    ]
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello\\n{0}\\n!"
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

#[test]
fn array_simple() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            prompt = ["Hello ", user, "!"]
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (0, 40),
                      span: SpanShape(
                        outer: (19, 40),
                        inner: (20, 39),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (21, 27),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (30, 34),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (37, 38),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (30, 34),
                            inner: (30, 34),
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
                    "enclosure": "# @prompt\nprompt = [\"Hello \", user, \"!\"]",
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
                "##);
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello {0}!"
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
