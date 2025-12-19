use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn single_var() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            user_prompt = "Welcome, #{user}!"
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
                          end: 33,
                        ),
                        inner: Span(
                          start: 15,
                          end: 32,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 33,
                      ),
                      exp: "\"Welcome, #{user}!\"",
                      vars: [
                        PromptVar(
                          exp: "#{user}",
                          span: SpanShape(
                            outer: Span(
                              start: 24,
                              end: 31,
                            ),
                            inner: Span(
                              start: 26,
                              end: 30,
                            ),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "user_prompt = \"Welcome, #{user}!\"",
                    "outer": "\"Welcome, #{user}!\"",
                    "inner": "Welcome, #{user}!",
                    "vars": [
                      {
                        "outer": "#{user}",
                        "inner": "user"
                      }
                    ]
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Welcome, {0}!"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r"
                [
                  []
                ]
                ");
            }),
        },
    );
}

#[test]
fn multiple_vars() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            user_prompt = "Hello, #{name}! How is the weather today in #{city}?"
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
                          end: 68,
                        ),
                        inner: Span(
                          start: 15,
                          end: 67,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 68,
                      ),
                      exp: "\"Hello, #{name}! How is the weather today in #{city}?\"",
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
                        PromptVar(
                          exp: "#{city}",
                          span: SpanShape(
                            outer: Span(
                              start: 59,
                              end: 66,
                            ),
                            inner: Span(
                              start: 61,
                              end: 65,
                            ),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "user_prompt = \"Hello, #{name}! How is the weather today in #{city}?\"",
                    "outer": "\"Hello, #{name}! How is the weather today in #{city}?\"",
                    "inner": "Hello, #{name}! How is the weather today in #{city}?",
                    "vars": [
                      {
                        "outer": "#{name}",
                        "inner": "name"
                      },
                      {
                        "outer": "#{city}",
                        "inner": "city"
                      }
                    ]
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, {0}! How is the weather today in {1}?"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r"
                [
                  []
                ]
                ");
            }),
        },
    );
}

#[test]
fn exp() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            user_prompt = "Hello, #{user.name}! How is the weather today in #{user.location.city}?"
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
                          end: 87,
                        ),
                        inner: Span(
                          start: 15,
                          end: 86,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 87,
                      ),
                      exp: "\"Hello, #{user.name}! How is the weather today in #{user.location.city}?\"",
                      vars: [
                        PromptVar(
                          exp: "#{user.name}",
                          span: SpanShape(
                            outer: Span(
                              start: 22,
                              end: 34,
                            ),
                            inner: Span(
                              start: 24,
                              end: 33,
                            ),
                          ),
                        ),
                        PromptVar(
                          exp: "#{user.location.city}",
                          span: SpanShape(
                            outer: Span(
                              start: 64,
                              end: 85,
                            ),
                            inner: Span(
                              start: 66,
                              end: 84,
                            ),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "user_prompt = \"Hello, #{user.name}! How is the weather today in #{user.location.city}?\"",
                    "outer": "\"Hello, #{user.name}! How is the weather today in #{user.location.city}?\"",
                    "inner": "Hello, #{user.name}! How is the weather today in #{user.location.city}?",
                    "vars": [
                      {
                        "outer": "#{user.name}",
                        "inner": "user.name"
                      },
                      {
                        "outer": "#{user.location.city}",
                        "inner": "user.location.city"
                      }
                    ]
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, {0}! How is the weather today in {1}?"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r"
                [
                  []
                ]
                ");
            }),
        },
    );
}

#[test]
fn exp_complex() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            user_prompt = "This item is #{price > 100 ? 'expensive' : 'cheap'}..."
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
                          end: 70,
                        ),
                        inner: Span(
                          start: 15,
                          end: 69,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 70,
                      ),
                      exp: "\"This item is #{price > 100 ? \'expensive\' : \'cheap\'}...\"",
                      vars: [
                        PromptVar(
                          exp: "#{price > 100 ? \'expensive\' : \'cheap\'}",
                          span: SpanShape(
                            outer: Span(
                              start: 28,
                              end: 66,
                            ),
                            inner: Span(
                              start: 30,
                              end: 65,
                            ),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "user_prompt = \"This item is #{price > 100 ? 'expensive' : 'cheap'}...\"",
                    "outer": "\"This item is #{price > 100 ? 'expensive' : 'cheap'}...\"",
                    "inner": "This item is #{price > 100 ? 'expensive' : 'cheap'}...",
                    "vars": [
                      {
                        "outer": "#{price > 100 ? 'expensive' : 'cheap'}",
                        "inner": "price > 100 ? 'expensive' : 'cheap'"
                      }
                    ]
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "This item is {0}..."
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r"
                [
                  []
                ]
                ");
            }),
        },
    );
}
