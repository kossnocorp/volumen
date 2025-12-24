use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn implode_assignment() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            /* @prompt */
            $prompt = implode("\n", ["Hello", $user, "!"]);
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @"");
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @"");
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
