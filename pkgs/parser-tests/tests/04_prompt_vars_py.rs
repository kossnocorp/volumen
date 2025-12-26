use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

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
                        outer: (18, 37),
                        inner: (20, 36),
                      ),
                      enclosure: (0, 37),
                      exp: "f\"Welcome, {user}!\"",
                      vars: [
                        PromptVar(
                          exp: "{user}",
                          span: SpanShape(
                            outer: (29, 35),
                            inner: (30, 34),
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
                assert_json_snapshot!(annotations, @"
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
                        outer: (14, 67),
                        inner: (16, 66),
                      ),
                      enclosure: (0, 67),
                      exp: "f\"Hello, {name}! How is the weather today in {city}?\"",
                      vars: [
                        PromptVar(
                          exp: "{name}",
                          span: SpanShape(
                            outer: (23, 29),
                            inner: (24, 28),
                          ),
                        ),
                        PromptVar(
                          exp: "{city}",
                          span: SpanShape(
                            outer: (59, 65),
                            inner: (60, 64),
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
                assert_json_snapshot!(annotations, @"
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
                        outer: (14, 86),
                        inner: (16, 85),
                      ),
                      enclosure: (0, 86),
                      exp: "f\"Hello, {user.name}! How is the weather today in {user.location.city}?\"",
                      vars: [
                        PromptVar(
                          exp: "{user.name}",
                          span: SpanShape(
                            outer: (23, 34),
                            inner: (24, 33),
                          ),
                        ),
                        PromptVar(
                          exp: "{user.location.city}",
                          span: SpanShape(
                            outer: (64, 84),
                            inner: (65, 83),
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
                assert_json_snapshot!(annotations, @"
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
                        outer: (14, 76),
                        inner: (16, 75),
                      ),
                      enclosure: (0, 76),
                      exp: "f\"This item is {(\'expensive\' if price > 100 else \'cheap\')}...\"",
                      vars: [
                        PromptVar(
                          exp: "{(\'expensive\' if price > 100 else \'cheap\')}",
                          span: SpanShape(
                            outer: (29, 72),
                            inner: (30, 71),
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
                assert_json_snapshot!(annotations, @"
                [
                  []
                ]
                ");
            }),
        },
    );
}
