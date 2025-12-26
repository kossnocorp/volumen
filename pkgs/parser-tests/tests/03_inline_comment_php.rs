use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            $greeting = /* @prompt */ "Welcome, {$user}!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (6, 51),
                      span: SpanShape(
                        outer: (32, 51),
                        inner: (33, 50),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (33, 42),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (42, 49),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (49, 50),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (42, 49),
                            inner: (43, 48),
                          ),
                          exp: "{$user}",
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (18, 31),
                              inner: (20, 29),
                            ),
                          ],
                          exp: "/* @prompt */",
                        ),
                      ],
                      exp: "\"Welcome, {$user}!\"",
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "$greeting = /* @prompt */ \"Welcome, {$user}!\"",
                    "outer": "\"Welcome, {$user}!\"",
                    "inner": "Welcome, {$user}!",
                    "vars": [
                      {
                        "outer": "{$user}",
                        "inner": "$user"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Welcome, {0}!"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    [
                      {
                        "outer": "/* @prompt */",
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

#[test]
fn phpdoc() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            $hello = /** @prompt */ "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (6, 45),
                      span: SpanShape(
                        outer: (30, 45),
                        inner: (31, 44),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (31, 44),
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
                              outer: (15, 29),
                              inner: (18, 27),
                            ),
                          ],
                          exp: "/** @prompt */",
                        ),
                      ],
                      exp: "\"Hello, world!\"",
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "$hello = /** @prompt */ \"Hello, world!\"",
                    "outer": "\"Hello, world!\"",
                    "inner": "Hello, world!",
                    "vars": []
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello, world!"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
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

#[test]
fn inexact() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            $greeting = /* @prompting */ "Welcome, {$user}!";
            $whatever = /* wrong@prompt */ "That's not it!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [],
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
fn dirty() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            $greeting = /* @prompt greeting */ "Welcome, {$user}!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.php",
                      enclosure: (6, 60),
                      span: SpanShape(
                        outer: (41, 60),
                        inner: (42, 59),
                      ),
                      content: [
                        PromptContentTokenStr(
                          type: "str",
                          span: (42, 51),
                        ),
                        PromptContentTokenVar(
                          type: "var",
                          span: (51, 58),
                        ),
                        PromptContentTokenStr(
                          type: "str",
                          span: (58, 59),
                        ),
                      ],
                      joint: SpanShape(
                        outer: (0, 0),
                        inner: (0, 0),
                      ),
                      vars: [
                        PromptVar(
                          span: SpanShape(
                            outer: (51, 58),
                            inner: (52, 57),
                          ),
                          exp: "{$user}",
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          spans: [
                            SpanShape(
                              outer: (18, 40),
                              inner: (20, 38),
                            ),
                          ],
                          exp: "/* @prompt greeting */",
                        ),
                      ],
                      exp: "\"Welcome, {$user}!\"",
                    ),
                  ],
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r#"
                [
                  {
                    "enclosure": "$greeting = /* @prompt greeting */ \"Welcome, {$user}!\"",
                    "outer": "\"Welcome, {$user}!\"",
                    "inner": "Welcome, {$user}!",
                    "vars": [
                      {
                        "outer": "{$user}",
                        "inner": "$user"
                      }
                    ]
                  }
                ]
                "#);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Welcome, {0}!"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r#"
                [
                  [
                    [
                      {
                        "outer": "/* @prompt greeting */",
                        "inner": " @prompt greeting "
                      }
                    ]
                  ]
                ]
                "#);
            }),
        },
    );
}
