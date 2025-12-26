use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[ignore]
#[test]
fn join_method() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            prompt = "\n".join(["Hello", user, "!"])
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

#[ignore]
#[test]
fn array_simple() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            prompt = ["Hello ", user, "!"]
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
