use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            string system = "You are a helpful assistant.";
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
                          end: 57,
                        ),
                        inner: Span(
                          start: 28,
                          end: 56,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 58,
                      ),
                      exp: "\"You are a helpful assistant.\"",
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
                    "enclosure": "// @prompt\nstring system = \"You are a helpful assistant.\";",
                    "outer": "\"You are a helpful assistant.\"",
                    "inner": "You are a helpful assistant.",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "You are a helpful assistant."
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
fn assigned() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            string assigned;
            assigned = "Assigned value";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[test]
fn assigned_late_comment() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            string assigned = "Assigned";
            // @prompt
            assigned = "Assigned again";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[test]
fn reassigned() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            string reassigned = "First";
            reassigned = "Second";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[test]
fn inexact() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            string inexactPrompt = "Exact prompt";
            string inexact = "Inexact";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[test]
fn mixed() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt system
            string system = "You are a helpful assistant.";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[test]
fn mixed_nested() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            void MyFunction() {
                string prompt = "First";
                string second = "Second";
            }
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
                          start: 40,
                          end: 47,
                        ),
                        inner: Span(
                          start: 41,
                          end: 46,
                        ),
                      ),
                      enclosure: Span(
                        start: 24,
                        end: 48,
                      ),
                      exp: "\"First\"",
                      vars: [],
                      annotations: [],
                    ),
                  ],
                )
                "#);
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @r#"
                [
                  {
                    "enclosure": "string prompt = \"First\";",
                    "outer": "\"First\"",
                    "inner": "First",
                    "vars": []
                  }
                ]
                "#);
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @r#"
                [
                  "First"
                ]
                "#);
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"
                [
                  []
                ]
                ");
            }),
        },
    );
}

#[test]
fn mixed_none() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            string prompt = "First";
            string second = "Second";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[test]
fn mixed_assign() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            string mixedPrompt = "First";
            string mixed = "Second";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[test]
fn mixed_reassign() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt def
            int hello = 123;
            hello = 456;
            // @prompting
            string helloStr = "Hi";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[test]
fn spaced() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // This is a comment
            // @prompt
            // This is another comment
            string spaced = "Spaced";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[test]
fn dirty() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            int dirty = 123;
            // @prompt
            string dirtyStr = "Dirty";
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[test]
fn multi() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            (string hello, string world) = ("Hello", "World");
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[test]
fn destructuring() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            var values = new[] { "Hello", "World" };
            var (hello1, world1) = (values[0], values[1]);
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}

#[test]
fn chained() {
    ParseTest::test(
        &ParseTestLang::cs(indoc! {r#"
            // @prompt
            string hello = "Hi";
            string world = hello;
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interp| {
                assert_json_snapshot!(interp, @"");
            }),
            annotations: Box::new(|annot| {
                assert_json_snapshot!(annot, @"");
            }),
        },
    );
}
