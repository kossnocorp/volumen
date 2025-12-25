use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn invalid() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            string invalid = "unclosed string
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
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"[]");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"[]");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"[]");
            }),
        },
    );
}

#[test]
fn verbatim_string() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            string system = @"You are a helpful assistant.
            You can help with various tasks.";
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
                        outer: Span(
                          start: 27,
                          end: 91,
                        ),
                        inner: Span(
                          start: 29,
                          end: 90,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 92,
                      ),
                      exp: "@\"You are a helpful assistant.\nYou can help with various tasks.\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 10,
                          ),
                          exp: "// @prompt",
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
                    "enclosure": "// @prompt\nstring system = @\"You are a helpful assistant.\nYou can help with various tasks.\";",
                    "outer": "@\"You are a helpful assistant.\nYou can help with various tasks.\"",
                    "inner": "You are a helpful assistant.\nYou can help with various tasks.",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "You are a helpful assistant.\nYou can help with various tasks."
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
                [
                  [
                    "// @prompt"
                  ]
                ]
                "#);
            }),
        },
    );
}

#[test]
fn interpolated_verbatim() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            string greeting = $@"Hello, {name}!
            Welcome to {city}.";
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
                        outer: Span(
                          start: 29,
                          end: 66,
                        ),
                        inner: Span(
                          start: 32,
                          end: 65,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 67,
                      ),
                      exp: "$@\"Hello, {name}!\nWelcome to {city}.\"",
                      vars: [
                        PromptVar(
                          exp: "{name}",
                          span: SpanShape(
                            outer: Span(
                              start: 39,
                              end: 45,
                            ),
                            inner: Span(
                              start: 40,
                              end: 44,
                            ),
                          ),
                        ),
                        PromptVar(
                          exp: "{city}",
                          span: SpanShape(
                            outer: Span(
                              start: 58,
                              end: 64,
                            ),
                            inner: Span(
                              start: 59,
                              end: 63,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 10,
                          ),
                          exp: "// @prompt",
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
                    "enclosure": "// @prompt\nstring greeting = $@\"Hello, {name}!\nWelcome to {city}.\";",
                    "outer": "$@\"Hello, {name}!\nWelcome to {city}.\"",
                    "inner": "Hello, {name}!\nWelcome to {city}.",
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
                  "Hello, {0}!\nWelcome to {1}."
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
                [
                  [
                    "// @prompt"
                  ]
                ]
                "#);
            }),
        },
    );
}

#[test]
fn multiline() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            string user = $"Hello, {name}!\nHow is the weather today in {city}?\n";
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
                        outer: Span(
                          start: 25,
                          end: 81,
                        ),
                        inner: Span(
                          start: 27,
                          end: 80,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 82,
                      ),
                      exp: "$\"Hello, {name}!\\nHow is the weather today in {city}?\\n\"",
                      vars: [
                        PromptVar(
                          exp: "{name}",
                          span: SpanShape(
                            outer: Span(
                              start: 34,
                              end: 40,
                            ),
                            inner: Span(
                              start: 35,
                              end: 39,
                            ),
                          ),
                        ),
                        PromptVar(
                          exp: "{city}",
                          span: SpanShape(
                            outer: Span(
                              start: 71,
                              end: 77,
                            ),
                            inner: Span(
                              start: 72,
                              end: 76,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 10,
                          ),
                          exp: "// @prompt",
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
                    "enclosure": "// @prompt\nstring user = $\"Hello, {name}!\\nHow is the weather today in {city}?\\n\";",
                    "outer": "$\"Hello, {name}!\\nHow is the weather today in {city}?\\n\"",
                    "inner": "Hello, {name}!\\nHow is the weather today in {city}?\\n",
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
                  "Hello, {0}!\\nHow is the weather today in {1}?\\n"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
                [
                  [
                    "// @prompt"
                  ]
                ]
                "#);
            }),
        },
    );
}
