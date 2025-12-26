use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[ignore]
#[test]
fn simple() {
    ParseTest::test(
        &ParseTestLang::go(indoc! {r#"
            userPrompt := "You are a helpful assistant."
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

#[ignore]
#[test]
fn nested() {
    ParseTest::test(
        &ParseTestLang::go(indoc! {r#"
            func world() {
                helloPrompt := "Hello, world!"

                // @prompt
                alsoPrompt := "Hi!"
            }
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
