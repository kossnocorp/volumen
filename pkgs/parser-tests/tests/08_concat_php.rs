use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn concat() {
    ParseTest::test(
        &ParseTestLang::php(indoc! {r#"
            <?php
            /* @prompt */
            g$greeting = "Hello, " . $name . "!";
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
