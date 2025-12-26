use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

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
                      enclosure: (0, 39),
                      span: SpanShape(
                        outer: (19, 38),
                        inner: (20, 37),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (20, 37),
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
                            inner: (31, 35),
                          ),
                          exp: "${user}",
                        ),
                      ],
                      annotations: [],
                      exp: "`Welcome, ${user}!`",
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
                      enclosure: (0, 74),
                      span: SpanShape(
                        outer: (19, 73),
                        inner: (20, 72),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (20, 72),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (27, 34),
                            inner: (29, 33),
                          ),
                          exp: "${name}",
                        ),
                        PromptVar(
                          span: SpanShape(
                            outer: (64, 71),
                            inner: (66, 70),
                          ),
                          exp: "${city}",
                        ),
                      ],
                      annotations: [],
                      exp: "`Hello, ${name}! How is the weather today in ${city}?`",
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
                      enclosure: (0, 93),
                      span: SpanShape(
                        outer: (19, 92),
                        inner: (20, 91),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (20, 91),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (27, 39),
                            inner: (29, 38),
                          ),
                          exp: "${user.name}",
                        ),
                        PromptVar(
                          span: SpanShape(
                            outer: (69, 90),
                            inner: (71, 89),
                          ),
                          exp: "${user.location.city}",
                        ),
                      ],
                      annotations: [],
                      exp: "`Hello, ${user.name}! How is the weather today in ${user.location.city}?`",
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
                      enclosure: (0, 76),
                      span: SpanShape(
                        outer: (19, 75),
                        inner: (20, 74),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (20, 74),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (33, 71),
                            inner: (35, 70),
                          ),
                          exp: "${price > 100 ? \"expensive\" : \"cheap\"}",
                        ),
                      ],
                      annotations: [],
                      exp: "`This item is ${price > 100 ? \"expensive\" : \"cheap\"}...`",
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
                assert_json_snapshot!(annotations, @"
                [
                  []
                ]
                ");
            }),
        },
    );
}
