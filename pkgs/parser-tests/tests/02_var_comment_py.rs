use indoc::indoc;
use insta::{assert_ron_snapshot, assert_snapshot, assert_toml_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        None,
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            system = "You are a helpful assistant."
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
                        outer: Span(
                          start: 19,
                          end: 49,
                        ),
                        inner: Span(
                          start: 20,
                          end: 48,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 49,
                      ),
                      exp: "\"You are a helpful assistant.\"",
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
                assert_toml_snapshot!(prompt_source_cuts, @r#"
                [[]]
                enclosure = '''
                # @prompt
                system = "You are a helpful assistant."'''
                outer = '"You are a helpful assistant."'
                inner = 'You are a helpful assistant.'
                vars = []
                "#);
            }),

            interpolate: None,
        },
    );
}

#[test]
fn assigned() {
    ParseTest::test(
        None,
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            assigned : str
            assigned = f"Assigned {value}";
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
                        outer: Span(
                          start: 36,
                          end: 55,
                        ),
                        inner: Span(
                          start: 38,
                          end: 54,
                        ),
                      ),
                      enclosure: Span(
                        start: 25,
                        end: 55,
                      ),
                      exp: "f\"Assigned {value}\"",
                      vars: [
                        PromptVar(
                          exp: "{value}",
                          span: SpanShape(
                            outer: Span(
                              start: 47,
                              end: 54,
                            ),
                            inner: Span(
                              start: 48,
                              end: 53,
                            ),
                          ),
                        ),
                      ],
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
                assert_toml_snapshot!(prompt_source_cuts, @r#"
                [[]]
                enclosure = 'assigned = f"Assigned {value}"'
                outer = 'f"Assigned {value}"'
                inner = 'Assigned {value}'

                [[vars]]
                outer = '{value}'
                inner = 'value'
                "#);
            }),

            interpolate: Some(Box::new(|interpolated| {
                assert_toml_snapshot!(interpolated, @"['Assigned {0}']");
            })),
        },
    );
}

#[test]
fn reassigned() {
    ParseTest::test(
        None,
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            reassigned : Union[str, int] = 123
            reassigned = 456
            reassigned = f"Reassigned {value}";
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
                        outer: Span(
                          start: 75,
                          end: 96,
                        ),
                        inner: Span(
                          start: 77,
                          end: 95,
                        ),
                      ),
                      enclosure: Span(
                        start: 62,
                        end: 96,
                      ),
                      exp: "f\"Reassigned {value}\"",
                      vars: [
                        PromptVar(
                          exp: "{value}",
                          span: SpanShape(
                            outer: Span(
                              start: 88,
                              end: 95,
                            ),
                            inner: Span(
                              start: 89,
                              end: 94,
                            ),
                          ),
                        ),
                      ],
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
                assert_toml_snapshot!(prompt_source_cuts, @r#"
                [[]]
                enclosure = 'reassigned = f"Reassigned {value}"'
                outer = 'f"Reassigned {value}"'
                inner = 'Reassigned {value}'

                [[vars]]
                outer = '{value}'
                inner = 'value'
                "#);
            }),

            interpolate: Some(Box::new(|interpolated| {
                assert_toml_snapshot!(interpolated, @"['Reassigned {0}']");
            })),
        },
    );
}

#[test]
fn not_exact() {
    ParseTest::test(
        None,
        &ParseTestLang::py(indoc! {r#"
            # @prompting
            hello = "Hello, world!"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @"[]");
            }),

            interpolate: None,
        },
    );
}

#[test]
fn mixed() {
    ParseTest::test(
        None,
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            number = 42
            hello = "Hello, world!"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @"[]");
            }),

            interpolate: None,
        },
    );
}

#[test]
fn mixed_nested() {
    ParseTest::test(
        None,
        &ParseTestLang::py(indoc! {r#"
            class Hello:
                def world(self):
                    # @prompt
                    hello = 42
                    # @prompt
                    hi = 42
                    hi = "Hi!"

            hello = "Hello, world!"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 118,
                          end: 123,
                        ),
                        inner: Span(
                          start: 119,
                          end: 122,
                        ),
                      ),
                      enclosure: Span(
                        start: 113,
                        end: 123,
                      ),
                      exp: "\"Hi!\"",
                      vars: [],
                      annotations: [],
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @r#"
                [[]]
                enclosure = 'hi = "Hi!"'
                outer = '"Hi!"'
                inner = 'Hi!'
                vars = []
                "#);
            }),

            interpolate: Some(Box::new(|interpolated| {
                assert_toml_snapshot!(interpolated, @"['Hi!']");
            })),
        },
    );
}

#[test]
fn mixed_none() {
    ParseTest::test(
        None,
        &ParseTestLang::py(indoc! {r#"
            regular_template = f"This is not a {value}"
            normal_string = "This is not special"
            regular = f"Regular template with {variable}"
            message = "Just a message"
            # @prompt
            number = 1
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_toml_snapshot!(prompt_source_cuts, @"[]");
            }),

            interpolate: None,
        },
    );
}
