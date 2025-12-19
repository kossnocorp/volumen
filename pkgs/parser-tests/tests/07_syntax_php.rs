use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn invalid() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            $x = "unclosed
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
fn heredoc() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            // @prompt
            $system = <<<TEXT
            You are a helpful assistant.
            You will answer the user's questions to the best of your ability.
            If you don't know the answer, just say that you don't know, don't try to make it up.
            TEXT;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      span: SpanShape(
                        outer: Span(
                          start: 27,
                          end: 219,
                        ),
                        inner: Span(
                          start: 27,
                          end: 219,
                        ),
                      ),
                      enclosure: Span(
                        start: 6,
                        end: 219,
                      ),
                      exp: "<<<TEXT\nYou are a helpful assistant.\nYou will answer the user\'s questions to the best of your ability.\nIf you don\'t know the answer, just say that you don\'t know, don\'t try to make it up.\nTEXT",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 6,
                            end: 16,
                          ),
                          exp: "// @prompt",
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
                    "enclosure": "// @prompt\n$system = <<<TEXT\nYou are a helpful assistant.\nYou will answer the user's questions to the best of your ability.\nIf you don't know the answer, just say that you don't know, don't try to make it up.\nTEXT",
                    "outer": "<<<TEXT\nYou are a helpful assistant.\nYou will answer the user's questions to the best of your ability.\nIf you don't know the answer, just say that you don't know, don't try to make it up.\nTEXT",
                    "inner": "<<<TEXT\nYou are a helpful assistant.\nYou will answer the user's questions to the best of your ability.\nIf you don't know the answer, just say that you don't know, don't try to make it up.\nTEXT",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "<<<TEXT\nYou are a helpful assistant.\nYou will answer the user's questions to the best of your ability.\nIf you don't know the answer, just say that you don't know, don't try to make it up.\nTEXT"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
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
fn heredoc_interpolated() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            // @prompt
            $user = <<<TEXT
            Hello, {$name}!
            How is the weather today in {$city}?
            TEXT;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      span: SpanShape(
                        outer: Span(
                          start: 25,
                          end: 90,
                        ),
                        inner: Span(
                          start: 25,
                          end: 90,
                        ),
                      ),
                      enclosure: Span(
                        start: 6,
                        end: 90,
                      ),
                      exp: "<<<TEXT\nHello, {$name}!\nHow is the weather today in {$city}?\nTEXT",
                      vars: [
                        PromptVar(
                          exp: "{$name}",
                          span: SpanShape(
                            outer: Span(
                              start: 40,
                              end: 47,
                            ),
                            inner: Span(
                              start: 41,
                              end: 46,
                            ),
                          ),
                        ),
                        PromptVar(
                          exp: "{$city}",
                          span: SpanShape(
                            outer: Span(
                              start: 77,
                              end: 84,
                            ),
                            inner: Span(
                              start: 78,
                              end: 83,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 6,
                            end: 16,
                          ),
                          exp: "// @prompt",
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
                    "enclosure": "// @prompt\n$user = <<<TEXT\nHello, {$name}!\nHow is the weather today in {$city}?\nTEXT",
                    "outer": "<<<TEXT\nHello, {$name}!\nHow is the weather today in {$city}?\nTEXT",
                    "inner": "<<<TEXT\nHello, {$name}!\nHow is the weather today in {$city}?\nTEXT",
                    "vars": [
                      {
                        "outer": "{$name}",
                        "inner": "$name"
                      },
                      {
                        "outer": "{$city}",
                        "inner": "$city"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "<<<TEXT\nHello, {0}!\nHow is the weather today in {1}?\nTEXT"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
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
