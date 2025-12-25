use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String system = "You are a helpful assistant.";
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
fn inline() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            /* @prompt */
            String hello = "Hello, world!";
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
fn doc() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            /**
             * @prompt
             */
            String hello = "Hello, world!";
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
fn assigned() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String assigned;
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
        &ParseTestLang::java(indoc! {r#"
            String assigned = "Assigned";
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
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String reassigned = "First";
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
        &ParseTestLang::java(indoc! {r#"
            // @prompting
            String hello = "Hello, world!";
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
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            int number = 42;
            String hello = "Hello, world!";
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
        &ParseTestLang::java(indoc! {r#"
            class Hello {
                void world() {
                    // @prompt
                    int hello = 42;

                    // @prompt
                    int hi = 42;

                    hi = "Hi!";
                }
            }

            String hello = "Hello, world!";
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
fn mixed_none() {
    ParseTest::test(
        &ParseTestLang::java(indoc! {r#"
            String regularString = "This is not special";
            String normalString = "This is not special";
            String regular = "Regular string";
            String message = "Just a message";
            // @prompt
            int number = 1;
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
        &ParseTestLang::java(indoc! {r#"
            // @prompt def
            String hello;
            // @prompt fresh
            hello = "Hi";
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
        &ParseTestLang::java(indoc! {r#"
            // @prompt


            String hello = "Hello, world!";

            // @prompt
            nope();

            String world = "Hello!";
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
        &ParseTestLang::java(indoc! {r#"
            // @prompt system
            String system = "You are a helpful assistant.";
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
        &ParseTestLang::java(indoc! {r#"
            // @prompt
            String hello = "Hello";
            String world = "World";
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
