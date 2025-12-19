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
                      span: SpanShape(
                        outer: Span(
                          start: 32,
                          end: 39,
                        ),
                        inner: Span(
                          start: 33,
                          end: 38,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 39,
                      ),
                      exp: "\"Hello\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 25,
                          ),
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
                    "# Hello\n# @prompt\n# world"
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
                      span: SpanShape(
                        outer: Span(
                          start: 55,
                          end: 62,
                        ),
                        inner: Span(
                          start: 56,
                          end: 61,
                        ),
                      ),
                      enclosure: Span(
                        start: 11,
                        end: 62,
                      ),
                      exp: "\"Hello\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 11,
                            end: 44,
                          ),
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
                    "# Hello\n    # @prompt\n    # world"
                  ]
                ]
                "##);
            }),
        },
    );
}
