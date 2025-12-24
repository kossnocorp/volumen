use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn multiple() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            const userPrompt = `Hello, ${name}!`;
            const greeting = /* @prompt */ `Welcome ${user}!`;
            // @prompt
            const farewell = `Goodbye ${user.name}!`;
            /** @prompt */
            const system = "You are an AI assistant";
            const regular = `Not a prompt ${value}`;
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
                          end: 36,
                        ),
                        inner: Span(
                          start: 20,
                          end: 35,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 37,
                      ),
                      exp: "`Hello, ${name}!`",
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
                      ],
                      annotations: [],
                    ),
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 69,
                          end: 87,
                        ),
                        inner: Span(
                          start: 70,
                          end: 86,
                        ),
                      ),
                      enclosure: Span(
                        start: 38,
                        end: 88,
                      ),
                      exp: "`Welcome ${user}!`",
                      vars: [
                        PromptVar(
                          exp: "${user}",
                          span: SpanShape(
                            outer: Span(
                              start: 78,
                              end: 85,
                            ),
                            inner: Span(
                              start: 80,
                              end: 84,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 55,
                            end: 68,
                          ),
                          exp: "/* @prompt */",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 117,
                          end: 140,
                        ),
                        inner: Span(
                          start: 118,
                          end: 139,
                        ),
                      ),
                      enclosure: Span(
                        start: 89,
                        end: 141,
                      ),
                      exp: "`Goodbye ${user.name}!`",
                      vars: [
                        PromptVar(
                          exp: "${user.name}",
                          span: SpanShape(
                            outer: Span(
                              start: 126,
                              end: 138,
                            ),
                            inner: Span(
                              start: 128,
                              end: 137,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 89,
                            end: 99,
                          ),
                          exp: "// @prompt",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.js",
                      span: SpanShape(
                        outer: Span(
                          start: 172,
                          end: 197,
                        ),
                        inner: Span(
                          start: 173,
                          end: 196,
                        ),
                      ),
                      enclosure: Span(
                        start: 142,
                        end: 198,
                      ),
                      exp: "\"You are an AI assistant\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 142,
                            end: 156,
                          ),
                          exp: "/** @prompt */",
                        ),
                      ],
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "const userPrompt = `Hello, ${name}!`;",
                    "outer": "`Hello, ${name}!`",
                    "inner": "Hello, ${name}!",
                    "vars": [
                      {
                        "outer": "${name}",
                        "inner": "name"
                      }
                    ]
                  },
                  {
                    "enclosure": "const greeting = /* @prompt */ `Welcome ${user}!`;",
                    "outer": "`Welcome ${user}!`",
                    "inner": "Welcome ${user}!",
                    "vars": [
                      {
                        "outer": "${user}",
                        "inner": "user"
                      }
                    ]
                  },
                  {
                    "enclosure": "// @prompt\nconst farewell = `Goodbye ${user.name}!`;",
                    "outer": "`Goodbye ${user.name}!`",
                    "inner": "Goodbye ${user.name}!",
                    "vars": [
                      {
                        "outer": "${user.name}",
                        "inner": "user.name"
                      }
                    ]
                  },
                  {
                    "enclosure": "/** @prompt */\nconst system = \"You are an AI assistant\";",
                    "outer": "\"You are an AI assistant\"",
                    "inner": "You are an AI assistant",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, {0}!",
                  "Welcome {0}!",
                  "Goodbye {0}!",
                  "You are an AI assistant"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [],
                  [
                    "/* @prompt */"
                  ],
                  [
                    "// @prompt"
                  ],
                  [
                    "/** @prompt */"
                  ]
                ]
                "#);
            }),
        },
    );
}
