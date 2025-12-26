use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn single_var() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            $user_prompt = "Welcome, {$user}!";
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
                          start: 21,
                          end: 40,
                        ),
                        inner: Span(
                          start: 22,
                          end: 39,
                        ),
                      ),
                      enclosure: Span(
                        start: 6,
                        end: 40,
                      ),
                      exp: "\"Welcome, {$user}!\"",
                      vars: [
                        PromptVar(
                          exp: "{$user}",
                          span: SpanShape(
                            outer: Span(
                              start: 31,
                              end: 38,
                            ),
                            inner: Span(
                              start: 32,
                              end: 37,
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
                    "enclosure": "$user_prompt = \"Welcome, {$user}!\"",
                    "outer": "\"Welcome, {$user}!\"",
                    "inner": "Welcome, {$user}!",
                    "vars": [
                      {
                        "outer": "{$user}",
                        "inner": "$user"
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
        &ParseTestLang::php(indoc! {r#"
            <?php
            $user_prompt = "Hello, {$name}! How is the weather today in {$city}?";
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
                          start: 21,
                          end: 75,
                        ),
                        inner: Span(
                          start: 22,
                          end: 74,
                        ),
                      ),
                      enclosure: Span(
                        start: 6,
                        end: 75,
                      ),
                      exp: "\"Hello, {$name}! How is the weather today in {$city}?\"",
                      vars: [
                        PromptVar(
                          exp: "{$name}",
                          span: SpanShape(
                            outer: Span(
                              start: 29,
                              end: 36,
                            ),
                            inner: Span(
                              start: 30,
                              end: 35,
                            ),
                          ),
                        ),
                        PromptVar(
                          exp: "{$city}",
                          span: SpanShape(
                            outer: Span(
                              start: 66,
                              end: 73,
                            ),
                            inner: Span(
                              start: 67,
                              end: 72,
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
                    "enclosure": "$user_prompt = \"Hello, {$name}! How is the weather today in {$city}?\"",
                    "outer": "\"Hello, {$name}! How is the weather today in {$city}?\"",
                    "inner": "Hello, {$name}! How is the weather today in {$city}?",
                    "vars": [
                      {
                        "outer": "{$name}",
                        "inner": "$name"
                      },
                      {
                        "outer": "{$city}",
                        "inner": "$city"
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

#[ignore]
#[test]
fn exp() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            $user_prompt = "Hello, {$user->name}! How is the weather today in {$user->location->city}?";
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
                          start: 21,
                          end: 97,
                        ),
                        inner: Span(
                          start: 22,
                          end: 96,
                        ),
                      ),
                      enclosure: Span(
                        start: 6,
                        end: 97,
                      ),
                      exp: "\"Hello, {$user->name}! How is the weather today in {$user->location->city}?\"",
                      vars: [
                        PromptVar(
                          exp: "{$user",
                          span: SpanShape(
                            outer: Span(
                              start: 29,
                              end: 35,
                            ),
                            inner: Span(
                              start: 30,
                              end: 35,
                            ),
                          ),
                        ),
                        PromptVar(
                          exp: "{$user",
                          span: SpanShape(
                            outer: Span(
                              start: 72,
                              end: 78,
                            ),
                            inner: Span(
                              start: 73,
                              end: 78,
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
                    "enclosure": "$user_prompt = \"Hello, {$user->name}! How is the weather today in {$user->location->city}?\"",
                    "outer": "\"Hello, {$user->name}! How is the weather today in {$user->location->city}?\"",
                    "inner": "Hello, {$user->name}! How is the weather today in {$user->location->city}?",
                    "vars": [
                      {
                        "outer": "{$user",
                        "inner": "$user"
                      },
                      {
                        "outer": "{$user",
                        "inner": "$user"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, {0}->name}! How is the weather today in {1}->location->city}?"
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

#[ignore]
#[test]
fn exp_complex() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            $user_prompt = "This item is {$price > 100 ? 'expensive' : 'cheap'}...";
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
                          start: 21,
                          end: 77,
                        ),
                        inner: Span(
                          start: 22,
                          end: 76,
                        ),
                      ),
                      enclosure: Span(
                        start: 6,
                        end: 77,
                      ),
                      exp: "\"This item is {$price > 100 ? \'expensive\' : \'cheap\'}...\"",
                      vars: [
                        PromptVar(
                          exp: "{$price",
                          span: SpanShape(
                            outer: Span(
                              start: 35,
                              end: 42,
                            ),
                            inner: Span(
                              start: 36,
                              end: 42,
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
                    "enclosure": "$user_prompt = \"This item is {$price > 100 ? 'expensive' : 'cheap'}...\"",
                    "outer": "\"This item is {$price > 100 ? 'expensive' : 'cheap'}...\"",
                    "inner": "This item is {$price > 100 ? 'expensive' : 'cheap'}...",
                    "vars": [
                      {
                        "outer": "{$price",
                        "inner": "$price"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "This item is {0} > 100 ? 'expensive' : 'cheap'}..."
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
