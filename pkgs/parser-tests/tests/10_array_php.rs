use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn implode_fn() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            /* @prompt */
            $prompt = implode("\n", ["Hello", $user, "!"]);
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (6, 66),
                      span: SpanShape(
                        outer: (30, 66),
                        inner: (45, 64),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (46, 51),
                        ),
                        PromptContentTokenJoint(
                          type: "joint",
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (54, 59),
                          index: 0,
                        ),
                        PromptContentTokenJoint(
                          type: "joint",
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (62, 63),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (38, 42),
                        inner: (39, 41),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (54, 59),
                            inner: (54, 59),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (6, 19),
                              inner: (8, 17),
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
                    "enclosure": "/* @prompt */\n$prompt = implode(\"\\n\", [\"Hello\", $user, \"!\"])",
                    "outer": "implode(\"\\n\", [\"Hello\", $user, \"!\"])",
                    "inner": "\"Hello\", $user, \"!\"",
                    "vars": [
                      {
                        "outer": "$user",
                        "inner": "$user"
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
                        "outer": "/* @prompt */",
                        "inner": " @prompt "
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
        &ParseTestLang::php(indoc! {r#"
            <?php
            /* @prompt */
            $prompt = ["Hello ", $user, "!"];
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (6, 52),
                      span: SpanShape(
                        outer: (30, 52),
                        inner: (31, 51),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (32, 38),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (41, 46),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (49, 50),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (41, 46),
                            inner: (41, 46),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (6, 19),
                              inner: (8, 17),
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
                    "enclosure": "/* @prompt */\n$prompt = [\"Hello \", $user, \"!\"]",
                    "outer": "[\"Hello \", $user, \"!\"]",
                    "inner": "\"Hello \", $user, \"!\"",
                    "vars": [
                      {
                        "outer": "$user",
                        "inner": "$user"
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
                        "outer": "/* @prompt */",
                        "inner": " @prompt "
                      }
                    ]
                  ]
                ]
                "#);
            }),
        },
    );
}
