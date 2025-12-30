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
                      enclosure: (6, 97),
                      span: SpanShape(
                        outer: (21, 97),
                        inner: (22, 96),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (22, 29),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (29, 42),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (42, 72),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (72, 95),
                          index: 1,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (95, 96),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (29, 42),
                            inner: (30, 41),
                          ),
                        ),
                        PromptVar(
                          span: SpanShape(
                            outer: (72, 95),
                            inner: (73, 94),
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
                        "outer": "{$user->name}",
                        "inner": "$user->name"
                      },
                      {
                        "outer": "{$user->location->city}",
                        "inner": "$user->location->city"
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
                      enclosure: (6, 77),
                      span: SpanShape(
                        outer: (21, 77),
                        inner: (22, 76),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (22, 35),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (35, 73),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (73, 76),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (35, 73),
                            inner: (36, 72),
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
                        "outer": "{$price > 100 ? 'expensive' : 'cheap'}",
                        "inner": "$price > 100 ? 'expensive' : 'cheap'"
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
