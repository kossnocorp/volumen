use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn multiple() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            $user_prompt = "Hello, {$name}!";
            $greeting_prompt = "Welcome {$user}!";
            // @prompt
            $farewell = "Goodbye {$user_name}!";
            // @prompt
            $system = "You are an AI assistant";
            $regular = "Not a prompt {$value}";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (6, 38),
                      span: SpanShape(
                        outer: (21, 38),
                        inner: (22, 37),
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
                          span: (36, 37),
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
                      ],
                      annotations: [],
                    ),
                    Prompt(
                      file: "prompts.php",
                      enclosure: (40, 77),
                      span: SpanShape(
                        outer: (59, 77),
                        inner: (60, 76),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (60, 68),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (68, 75),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (75, 76),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (68, 75),
                            inner: (69, 74),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                    Prompt(
                      file: "prompts.php",
                      enclosure: (79, 125),
                      span: SpanShape(
                        outer: (102, 125),
                        inner: (103, 124),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (103, 111),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (111, 123),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (123, 124),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (111, 123),
                            inner: (112, 122),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (79, 89),
                              inner: (81, 89),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "prompts.php",
                      enclosure: (127, 173),
                      span: SpanShape(
                        outer: (148, 173),
                        inner: (149, 172),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (149, 172),
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
                              outer: (127, 137),
                              inner: (129, 137),
                            ),
                          ],
                        ),
                      ],
                    ),
                  ],
                )
                "#);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "$user_prompt = \"Hello, {$name}!\"",
                    "outer": "\"Hello, {$name}!\"",
                    "inner": "Hello, {$name}!",
                    "vars": [
                      {
                        "outer": "{$name}",
                        "inner": "$name"
                      }
                    ]
                  },
                  {
                    "enclosure": "$greeting_prompt = \"Welcome {$user}!\"",
                    "outer": "\"Welcome {$user}!\"",
                    "inner": "Welcome {$user}!",
                    "vars": [
                      {
                        "outer": "{$user}",
                        "inner": "$user"
                      }
                    ]
                  },
                  {
                    "enclosure": "// @prompt\n$farewell = \"Goodbye {$user_name}!\"",
                    "outer": "\"Goodbye {$user_name}!\"",
                    "inner": "Goodbye {$user_name}!",
                    "vars": [
                      {
                        "outer": "{$user_name}",
                        "inner": "$user_name"
                      }
                    ]
                  },
                  {
                    "enclosure": "// @prompt\n$system = \"You are an AI assistant\"",
                    "outer": "\"You are an AI assistant\"",
                    "inner": "You are an AI assistant",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello, {0}!",
                  "Welcome {0}!",
                  "Goodbye {0}!",
                  "You are an AI assistant"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
                [
                  [],
                  [],
                  [
                    [
                      {
                        "outer": "// @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ],
                  [
                    [
                      {
                        "outer": "// @prompt",
                        "inner": " @prompt"
                      }
                    ]
                  ]
                ]
                "#);
            }),
        },
    );
}
