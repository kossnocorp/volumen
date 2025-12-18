use indoc::indoc;
use insta::{assert_ron_snapshot, assert_json_snapshot};

mod utils;
use utils::*;

#[test]
fn single_var() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            greeting_prompt = f"Welcome, {user}!"
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
                          start: 18,
                          end: 37,
                        ),
                        inner: Span(
                          start: 20,
                          end: 36,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 37,
                      ),
                      exp: "f\"Welcome, {user}!\"",
                      vars: [
                        PromptVar(
                          exp: "{user}",
                          span: SpanShape(
                            outer: Span(
                              start: 29,
                              end: 35,
                            ),
                            inner: Span(
                              start: 30,
                              end: 34,
                            ),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "greeting_prompt = f\"Welcome, {user}!\"",
                    "outer": "f\"Welcome, {user}!\"",
                    "inner": "Welcome, {user}!",
                    "vars": [
                      {
                        "outer": "{user}",
                        "inner": "user"
                      }
                    ]
                  }
                ]
                "#);
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
        &ParseTestLang::py(indoc! {r#"
            user_prompt = f"Hello, {name}! How is the weather today in {city}?"
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
                          start: 14,
                          end: 67,
                        ),
                        inner: Span(
                          start: 16,
                          end: 66,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 67,
                      ),
                      exp: "f\"Hello, {name}! How is the weather today in {city}?\"",
                      vars: [
                        PromptVar(
                          exp: "{name}",
                          span: SpanShape(
                            outer: Span(
                              start: 23,
                              end: 29,
                            ),
                            inner: Span(
                              start: 24,
                              end: 28,
                            ),
                          ),
                        ),
                        PromptVar(
                          exp: "{city}",
                          span: SpanShape(
                            outer: Span(
                              start: 59,
                              end: 65,
                            ),
                            inner: Span(
                              start: 60,
                              end: 64,
                            ),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "user_prompt = f\"Hello, {name}! How is the weather today in {city}?\"",
                    "outer": "f\"Hello, {name}! How is the weather today in {city}?\"",
                    "inner": "Hello, {name}! How is the weather today in {city}?",
                    "vars": [
                      {
                        "outer": "{name}",
                        "inner": "name"
                      },
                      {
                        "outer": "{city}",
                        "inner": "city"
                      }
                    ]
                  }
                ]
                "#);
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
        &ParseTestLang::py(indoc! {r#"
            user_prompt = f"Hello, {user.name}! How is the weather today in {user.location.city}?"
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
                          start: 14,
                          end: 86,
                        ),
                        inner: Span(
                          start: 16,
                          end: 85,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 86,
                      ),
                      exp: "f\"Hello, {user.name}! How is the weather today in {user.location.city}?\"",
                      vars: [
                        PromptVar(
                          exp: "{user.name}",
                          span: SpanShape(
                            outer: Span(
                              start: 23,
                              end: 34,
                            ),
                            inner: Span(
                              start: 24,
                              end: 33,
                            ),
                          ),
                        ),
                        PromptVar(
                          exp: "{user.location.city}",
                          span: SpanShape(
                            outer: Span(
                              start: 64,
                              end: 84,
                            ),
                            inner: Span(
                              start: 65,
                              end: 83,
                            ),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "user_prompt = f\"Hello, {user.name}! How is the weather today in {user.location.city}?\"",
                    "outer": "f\"Hello, {user.name}! How is the weather today in {user.location.city}?\"",
                    "inner": "Hello, {user.name}! How is the weather today in {user.location.city}?",
                    "vars": [
                      {
                        "outer": "{user.name}",
                        "inner": "user.name"
                      },
                      {
                        "outer": "{user.location.city}",
                        "inner": "user.location.city"
                      }
                    ]
                  }
                ]
                "#);
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
        &ParseTestLang::py(indoc! {r#"
            user_prompt = f"This item is {('expensive' if price > 100 else 'cheap')}..."
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
                          start: 14,
                          end: 76,
                        ),
                        inner: Span(
                          start: 16,
                          end: 75,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 76,
                      ),
                      exp: "f\"This item is {(\'expensive\' if price > 100 else \'cheap\')}...\"",
                      vars: [
                        PromptVar(
                          exp: "{(\'expensive\' if price > 100 else \'cheap\')}",
                          span: SpanShape(
                            outer: Span(
                              start: 29,
                              end: 72,
                            ),
                            inner: Span(
                              start: 30,
                              end: 71,
                            ),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "user_prompt = f\"This item is {('expensive' if price > 100 else 'cheap')}...\"",
                    "outer": "f\"This item is {('expensive' if price > 100 else 'cheap')}...\"",
                    "inner": "This item is {('expensive' if price > 100 else 'cheap')}...",
                    "vars": [
                      {
                        "outer": "{('expensive' if price > 100 else 'cheap')}",
                        "inner": "('expensive' if price > 100 else 'cheap')"
                      }
                    ]
                  }
                ]
                "#);
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
