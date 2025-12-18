use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
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
                      span: SpanShape(
                        outer: Span(
                          start: 44,
                          end: 49,
                        ),
                        inner: Span(
                          start: 45,
                          end: 48,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 50,
                      ),
                      exp: "\"asd\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 15,
                          ),
                          exp: "// Hello, world",
                        ),
                        PromptAnnotation(
                          span: Span(
                            start: 30,
                            end: 43,
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
                    "// Hello, world",
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
                      span: SpanShape(
                        outer: Span(
                          start: 54,
                          end: 57,
                        ),
                        inner: Span(
                          start: 55,
                          end: 56,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 58,
                      ),
                      exp: "`x`",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 25,
                          ),
                          exp: "/*\n Multi\n Line\n Block\n*/",
                        ),
                        PromptAnnotation(
                          span: Span(
                            start: 40,
                            end: 53,
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
                    "/*\n Multi\n Line\n Block\n*/",
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
                      span: SpanShape(
                        outer: Span(
                          start: 73,
                          end: 80,
                        ),
                        inner: Span(
                          start: 74,
                          end: 79,
                        ),
                      ),
                      enclosure: Span(
                        start: 20,
                        end: 81,
                      ),
                      exp: "\"Hello\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 20,
                            end: 56,
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
                    "// Hello\n    // @prompt\n    // world"
                  ]
                ]
                "#);
            }),
        },
    );
}
