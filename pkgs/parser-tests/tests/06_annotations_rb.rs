use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn multiline() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # Hello
            # @prompt
            # world
            msg = "Hello"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (0, 39),
                      span: SpanShape(
                        outer: (32, 39),
                        inner: (33, 38),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (33, 38),
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
                              outer: (0, 7),
                              inner: (1, 7),
                            ),
                            SpanShape(
                              outer: (8, 17),
                              inner: (9, 17),
                            ),
                            SpanShape(
                              outer: (18, 25),
                              inner: (19, 25),
                            ),
                          ],
                          exp: "# Hello\n# @prompt\n# world",
                        ),
                      ],
                      exp: "\"Hello\"",
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# Hello\n# @prompt\n# world\nmsg = \"Hello\"",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    [
                      {
                        "outer": "# Hello",
                        "inner": " Hello"
                      },
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      },
                      {
                        "outer": "# world",
                        "inner": " world"
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
fn multiline_nested() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            def fn
                # Hello
                # @prompt
                # world
                msg = "Hello"
            end
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (11, 62),
                      span: SpanShape(
                        outer: (55, 62),
                        inner: (56, 61),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (56, 61),
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
                              outer: (11, 18),
                              inner: (12, 18),
                            ),
                            SpanShape(
                              outer: (23, 32),
                              inner: (24, 32),
                            ),
                            SpanShape(
                              outer: (37, 44),
                              inner: (38, 44),
                            ),
                          ],
                          exp: "# Hello\n    # @prompt\n    # world",
                        ),
                      ],
                      exp: "\"Hello\"",
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# Hello\n    # @prompt\n    # world\n    msg = \"Hello\"",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    [
                      {
                        "outer": "# Hello",
                        "inner": " Hello"
                      },
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      },
                      {
                        "outer": "# world",
                        "inner": " world"
                      }
                    ]
                  ]
                ]
                "##);
            }),
        },
    );
}
