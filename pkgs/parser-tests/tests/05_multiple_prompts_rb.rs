use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn multiple() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            user_prompt = "Hello, #{name}!"
            # @prompt
            farewell = "Goodbye #{user.name}!"
            regular = "Not a prompt #{value}"
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
                          start: 14,
                          end: 31,
                        ),
                        inner: Span(
                          start: 15,
                          end: 30,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 31,
                      ),
                      exp: "\"Hello, #{name}!\"",
                      vars: [
                        PromptVar(
                          exp: "#{name}",
                          span: SpanShape(
                            outer: Span(
                              start: 22,
                              end: 29,
                            ),
                            inner: Span(
                              start: 24,
                              end: 28,
                            ),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: Span(
                          start: 53,
                          end: 76,
                        ),
                        inner: Span(
                          start: 54,
                          end: 75,
                        ),
                      ),
                      enclosure: Span(
                        start: 32,
                        end: 76,
                      ),
                      exp: "\"Goodbye #{user.name}!\"",
                      vars: [
                        PromptVar(
                          exp: "#{user.name}",
                          span: SpanShape(
                            outer: Span(
                              start: 62,
                              end: 74,
                            ),
                            inner: Span(
                              start: 64,
                              end: 73,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 32,
                            end: 41,
                          ),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r##"
                [
                  {
                    "enclosure": "user_prompt = \"Hello, #{name}!\"",
                    "outer": "\"Hello, #{name}!\"",
                    "inner": "Hello, #{name}!",
                    "vars": [
                      {
                        "outer": "#{name}",
                        "inner": "name"
                      }
                    ]
                  },
                  {
                    "enclosure": "# @prompt\nfarewell = \"Goodbye #{user.name}!\"",
                    "outer": "\"Goodbye #{user.name}!\"",
                    "inner": "Goodbye #{user.name}!",
                    "vars": [
                      {
                        "outer": "#{user.name}",
                        "inner": "user.name"
                      }
                    ]
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello, {0}!",
                  "Goodbye {0}!"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r##"
                [
                  [],
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}
