use indoc::indoc;
use insta::{assert_ron_snapshot, assert_json_snapshot};

mod utils;
use utils::*;

#[test]
fn multiple() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            user_prompt = f"Hello, {name}!"
            greeting_prompt = f"Welcome {user}!"
            # @prompt
            farewell = f"Goodbye {user.name}!"
            # @prompt
            system = "You are an AI assistant";
            regular = f"Not a prompt {value}";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 14,
                          end: 31,
                        ),
                        inner: Span(
                          start: 16,
                          end: 30,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 31,
                      ),
                      exp: "f\"Hello, {name}!\"",
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
                      ],
                      annotations: [],
                    ),
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 50,
                          end: 68,
                        ),
                        inner: Span(
                          start: 52,
                          end: 67,
                        ),
                      ),
                      enclosure: Span(
                        start: 32,
                        end: 68,
                      ),
                      exp: "f\"Welcome {user}!\"",
                      vars: [
                        PromptVar(
                          exp: "{user}",
                          span: SpanShape(
                            outer: Span(
                              start: 60,
                              end: 66,
                            ),
                            inner: Span(
                              start: 61,
                              end: 65,
                            ),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 90,
                          end: 113,
                        ),
                        inner: Span(
                          start: 92,
                          end: 112,
                        ),
                      ),
                      enclosure: Span(
                        start: 69,
                        end: 113,
                      ),
                      exp: "f\"Goodbye {user.name}!\"",
                      vars: [
                        PromptVar(
                          exp: "{user.name}",
                          span: SpanShape(
                            outer: Span(
                              start: 100,
                              end: 111,
                            ),
                            inner: Span(
                              start: 101,
                              end: 110,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 69,
                            end: 78,
                          ),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.py",
                      span: SpanShape(
                        outer: Span(
                          start: 133,
                          end: 158,
                        ),
                        inner: Span(
                          start: 134,
                          end: 157,
                        ),
                      ),
                      enclosure: Span(
                        start: 114,
                        end: 158,
                      ),
                      exp: "\"You are an AI assistant\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 114,
                            end: 123,
                          ),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "user_prompt = f\"Hello, {name}!\"",
                    "outer": "f\"Hello, {name}!\"",
                    "inner": "Hello, {name}!",
                    "vars": [
                      {
                        "outer": "{name}",
                        "inner": "name"
                      }
                    ]
                  },
                  {
                    "enclosure": "greeting_prompt = f\"Welcome {user}!\"",
                    "outer": "f\"Welcome {user}!\"",
                    "inner": "Welcome {user}!",
                    "vars": [
                      {
                        "outer": "{user}",
                        "inner": "user"
                      }
                    ]
                  },
                  {
                    "enclosure": "# @prompt\nfarewell = f\"Goodbye {user.name}!\"",
                    "outer": "f\"Goodbye {user.name}!\"",
                    "inner": "Goodbye {user.name}!",
                    "vars": [
                      {
                        "outer": "{user.name}",
                        "inner": "user.name"
                      }
                    ]
                  },
                  {
                    "enclosure": "# @prompt\nsystem = \"You are an AI assistant\"",
                    "outer": "\"You are an AI assistant\"",
                    "inner": "You are an AI assistant",
                    "vars": []
                  }
                ]
                "##);
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
                assert_json_snapshot!(annotations, @r##"
                [
                  [],
                  [],
                  [
                    "# @prompt"
                  ],
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}
