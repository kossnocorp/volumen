use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
#[ignore = "TODO: Fix annotation collection for inline @prompt with non-@prompt leading comments"]
fn multiple() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // Hello, world
            const hello = /* @prompt */ "asd";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (0, 50),
                      span: SpanShape(
                        outer: (44, 49),
                        inner: (45, 48),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (45, 48),
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
                              outer: (0, 15),
                              inner: (2, 15),
                            ),
                          ],
                          exp: "// Hello, world",
                        ),
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (30, 43),
                              inner: (32, 41),
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
                    "enclosure": "// Hello, world\nconst hello = /* @prompt */ \"asd\";",
                    "outer": "\"asd\"",
                    "inner": "asd",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "asd"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    [
                      {
                        "outer": "// Hello, world",
                        "inner": " Hello, world"
                      }
                    ],
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
#[ignore = "TODO: Fix annotation collection for inline @prompt with non-@prompt leading comments"]
fn multiline() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            /*
             Multi
             Line
             Block
            */
            const hello = /* @prompt */ `x`;
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
                        outer: (54, 57),
                        inner: (55, 56),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (55, 56),
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
                              outer: (0, 25),
                              inner: (2, 23),
                            ),
                          ],
                          exp: "/*\n Multi\n Line\n Block\n*/",
                        ),
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (40, 53),
                              inner: (42, 51),
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
                    "enclosure": "/*\n Multi\n Line\n Block\n*/\nconst hello = /* @prompt */ `x`;",
                    "outer": "`x`",
                    "inner": "x",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "x"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    [
                      {
                        "outer": "/*\n Multi\n Line\n Block\n*/",
                        "inner": "\n Multi\n Line\n Block\n"
                      }
                    ],
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
fn multiline_nested() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            function fn() {
                // Hello
                // @prompt
                // world
                const msg = "Hello";
            }
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.js",
                      enclosure: (20, 81),
                      span: SpanShape(
                        outer: (73, 80),
                        inner: (74, 79),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (74, 79),
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
                              outer: (20, 28),
                              inner: (22, 28),
                            ),
                            SpanShape(
                              outer: (33, 43),
                              inner: (35, 43),
                            ),
                            SpanShape(
                              outer: (48, 56),
                              inner: (50, 56),
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
                    "enclosure": "// Hello\n    // @prompt\n    // world\n    const msg = \"Hello\";",
                    "outer": "\"Hello\"",
                    "inner": "Hello",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    [
                      {
                        "outer": "// Hello",
                        "inner": " Hello"
                      },
                      {
                        "outer": "// @prompt",
                        "inner": " @prompt"
                      },
                      {
                        "outer": "// world",
                        "inner": " world"
                      }
                    ]
                  ]
                ]
                "#);
            }),
        },
    );
}
