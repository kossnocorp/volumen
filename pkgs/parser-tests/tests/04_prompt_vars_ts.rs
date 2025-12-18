use indoc::indoc;
use insta::{assert_ron_snapshot, assert_json_snapshot};

mod utils;
use utils::*;

#[test]
fn single_var() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            const userPrompt = `Welcome, ${user}!`;
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
                          start: 19,
                          end: 38,
                        ),
                        inner: Span(
                          start: 20,
                          end: 37,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 39,
                      ),
                      exp: "`Welcome, ${user}!`",
                      vars: [
                        PromptVar(
                          exp: "${user}",
                          span: SpanShape(
                            outer: Span(
                              start: 29,
                              end: 36,
                            ),
                            inner: Span(
                              start: 31,
                              end: 35,
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
                    "enclosure": "const userPrompt = `Welcome, ${user}!`;",
                    "outer": "`Welcome, ${user}!`",
                    "inner": "Welcome, ${user}!",
                    "vars": [
                      {
                        "outer": "${user}",
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
        &ParseTestLang::ts(indoc! {r#"
            const userPrompt = `Hello, ${name}! How is the weather today in ${city}?`;
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
                          start: 19,
                          end: 73,
                        ),
                        inner: Span(
                          start: 20,
                          end: 72,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 74,
                      ),
                      exp: "`Hello, ${name}! How is the weather today in ${city}?`",
                      vars: [
                        PromptVar(
                          exp: "${name}",
                          span: SpanShape(
                            outer: Span(
                              start: 27,
                              end: 34,
                            ),
                            inner: Span(
                              start: 29,
                              end: 33,
                            ),
                          ),
                        ),
                        PromptVar(
                          exp: "${city}",
                          span: SpanShape(
                            outer: Span(
                              start: 64,
                              end: 71,
                            ),
                            inner: Span(
                              start: 66,
                              end: 70,
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
                    "enclosure": "const userPrompt = `Hello, ${name}! How is the weather today in ${city}?`;",
                    "outer": "`Hello, ${name}! How is the weather today in ${city}?`",
                    "inner": "Hello, ${name}! How is the weather today in ${city}?",
                    "vars": [
                      {
                        "outer": "${name}",
                        "inner": "name"
                      },
                      {
                        "outer": "${city}",
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
        &ParseTestLang::ts(indoc! {r#"
            const userPrompt = `Hello, ${user.name}! How is the weather today in ${user.location.city}?`;
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
                          start: 19,
                          end: 92,
                        ),
                        inner: Span(
                          start: 20,
                          end: 91,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 93,
                      ),
                      exp: "`Hello, ${user.name}! How is the weather today in ${user.location.city}?`",
                      vars: [
                        PromptVar(
                          exp: "${user.name}",
                          span: SpanShape(
                            outer: Span(
                              start: 27,
                              end: 39,
                            ),
                            inner: Span(
                              start: 29,
                              end: 38,
                            ),
                          ),
                        ),
                        PromptVar(
                          exp: "${user.location.city}",
                          span: SpanShape(
                            outer: Span(
                              start: 69,
                              end: 90,
                            ),
                            inner: Span(
                              start: 71,
                              end: 89,
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
                    "enclosure": "const userPrompt = `Hello, ${user.name}! How is the weather today in ${user.location.city}?`;",
                    "outer": "`Hello, ${user.name}! How is the weather today in ${user.location.city}?`",
                    "inner": "Hello, ${user.name}! How is the weather today in ${user.location.city}?",
                    "vars": [
                      {
                        "outer": "${user.name}",
                        "inner": "user.name"
                      },
                      {
                        "outer": "${user.location.city}",
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
        &ParseTestLang::ts(indoc! {r#"
            const userPrompt = `This item is ${price > 100 ? "expensive" : "cheap"}...`;
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
                          start: 19,
                          end: 75,
                        ),
                        inner: Span(
                          start: 20,
                          end: 74,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 76,
                      ),
                      exp: "`This item is ${price > 100 ? \"expensive\" : \"cheap\"}...`",
                      vars: [
                        PromptVar(
                          exp: "${price > 100 ? \"expensive\" : \"cheap\"}",
                          span: SpanShape(
                            outer: Span(
                              start: 33,
                              end: 71,
                            ),
                            inner: Span(
                              start: 35,
                              end: 70,
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
                    "enclosure": "const userPrompt = `This item is ${price > 100 ? \"expensive\" : \"cheap\"}...`;",
                    "outer": "`This item is ${price > 100 ? \"expensive\" : \"cheap\"}...`",
                    "inner": "This item is ${price > 100 ? \"expensive\" : \"cheap\"}...",
                    "vars": [
                      {
                        "outer": "${price > 100 ? \"expensive\" : \"cheap\"}",
                        "inner": "price > 100 ? \"expensive\" : \"cheap\""
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
