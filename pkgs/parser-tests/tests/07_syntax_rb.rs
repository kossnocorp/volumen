use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn invalid() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            x = "unclosed
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultError(
                  state: "error",
                  error: "<error>",
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @"[]");
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @"[]");
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @"[]");
            }),
        },
    );
}

#[ignore]
#[test]
fn heredoc() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            system = <<~TEXT
              You are a helpful assistant.
              You will answer the user's questions to the best of your ability.
              If you don't know the answer, just say that you don't know, don't try to make it up.
            TEXT
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
                          start: 19,
                          end: 26,
                        ),
                        inner: Span(
                          start: 19,
                          end: 26,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 26,
                      ),
                      exp: "<<~TEXT",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
                          exp: "# @prompt",
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
                    "enclosure": "# @prompt\nsystem = <<~TEXT",
                    "outer": "<<~TEXT",
                    "inner": "<<~TEXT",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "<<~TEXT"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[ignore]
#[test]
fn heredoc_interpolated() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            user = <<~TEXT
              Hello, #{name}!
              How is the weather today in #{city}?
            TEXT
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
                          start: 17,
                          end: 24,
                        ),
                        inner: Span(
                          start: 17,
                          end: 24,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 24,
                      ),
                      exp: "<<~TEXT",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
                          exp: "# @prompt",
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
                    "enclosure": "# @prompt\nuser = <<~TEXT",
                    "outer": "<<~TEXT",
                    "inner": "<<~TEXT",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "<<~TEXT"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}
