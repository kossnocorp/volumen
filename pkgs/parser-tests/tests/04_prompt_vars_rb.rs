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
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (0, 33),
                      span: SpanShape(
                        outer: (14, 33),
                        inner: (15, 32),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (15, 24),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (24, 31),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (31, 32),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (24, 31),
                            inner: (26, 30),
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
        &ParseTestLang::rb(indoc! {r#"
            user_prompt = "Hello, #{name}! How is the weather today in #{city}?"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (0, 68),
                      span: SpanShape(
                        outer: (14, 68),
                        inner: (15, 67),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (15, 22),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (22, 29),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (29, 59),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (59, 66),
                          index: 1,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (66, 67),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (22, 29),
                            inner: (24, 28),
                          ),
                        ),
                        PromptVar(
                          span: SpanShape(
                            outer: (59, 66),
                            inner: (61, 65),
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
        &ParseTestLang::rb(indoc! {r#"
            user_prompt = "Hello, #{user.name}! How is the weather today in #{user.location.city}?"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (0, 87),
                      span: SpanShape(
                        outer: (14, 87),
                        inner: (15, 86),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (15, 22),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (22, 34),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (34, 64),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (64, 85),
                          index: 1,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (85, 86),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (22, 34),
                            inner: (24, 33),
                          ),
                        ),
                        PromptVar(
                          span: SpanShape(
                            outer: (64, 85),
                            inner: (66, 84),
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
        &ParseTestLang::rb(indoc! {r#"
            user_prompt = "This item is #{price > 100 ? 'expensive' : 'cheap'}..."
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (0, 70),
                      span: SpanShape(
                        outer: (14, 70),
                        inner: (15, 69),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (15, 28),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (28, 66),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (66, 69),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (28, 66),
                            inner: (30, 65),
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
                assert_json_snapshot!(annotations, @"
                [
                  []
                ]
                ");
            }),
        },
    );
}
