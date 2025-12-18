use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn invalid() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            x = "unclosed
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultError(
                  state: "error",
                  error: "<error>",
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @"[]");
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @"[]");
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @"[]");
            }),
        },
    );
}

#[test]
fn multiline_str() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            system = """You are a helpful assistant.
            You will answer the user's questions to the best of your ability.
            If you don't know the answer, just say that you don't know, don't try to make it up."""
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
                          start: 19,
                          end: 204,
                        ),
                        inner: Span(
                          start: 22,
                          end: 201,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 204,
                      ),
                      exp: "\"\"\"You are a helpful assistant.\nYou will answer the user\'s questions to the best of your ability.\nIf you don\'t know the answer, just say that you don\'t know, don\'t try to make it up.\"\"\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
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
                    "enclosure": "# @prompt\nsystem = \"\"\"You are a helpful assistant.\nYou will answer the user's questions to the best of your ability.\nIf you don't know the answer, just say that you don't know, don't try to make it up.\"\"\"",
                    "outer": "\"\"\"You are a helpful assistant.\nYou will answer the user's questions to the best of your ability.\nIf you don't know the answer, just say that you don't know, don't try to make it up.\"\"\"",
                    "inner": "You are a helpful assistant.\nYou will answer the user's questions to the best of your ability.\nIf you don't know the answer, just say that you don't know, don't try to make it up.",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "You are a helpful assistant.\nYou will answer the user's questions to the best of your ability.\nIf you don't know the answer, just say that you don't know, don't try to make it up."
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn multiline_fstr() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            user = f"""Hello, {name}!
            How is the weather today in {city}?
            """
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
                          start: 17,
                          end: 75,
                        ),
                        inner: Span(
                          start: 21,
                          end: 72,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 75,
                      ),
                      exp: "f\"\"\"Hello, {name}!\nHow is the weather today in {city}?\n\"\"\"",
                      vars: [
                        PromptVar(
                          exp: "{name}",
                          span: SpanShape(
                            outer: Span(
                              start: 28,
                              end: 34,
                            ),
                            inner: Span(
                              start: 29,
                              end: 33,
                            ),
                          ),
                        ),
                        PromptVar(
                          exp: "{city}",
                          span: SpanShape(
                            outer: Span(
                              start: 64,
                              end: 70,
                            ),
                            inner: Span(
                              start: 65,
                              end: 69,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
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
                    "enclosure": "# @prompt\nuser = f\"\"\"Hello, {name}!\nHow is the weather today in {city}?\n\"\"\"",
                    "outer": "f\"\"\"Hello, {name}!\nHow is the weather today in {city}?\n\"\"\"",
                    "inner": "Hello, {name}!\nHow is the weather today in {city}?\n",
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
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, {0}!\nHow is the weather today in {1}?\n"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}
