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
                      enclosure: (6, 40),
                      span: SpanShape(
                        outer: (21, 40),
                        inner: (22, 39),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (22, 31),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (31, 38),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (38, 39),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (31, 38),
                            inner: (32, 37),
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
                      enclosure: (6, 75),
                      span: SpanShape(
                        outer: (21, 75),
                        inner: (22, 74),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (22, 29),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (29, 36),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (36, 66),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (66, 73),
                          index: 1,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (73, 74),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (29, 36),
                            inner: (30, 35),
                          ),
                        ),
                        PromptVar(
                          span: SpanShape(
                            outer: (66, 73),
                            inner: (67, 72),
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
                assert_json_snapshot!(annotations, @"
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
