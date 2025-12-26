use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn multiple() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            user_prompt = "Hello, #{name}!"
            # @prompt
            farewell = "Goodbye #{user.name}!"
            regular = "Not a prompt #{value}"
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (0, 31),
                      span: SpanShape(
                        outer: (14, 31),
                        inner: (15, 30),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (15, 30),
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
                          exp: "#{name}",
                        ),
                      ],
                      annotations: [],
                      exp: "\"Hello, #{name}!\"",
                    ),
                    Prompt(
                      file: "prompts.rb",
                      enclosure: (32, 76),
                      span: SpanShape(
                        outer: (53, 76),
                        inner: (54, 75),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (54, 75),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (62, 74),
                            inner: (64, 73),
                          ),
                          exp: "#{user.name}",
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (32, 41),
                              inner: (33, 41),
                            ),
                          ],
                          exp: "# @prompt",
                        ),
                      ],
                      exp: "\"Goodbye #{user.name}!\"",
                    ),
                  ],
                )
                "##);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r##"
                [
                  {
                    "enclosure": "user_prompt = \"Hello, #{name}!\"",
                    "outer": "\"Hello, #{name}!\"",
                    "inner": "Hello, #{name}!",
                    "vars": [
                      {
                        "outer": "#{name}",
                        "inner": "name"
                      }
                    ]
                  },
                  {
                    "enclosure": "# @prompt\nfarewell = \"Goodbye #{user.name}!\"",
                    "outer": "\"Goodbye #{user.name}!\"",
                    "inner": "Goodbye #{user.name}!",
                    "vars": [
                      {
                        "outer": "#{user.name}",
                        "inner": "user.name"
                      }
                    ]
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello, {0}!",
                  "Goodbye {0}!"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r##"
                [
                  [],
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
