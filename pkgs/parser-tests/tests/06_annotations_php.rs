use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

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
            function fn() {
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
                      span: SpanShape(
                        outer: Span(
                          start: 74,
                          end: 81,
                        ),
                        inner: Span(
                          start: 75,
                          end: 80,
                        ),
                      ),
                      enclosure: Span(
                        start: 26,
                        end: 81,
                      ),
                      exp: "\"Hello\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 26,
                            end: 62,
                          ),
                          exp: "// Hello\n    // @prompt\n    // world",
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
                    "// Hello\n    // @prompt\n    // world"
                  ]
                ]
                "#);
            }),
        },
    );
}
