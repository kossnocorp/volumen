use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn join_assignment() {
    ParseTest::test(
        &ParseTestLang::py(indoc! {r#"
            # @prompt
            prompt = " ".join(["Hello", user, "!"])
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
