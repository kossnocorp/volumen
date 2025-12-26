use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn single_var() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            string userPrompt = $"Welcome, {user}!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
                      span: SpanShape(
                        outer: (20, 39),
                        inner: (22, 38),
                      ),
                      enclosure: (0, 40),
                      exp: "$\"Welcome, {user}!\"",
                      vars: [
                        PromptVar(
                          exp: "{user}",
                          span: SpanShape(
                            outer: (31, 37),
                            inner: (32, 36),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                  ],
                )
                "#);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "string userPrompt = $\"Welcome, {user}!\";",
                    "outer": "$\"Welcome, {user}!\"",
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
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Welcome, {0}!"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"
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
        &ParseTestLang::cs(indoc! {r#"
            string userPrompt = $"Hello, {name}! How is the weather today in {city}?";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
                      span: SpanShape(
                        outer: (20, 73),
                        inner: (22, 72),
                      ),
                      enclosure: (0, 74),
                      exp: "$\"Hello, {name}! How is the weather today in {city}?\"",
                      vars: [
                        PromptVar(
                          exp: "{name}",
                          span: SpanShape(
                            outer: (29, 35),
                            inner: (30, 34),
                          ),
                        ),
                        PromptVar(
                          exp: "{city}",
                          span: SpanShape(
                            outer: (65, 71),
                            inner: (66, 70),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                  ],
                )
                "#);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "string userPrompt = $\"Hello, {name}! How is the weather today in {city}?\";",
                    "outer": "$\"Hello, {name}! How is the weather today in {city}?\"",
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
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello, {0}! How is the weather today in {1}?"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"
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
        &ParseTestLang::cs(indoc! {r#"
            string userPrompt = $"Hello, {user.Name}! How is the weather today in {user.Location.City}?";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
                      span: SpanShape(
                        outer: (20, 92),
                        inner: (22, 91),
                      ),
                      enclosure: (0, 93),
                      exp: "$\"Hello, {user.Name}! How is the weather today in {user.Location.City}?\"",
                      vars: [
                        PromptVar(
                          exp: "{user.Name}",
                          span: SpanShape(
                            outer: (29, 40),
                            inner: (30, 39),
                          ),
                        ),
                        PromptVar(
                          exp: "{user.Location.City}",
                          span: SpanShape(
                            outer: (70, 90),
                            inner: (71, 89),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                  ],
                )
                "#);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "string userPrompt = $\"Hello, {user.Name}! How is the weather today in {user.Location.City}?\";",
                    "outer": "$\"Hello, {user.Name}! How is the weather today in {user.Location.City}?\"",
                    "inner": "Hello, {user.Name}! How is the weather today in {user.Location.City}?",
                    "vars": [
                      {
                        "outer": "{user.Name}",
                        "inner": "user.Name"
                      },
                      {
                        "outer": "{user.Location.City}",
                        "inner": "user.Location.City"
                      }
                    ]
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello, {0}! How is the weather today in {1}?"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"
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
        &ParseTestLang::cs(indoc! {r#"
            string userPrompt = $"This item is {(price > 100 ? "expensive" : "cheap")}...";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
                      span: SpanShape(
                        outer: (20, 78),
                        inner: (22, 77),
                      ),
                      enclosure: (0, 79),
                      exp: "$\"This item is {(price > 100 ? \"expensive\" : \"cheap\")}...\"",
                      vars: [
                        PromptVar(
                          exp: "{(price > 100 ? \"expensive\" : \"cheap\")}",
                          span: SpanShape(
                            outer: (35, 74),
                            inner: (36, 73),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                  ],
                )
                "#);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "string userPrompt = $\"This item is {(price > 100 ? \"expensive\" : \"cheap\")}...\";",
                    "outer": "$\"This item is {(price > 100 ? \"expensive\" : \"cheap\")}...\"",
                    "inner": "This item is {(price > 100 ? \"expensive\" : \"cheap\")}...",
                    "vars": [
                      {
                        "outer": "{(price > 100 ? \"expensive\" : \"cheap\")}",
                        "inner": "(price > 100 ? \"expensive\" : \"cheap\")"
                      }
                    ]
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "This item is {0}..."
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"
                [
                  []
                ]
                ");
            }),
        },
    );
}
