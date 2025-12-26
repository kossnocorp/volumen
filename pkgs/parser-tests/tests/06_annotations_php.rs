use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[ignore]
#[test]
fn multiple() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            // Hello, world
            $hello = /* @prompt */ "asd";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      span: SpanShape(
                        outer: Span(
                          start: 45,
                          end: 50,
                        ),
                        inner: Span(
                          start: 46,
                          end: 49,
                        ),
                      ),
                      enclosure: Span(
                        start: 6,
                        end: 50,
                      ),
                      exp: "\"asd\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 31,
                            end: 44,
                          ),
                          exp: "/* @prompt */",
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
                    "enclosure": "// Hello, world\n$hello = /* @prompt */ \"asd\"",
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
                    "/* @prompt */"
                  ]
                ]
                "#);
            }),
        },
    );
}

#[ignore]
#[test]
fn multiline() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            /*
             Multi
             Line
             Block
            */
            $hello = /* @prompt */ "x";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      span: SpanShape(
                        outer: Span(
                          start: 55,
                          end: 58,
                        ),
                        inner: Span(
                          start: 56,
                          end: 57,
                        ),
                      ),
                      enclosure: Span(
                        start: 6,
                        end: 58,
                      ),
                      exp: "\"x\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 41,
                            end: 54,
                          ),
                          exp: "/* @prompt */",
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
                    "enclosure": "/*\n Multi\n Line\n Block\n*/\n$hello = /* @prompt */ \"x\"",
                    "outer": "\"x\"",
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
                    "/* @prompt */"
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
        &ParseTestLang::php(indoc! {r#"
            <?php
            function foo() {
                // Hello
                // @prompt
                // world
                $msg = "Hello";
            }
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (27, 82),
                      span: SpanShape(
                        outer: (75, 82),
                        inner: (76, 81),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (76, 81),
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
                              outer: (27, 35),
                              inner: (29, 35),
                            ),
                            SpanShape(
                              outer: (40, 50),
                              inner: (42, 50),
                            ),
                            SpanShape(
                              outer: (55, 63),
                              inner: (57, 63),
                            ),
                          ],
                          exp: "// Hello\n    // @prompt\n    // world",
                        ),
                      ],
                      exp: "\"Hello\"",
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "// Hello\n    // @prompt\n    // world\n    $msg = \"Hello\"",
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
