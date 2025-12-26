use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[ignore]
#[test]
fn multiple() {
    ParseTest::test(
        &ParseTestLang::go(indoc! {r#"
            userPrompt := "Hello, name!"
            greeting := /* @prompt */ "Welcome!"
            // @prompt
            farewell := "Goodbye!"
            /** @prompt */
            system := "You are an AI assistant"
            regular := "Not a prompt"
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
