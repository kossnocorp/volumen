use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
#[ignore = "TODO: Tree-sitter C# does not include block comments in AST - inline `/* @prompt */` not detected"]
fn multiple() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            string userPrompt = $"Hello, {name}!";
            string greeting = /* @prompt */ $"Welcome {user}!";
            // @prompt
            string farewell = $"Goodbye {user.name}!";
            /** @prompt */
            string system = "You are an AI assistant";
            string regular = $"Not a prompt {value}";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.cs",
                      enclosure: (0, 38),
                      span: SpanShape(
                        outer: (20, 37),
                        inner: (22, 36),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (22, 29),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (29, 35),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (35, 36),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (29, 35),
                            inner: (30, 34),
                          ),
                        ),
                      ],
                      annotations: [],
                    ),
                    Prompt(
                      file: "Prompts.cs",
                      enclosure: (91, 144),
                      span: SpanShape(
                        outer: (120, 143),
                        inner: (122, 142),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (122, 130),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (130, 141),
                          index: 0,
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (141, 142),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (130, 141),
                            inner: (131, 140),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (91, 101),
                              inner: (93, 101),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Prompt(
                      file: "Prompts.cs",
                      enclosure: (145, 202),
                      span: SpanShape(
                        outer: (176, 201),
                        inner: (177, 200),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (177, 200),
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
                              outer: (145, 159),
                              inner: (148, 157),
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
                    "enclosure": "string userPrompt = $\"Hello, {name}!\";",
                    "outer": "$\"Hello, {name}!\"",
                    "inner": "Hello, {name}!",
                    "vars": [
                      {
                        "outer": "{name}",
                        "inner": "name"
                      }
                    ]
                  },
                  {
                    "enclosure": "// @prompt\nstring farewell = $\"Goodbye {user.name}!\";",
                    "outer": "$\"Goodbye {user.name}!\"",
                    "inner": "Goodbye {user.name}!",
                    "vars": [
                      {
                        "outer": "{user.name}",
                        "inner": "user.name"
                      }
                    ]
                  },
                  {
                    "enclosure": "/** @prompt */\nstring system = \"You are an AI assistant\";",
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
                  "Goodbye {0}!",
                  "You are an AI assistant"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
                [
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
                        "outer": "/** @prompt */",
                        "inner": " @prompt "
                      }
                    ]
                  ]
                ]
                "#);
            }),
        },
    );
}
