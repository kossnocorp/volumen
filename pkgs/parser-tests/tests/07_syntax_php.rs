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
                      enclosure: (6, 219),
                      span: SpanShape(
                        outer: (27, 219),
                        inner: (35, 215),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (35, 215),
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
                              outer: (6, 16),
                              inner: (8, 16),
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
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "// @prompt\n$system = <<<TEXT\nYou are a helpful assistant.\nYou will answer the user's questions to the best of your ability.\nIf you don't know the answer, just say that you don't know, don't try to make it up.\nTEXT",
                    "outer": "<<<TEXT\nYou are a helpful assistant.\nYou will answer the user's questions to the best of your ability.\nIf you don't know the answer, just say that you don't know, don't try to make it up.\nTEXT",
                    "inner": "You are a helpful assistant.\nYou will answer the user's questions to the best of your ability.\nIf you don't know the answer, just say that you don't know, don't try to make it up.\n",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "You are a helpful assistant.\nYou will answer the user's questions to the best of your ability.\nIf you don't know the answer, just say that you don't know, don't try to make it up.\n"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
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
                      enclosure: (6, 90),
                      span: SpanShape(
                        outer: (25, 90),
                        inner: (33, 86),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (33, 40),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (40, 47),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (47, 77),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (77, 84),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (84, 86),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (40, 47),
                            inner: (41, 46),
                          ),
                        ),
                        PromptVar(
                          span: SpanShape(
                            outer: (77, 84),
                            inner: (78, 83),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (6, 16),
                              inner: (8, 16),
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
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "// @prompt\n$user = <<<TEXT\nHello, {$name}!\nHow is the weather today in {$city}?\nTEXT",
                    "outer": "<<<TEXT\nHello, {$name}!\nHow is the weather today in {$city}?\nTEXT",
                    "inner": "Hello, {$name}!\nHow is the weather today in {$city}?\n",
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
                  "Hello, {0}!\nHow is the weather today in {1}?\n"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
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
