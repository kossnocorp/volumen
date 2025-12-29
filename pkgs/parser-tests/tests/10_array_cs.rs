use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn join_method() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            string prompt = String.Join("\n", new[] {"Hello", user, "!"});
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
                      enclosure: (0, 73),
                      span: SpanShape(
                        outer: (27, 72),
                        inner: (52, 70),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (53, 58),
                        ),
                        PromptContentTokenJoint(
                          type: "joint",
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (61, 65),
                          index: 0,
                        ),
                        PromptContentTokenJoint(
                          type: "joint",
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (68, 69),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (39, 43),
                        inner: (40, 42),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (61, 65),
                            inner: (61, 65),
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
                    "enclosure": "// @prompt\nstring prompt = String.Join(\"\\n\", new[] {\"Hello\", user, \"!\"});",
                    "outer": "String.Join(\"\\n\", new[] {\"Hello\", user, \"!\"})",
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
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            string[] prompt = new[] {"Hello ", user, "!"};
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
                      enclosure: (0, 57),
                      span: SpanShape(
                        outer: (29, 56),
                        inner: (36, 55),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (37, 43),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (46, 50),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (53, 54),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (46, 50),
                            inner: (46, 50),
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
                    "enclosure": "// @prompt\nstring[] prompt = new[] {\"Hello \", user, \"!\"};",
                    "outer": "new[] {\"Hello \", user, \"!\"}",
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
