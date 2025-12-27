use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

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
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.py",
                      enclosure: (0, 31),
                      span: SpanShape(
                        outer: (14, 31),
                        inner: (16, 30),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (16, 23),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (23, 29),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (29, 30),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (23, 29),
                            inner: (24, 28),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (32, 68),
                      span: SpanShape(
                        outer: (50, 68),
                        inner: (52, 67),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (52, 60),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (60, 66),
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
                            outer: (60, 66),
                            inner: (61, 65),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (69, 113),
                      span: SpanShape(
                        outer: (90, 113),
                        inner: (92, 112),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (92, 100),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (100, 111),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (111, 112),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (100, 111),
                            inner: (101, 110),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (69, 78),
                              inner: (70, 78),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.py",
                      enclosure: (114, 158),
                      span: SpanShape(
                        outer: (133, 158),
                        inner: (134, 157),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (134, 157),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (114, 123),
                              inner: (115, 123),
                            ),
                          ],
                        ),
                      ],
                    ),
                  ],
                )
                "#);
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
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "# @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ]
                ]
                "##);
            }),
        },
    );
}
