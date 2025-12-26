use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn multiline() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
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
                      file: "prompts.py",
                      span: SpanShape(
                        outer: (32, 39),
                        inner: (33, 38),
                      ),
                      enclosure: (0, 39),
                      exp: "\"Hello\"",
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
        &ParseTestLang::py(indoc! {r#"
            def fn():
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
                      file: "prompts.py",
                      span: SpanShape(
                        outer: (58, 65),
                        inner: (59, 64),
                      ),
                      enclosure: (14, 65),
                      exp: "\"Hello\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (14, 21),
                              inner: (15, 21),
                            ),
                            SpanShape(
                              outer: (26, 35),
                              inner: (27, 35),
                            ),
                            SpanShape(
                              outer: (40, 47),
                              inner: (41, 47),
                            ),
                          ],
                          exp: "# Hello\n    # @prompt\n    # world",
                        ),
                      ],
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
