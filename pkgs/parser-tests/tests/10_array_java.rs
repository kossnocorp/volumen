use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn join_method() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String prompt = String.join("\n", new String[]{"Hello", user, "!"});
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
                      enclosure: (0, 79),
                      span: SpanShape(
                        outer: (27, 78),
                        inner: (58, 76),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (59, 64),
                        ),
                        PromptContentTokenJoint(
                          type: "joint",
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (67, 71),
                          index: 0,
                        ),
                        PromptContentTokenJoint(
                          type: "joint",
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (74, 75),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (39, 43),
                        inner: (40, 42),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (67, 71),
                            inner: (67, 71),
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
                    "enclosure": "// @prompt\nString prompt = String.join(\"\\n\", new String[]{\"Hello\", user, \"!\"});",
                    "outer": "String.join(\"\\n\", new String[]{\"Hello\", user, \"!\"})",
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
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String[] prompt = new String[]{"Hello ", user, "!"};
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
                      enclosure: (0, 63),
                      span: SpanShape(
                        outer: (29, 62),
                        inner: (42, 61),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (43, 49),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (52, 56),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (59, 60),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
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
                    "enclosure": "// @prompt\nString[] prompt = new String[]{\"Hello \", user, \"!\"};",
                    "outer": "new String[]{\"Hello \", user, \"!\"}",
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
