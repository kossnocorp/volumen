use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            String greeting = /* @prompt */ "Welcome!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
                      span: SpanShape(
                        outer: Span(
                          start: 32,
                          end: 42,
                        ),
                        inner: Span(
                          start: 33,
                          end: 41,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 43,
                      ),
                      exp: "\"Welcome!\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 18,
                            end: 31,
                          ),
                          exp: "/* @prompt */",
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
                    "enclosure": "String greeting = /* @prompt */ \"Welcome!\";",
                    "outer": "\"Welcome!\"",
                    "inner": "Welcome!",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Welcome!"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
                [
                  [
                    "/* @prompt */"
                  ]
                ]
                "#);
            }),
        },
    );
}

#[test]
fn javadoc() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            String hello = /** @prompt */ "Hello, world!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
                      span: SpanShape(
                        outer: Span(
                          start: 30,
                          end: 45,
                        ),
                        inner: Span(
                          start: 31,
                          end: 44,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 46,
                      ),
                      exp: "\"Hello, world!\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 15,
                            end: 29,
                          ),
                          exp: "/** @prompt */",
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
                    "enclosure": "String hello = /** @prompt */ \"Hello, world!\";",
                    "outer": "\"Hello, world!\"",
                    "inner": "Hello, world!",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Hello, world!"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
                [
                  [
                    "/** @prompt */"
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
        &ParseTestLang::java(indoc! {r#"
            String greeting = /* @prompting */ "Welcome!";
            String whatever = /* wrong@prompt */ "That's not it!";
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
fn dirty() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            String greeting = /* @prompt greeting */ "Welcome!";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "Prompts.java",
                      span: SpanShape(
                        outer: Span(
                          start: 41,
                          end: 51,
                        ),
                        inner: Span(
                          start: 42,
                          end: 50,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 52,
                      ),
                      exp: "\"Welcome!\"",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 18,
                            end: 40,
                          ),
                          exp: "/* @prompt greeting */",
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
                    "enclosure": "String greeting = /* @prompt greeting */ \"Welcome!\";",
                    "outer": "\"Welcome!\"",
                    "inner": "Welcome!",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "Welcome!"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @r#"
                [
                  [
                    "/* @prompt greeting */"
                  ]
                ]
                "#);
            }),
        },
    );
}
