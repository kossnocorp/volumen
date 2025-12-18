// Comparison tests between Tree-sitter parser and Oxc parser
// This ensures both parsers produce identical results

use indoc::indoc;
use volumen_parser_ts::ParserTs as ParserTsOxc;
use volumen_parser_ts_tree_sitter::ParserTs as ParserTsTreeSitter;

fn compare_parsers(source: &str, filename: &str, test_name: &str) {
    let oxc_result = ParserTsOxc::parse(source, filename);
    let treesitter_result = ParserTsTreeSitter::parse(source, filename);

    // Both should succeed or fail in the same way
    let oxc_prompts = match oxc_result {
        volumen_types::ParseResult::ParseResultSuccess(s) => s.prompts,
        volumen_types::ParseResult::ParseResultError(e) => {
            // If Oxc fails, Tree-sitter should also fail
            assert!(
                matches!(treesitter_result, volumen_types::ParseResult::ParseResultError(_)),
                "{}: Oxc failed but Tree-sitter succeeded", test_name
            );
            return;
        }
    };

    let treesitter_prompts = match treesitter_result {
        volumen_types::ParseResult::ParseResultSuccess(s) => s.prompts,
        volumen_types::ParseResult::ParseResultError(e) => {
            panic!("{}: Tree-sitter failed but Oxc succeeded: {}", test_name, e.error);
        }
    };

    // Compare prompt counts
    assert_eq!(
        oxc_prompts.len(),
        treesitter_prompts.len(),
        "{}: Different number of prompts. Oxc: {}, Tree-sitter: {}",
        test_name,
        oxc_prompts.len(),
        treesitter_prompts.len()
    );

    // Compare each prompt
    for (i, (op, tp)) in oxc_prompts.iter().zip(treesitter_prompts.iter()).enumerate() {
        assert_eq!(op.file, tp.file, "{} prompt {}: different file", test_name, i);
        assert_eq!(op.exp, tp.exp, "{} prompt {}: different exp", test_name, i);
        assert_eq!(op.span, tp.span, "{} prompt {}: different span", test_name, i);
        assert_eq!(op.enclosure, tp.enclosure, "{} prompt {}: different enclosure", test_name, i);
        assert_eq!(op.vars.len(), tp.vars.len(), "{} prompt {}: different vars count", test_name, i);

        for (j, (ov, tv)) in op.vars.iter().zip(tp.vars.iter()).enumerate() {
            assert_eq!(ov.exp, tv.exp, "{} prompt {} var {}: different exp", test_name, i, j);
            assert_eq!(ov.span, tv.span, "{} prompt {} var {}: different span", test_name, i, j);
        }

        assert_eq!(op.annotations.len(), tp.annotations.len(), "{} prompt {}: different annotations count", test_name, i);

        for (j, (oa, ta)) in op.annotations.iter().zip(tp.annotations.iter()).enumerate() {
            assert_eq!(oa.exp, ta.exp, "{} prompt {} annotation {}: different exp", test_name, i, j);
            assert_eq!(oa.span, ta.span, "{} prompt {} annotation {}: different span", test_name, i, j);
        }
    }
}

#[test]
fn compare_detect_const_name() {
    let src = r#"const userPrompt = "You are a helpful assistant.";"#;
    compare_parsers(src, "prompts.ts", "detect_const_name");
}

#[test]
fn compare_detect_let_name() {
    let src = r#"let userPrompt = "You are a helpful assistant.";"#;
    compare_parsers(src, "prompts.ts", "detect_let_name");
}

#[test]
fn compare_detect_var_name() {
    let src = r#"var userPrompt = "You are a helpful assistant.";"#;
    compare_parsers(src, "prompts.ts", "detect_var_name");
}

#[test]
fn compare_detect_inline() {
    let src = r#"const greeting = /* @prompt */ `Welcome ${user}!`;"#;
    compare_parsers(src, "prompts.ts", "detect_inline");
}

#[test]
fn compare_detect_inline_jsdoc() {
    let src = r#"const msg = /** @prompt */ "Hello world";"#;
    compare_parsers(src, "prompts.ts", "detect_inline_jsdoc");
}

#[test]
fn compare_detect_var_comment() {
    let src = indoc! {r#"
        // @prompt
        const hello = `Hello, world!`;
    "#};
    compare_parsers(src, "prompts.ts", "detect_var_comment");
}

#[test]
fn compare_detect_var_inline() {
    let src = indoc! {r#"
        /* @prompt */
        const hello = `Hello, world!`;
    "#};
    compare_parsers(src, "prompts.ts", "detect_var_inline");
}

#[test]
fn compare_detect_var_jsdoc() {
    let src = indoc! {r#"
        /** @prompt */
        const hello = `Hello, world!`;
    "#};
    compare_parsers(src, "prompts.ts", "detect_var_jsdoc");
}

#[test]
fn compare_detect_var_comment_spaced() {
    let src = indoc! {r#"
        // @prompt


        const hello = `Hello, world!`;

        // @prompt
        nope()

        const world = "Hello!";
    "#};
    compare_parsers(src, "prompts.ts", "detect_var_comment_spaced");
}

#[test]
fn compare_detect_var_comment_mixed() {
    let src = indoc! {r#"
        // @prompt
        const number = 42;

        const hello = "Hello, world!"
    "#};
    compare_parsers(src, "prompts.ts", "detect_var_comment_mixed");
}

#[test]
fn compare_detect_var_comment_mixed_nested() {
    let src = indoc! {r#"
        class Hello {
            world(self) {
                // @prompt
                let hello = 42;

                // @prompt
                let hi = 42;

                hi = "Hi!"
            }
        }

        hello = "Hello, world!";
    "#};
    compare_parsers(src, "prompts.ts", "detect_var_comment_mixed_nested");
}

#[test]
fn compare_multiline_annotation() {
    let src = indoc! {r#"
        // Hello
        // @prompt
        // world
        const msg = "Hello";
    "#};
    compare_parsers(src, "prompts.ts", "multiline_annotation");
}

#[test]
fn compare_detect_var_comment_none() {
    let src = r#"// @prompting
const hello = `Hello, world!`;"#;
    compare_parsers(src, "prompts.ts", "detect_var_comment_none");
}

#[test]
fn compare_detect_multi() {
    let src = indoc! {r#"
        const userPrompt = `Hello, ${name}!`;
        const greeting = /* @prompt */ `Welcome ${user}!`;
        // @prompt
        const farewell = `Goodbye ${user.name}!`;
        /** @prompt */
        const system = "You are an AI assistant";
        const regular = `Not a prompt ${value}`;
    "#};
    compare_parsers(src, "prompts.ts", "detect_multi");
}

#[test]
fn compare_detect_assign_var_comment() {
    let src = indoc! {r#"
        // @prompt
        let hello;
        hello = `Assigned ${value}`;
    "#};
    compare_parsers(src, "prompts.ts", "detect_assign_var_comment");
}

#[test]
fn compare_detect_reassign_var_comment() {
    let src = indoc! {r#"
        // @prompt
        let hello;
        hello = 123;

        hello = `Assigned ${value}`;
    "#};
    compare_parsers(src, "prompts.ts", "detect_reassign_var_comment");
}

#[test]
fn compare_detect_none() {
    let src = indoc! {r#"
        const regularTemplate = `This is not a ${value}`;
        const normalString = "This is not special";
        const regular = `Regular template with ${variable}`;
        const message = "Just a message";
        // @prompt
        const number = 1;
    "#};
    compare_parsers(src, "prompts.ts", "detect_none");
}

#[test]
fn compare_detect_single_var() {
    let src = r#"const userPrompt = `Hello, ${name}!`;"#;
    compare_parsers(src, "prompts.ts", "detect_single_var");
}

#[test]
fn compare_detect_multi_vars() {
    let src = r#"const userPrompt = `Hello, ${name}! How is the weather today in ${city}?`;"#;
    compare_parsers(src, "prompts.ts", "detect_multi_vars");
}

#[test]
fn compare_parse_exp_vars() {
    let src = r#"const userPrompt = `Hello, ${user.name}! How is the weather today in ${user.location.city}?`;"#;
    compare_parsers(src, "prompts.ts", "parse_exp_vars");
}

#[test]
fn compare_parse_exp_vars_complex() {
    let src = r#"const userPrompt = `Hello, ${User.fullName({ ...user.name, last: null })}!`;"#;
    compare_parsers(src, "prompts.ts", "parse_exp_vars_complex");
}

#[test]
fn compare_handle_invalid_syntax() {
    let src = r#"const invalid = `unclosed template"#;
    compare_parsers(src, "prompts.ts", "handle_invalid_syntax");
}

#[test]
fn compare_parse_js_code() {
    let src = r#"const prompt = /* @prompt */ `Hello ${world}!`;"#;
    compare_parsers(src, "test.js", "parse_js_code");
}

#[test]
fn compare_parse_jsx_code() {
    let src = indoc! {r#"
        const prompt = /* @prompt */ `Hello ${world}!`;
        const element = <div>{prompt}</div>;
    "#};
    compare_parsers(src, "test.jsx", "parse_jsx_code");
}

#[test]
fn compare_parse_ts_code() {
    let src = r#"const prompt : string = /* @prompt */ `Hello ${world}!`;"#;
    compare_parsers(src, "test.ts", "parse_ts_code");
}

#[test]
fn compare_parse_tsx_code() {
    let src = indoc! {r#"
        const prompt : string = /* @prompt */ `Hello ${world}!`;
        const element = <div>{prompt}</div>;
    "#};
    compare_parsers(src, "test.tsx", "parse_tsx_code");
}

#[test]
fn compare_parse_nested() {
    let src = indoc! {r#"
        class Hello {
            world(self) {
                const fn = () => {
                    const helloPrompt = `Hello, ${name}!`;

                    // @prompt
                    const alsoPrmpt = "Hi!";
                };

                return fn;
            }
        }
    "#};
    compare_parsers(src, "prompts.ts", "parse_nested");
}

#[test]
fn compare_multi_annotations() {
    let src = indoc! {r#"
        // Hello, world
        const hello = /* @prompt */ "asd";
    "#};
    compare_parsers(src, "prompts.ts", "multi_annotations");
}

#[test]
fn compare_multiline_annotations() {
    let src = indoc! {r#"
        /*
         Multi
         Line
         Block
        */
        const hello = /* @prompt */ `x`;
    "#};
    compare_parsers(src, "prompts.ts", "multiline_annotations");
}

#[test]
fn compare_multiline_annotation_nested() {
    let src = indoc! {r#"
        function fn() {
            // Hello
            // @prompt
            // world
            const msg = "Hello";
        }
    "#};
    compare_parsers(src, "prompts.ts", "multiline_annotation_nested");
}

#[test]
fn compare_reassign_no_comment() {
    let src = indoc! {r#"
        // @prompt
        let hello: string;
        hello = `Hi`;
    "#};
    compare_parsers(src, "prompts.ts", "reassign_no_comment");
}

#[test]
fn compare_reassign_with_comment() {
    let src = indoc! {r#"
        // @prompt def
        let hello: string;
        // @prompt fresh
        hello = `Hi`;
    "#};
    compare_parsers(src, "prompts.ts", "reassign_with_comment");
}

#[test]
fn compare_reassign_with_comment_multi() {
    let src = indoc! {r#"
        // @prompt def
        let hello: string;
        // hello
        hello = /* @prompt fresh */ `Hi`;
    "#};
    compare_parsers(src, "prompts.ts", "reassign_with_comment_multi");
}
