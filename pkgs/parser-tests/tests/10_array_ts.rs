use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn join_method() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const prompt = ["Hello", user, "!"].join("\n");
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @"");
            }),
            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @"");
            }),
        },
    );
}

#[test]
fn array_simple() {
    ParseTest::test(
        &ParseTestLang::ts(indoc! {r#"
            // @prompt
            const prompt = ["Hello ", user, "!"];
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),
            cuts: Box::new(|cuts| {
                assert_json_snapshot!(cuts, @"");
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @"");
            }),
            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @"");
            }),
        },
    );
}
