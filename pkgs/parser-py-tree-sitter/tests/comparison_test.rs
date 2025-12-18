// Comparison tests between Tree-sitter, RustPython, and Ruff parsers
// This ensures all parsers produce identical results

use indoc::indoc;
use volumen_parser_py::ParserPy as ParserPyRustPython;
use volumen_parser_py_ruff::ParserPy as ParserPyRuff;
use volumen_parser_py_tree_sitter::ParserPy as ParserPyTreeSitter;

fn compare_parsers(source: &str, filename: &str, test_name: &str) {
    let rustpython_result = ParserPyRustPython::parse(source, filename);
    let treesitter_result = ParserPyTreeSitter::parse(source, filename);
    let ruff_result = ParserPyRuff::parse(source, filename);

    // All should succeed or fail in the same way
    let rustpython_prompts = match rustpython_result {
        volumen_types::ParseResult::ParseResultSuccess(s) => s.prompts,
        volumen_types::ParseResult::ParseResultError(_e) => {
            // If RustPython fails, others should also fail
            assert!(
                matches!(treesitter_result, volumen_types::ParseResult::ParseResultError(_)),
                "{}: RustPython failed but Tree-sitter succeeded", test_name
            );
            assert!(
                matches!(ruff_result, volumen_types::ParseResult::ParseResultError(_)),
                "{}: RustPython failed but Ruff succeeded", test_name
            );
            return;
        }
    };

    let treesitter_prompts = match treesitter_result {
        volumen_types::ParseResult::ParseResultSuccess(s) => s.prompts,
        volumen_types::ParseResult::ParseResultError(e) => {
            panic!("{}: Tree-sitter failed but RustPython succeeded: {}", test_name, e.error);
        }
    };

    let ruff_prompts = match ruff_result {
        volumen_types::ParseResult::ParseResultSuccess(s) => s.prompts,
        volumen_types::ParseResult::ParseResultError(e) => {
            panic!("{}: Ruff failed but RustPython succeeded: {}", test_name, e.error);
        }
    };

    // Compare prompt counts (RustPython vs Tree-sitter)
    assert_eq!(
        rustpython_prompts.len(),
        treesitter_prompts.len(),
        "{}: Different number of prompts. RustPython: {}, Tree-sitter: {}",
        test_name,
        rustpython_prompts.len(),
        treesitter_prompts.len()
    );

    // Compare prompt counts (RustPython vs Ruff)
    assert_eq!(
        rustpython_prompts.len(),
        ruff_prompts.len(),
        "{}: Different number of prompts. RustPython: {}, Ruff: {}",
        test_name,
        rustpython_prompts.len(),
        ruff_prompts.len()
    );

    // Compare each prompt across all three parsers
    for (i, ((rp, tp), rfp)) in rustpython_prompts.iter()
        .zip(treesitter_prompts.iter())
        .zip(ruff_prompts.iter())
        .enumerate() {
        // RustPython vs Tree-sitter
        assert_eq!(rp.file, tp.file, "{} prompt {}: RustPython vs Tree-sitter different file", test_name, i);
        assert_eq!(rp.exp, tp.exp, "{} prompt {}: RustPython vs Tree-sitter different exp", test_name, i);
        assert_eq!(rp.span, tp.span, "{} prompt {}: RustPython vs Tree-sitter different span", test_name, i);
        assert_eq!(rp.enclosure, tp.enclosure, "{} prompt {}: RustPython vs Tree-sitter different enclosure", test_name, i);
        assert_eq!(rp.vars.len(), tp.vars.len(), "{} prompt {}: RustPython vs Tree-sitter different vars count", test_name, i);

        // RustPython vs Ruff
        assert_eq!(rp.file, rfp.file, "{} prompt {}: RustPython vs Ruff different file", test_name, i);
        assert_eq!(rp.exp, rfp.exp, "{} prompt {}: RustPython vs Ruff different exp", test_name, i);
        assert_eq!(rp.span, rfp.span, "{} prompt {}: RustPython vs Ruff different span", test_name, i);
        assert_eq!(rp.enclosure, rfp.enclosure, "{} prompt {}: RustPython vs Ruff different enclosure", test_name, i);
        assert_eq!(rp.vars.len(), rfp.vars.len(), "{} prompt {}: RustPython vs Ruff different vars count", test_name, i);

        // Compare vars (RustPython vs Tree-sitter vs Ruff)
        for (j, ((rv, tv), rfv)) in rp.vars.iter()
            .zip(tp.vars.iter())
            .zip(rfp.vars.iter())
            .enumerate() {
            assert_eq!(rv.exp, tv.exp, "{} prompt {} var {}: RustPython vs Tree-sitter different exp", test_name, i, j);
            assert_eq!(rv.span, tv.span, "{} prompt {} var {}: RustPython vs Tree-sitter different span", test_name, i, j);
            assert_eq!(rv.exp, rfv.exp, "{} prompt {} var {}: RustPython vs Ruff different exp", test_name, i, j);
            assert_eq!(rv.span, rfv.span, "{} prompt {} var {}: RustPython vs Ruff different span", test_name, i, j);
        }

        // Compare annotations (RustPython vs Tree-sitter)
        assert_eq!(rp.annotations.len(), tp.annotations.len(), "{} prompt {}: RustPython vs Tree-sitter different annotations count", test_name, i);

        // Compare annotations (RustPython vs Ruff)
        assert_eq!(rp.annotations.len(), rfp.annotations.len(), "{} prompt {}: RustPython vs Ruff different annotations count", test_name, i);

        for (j, ((ra, ta), rfa)) in rp.annotations.iter()
            .zip(tp.annotations.iter())
            .zip(rfp.annotations.iter())
            .enumerate() {
            assert_eq!(ra.exp, ta.exp, "{} prompt {} annotation {}: RustPython vs Tree-sitter different exp", test_name, i, j);
            assert_eq!(ra.span, ta.span, "{} prompt {} annotation {}: RustPython vs Tree-sitter different span", test_name, i, j);
            assert_eq!(ra.exp, rfa.exp, "{} prompt {} annotation {}: RustPython vs Ruff different exp", test_name, i, j);
            assert_eq!(ra.span, rfa.span, "{} prompt {} annotation {}: RustPython vs Ruff different span", test_name, i, j);
        }
    }
}

#[test]
fn compare_detect_none() {
    let src = indoc! {r#"
        hello = 42
    "#};
    compare_parsers(src, "prompts.py", "detect_none");
}

#[test]
fn compare_detect_var_name_str() {
    let src = indoc! {r#"
        user_prompt = "You are a helpful assistant."
    "#};
    compare_parsers(src, "prompts.py", "detect_var_name_str");
}

#[test]
fn compare_detect_var_name_fstr() {
    let src = indoc! {r#"
        user_prompt = f"You are a {role}."
    "#};
    compare_parsers(src, "prompts.py", "detect_var_name_fstr");
}

#[test]
fn compare_detect_var_comment_str() {
    let src = indoc! {r#"
        # @prompt
        hello = "You are a helpful assistant."
    "#};
    compare_parsers(src, "prompts.py", "detect_var_comment_str");
}

#[test]
fn compare_detect_var_comment_fstr() {
    let src = indoc! {r#"
        # @prompt
        hello = f"You are a {role}."
    "#};
    compare_parsers(src, "prompts.py", "detect_var_comment_fstr");
}

#[test]
fn compare_detect_var_comment_dirty_str() {
    let src = indoc! {r#"
        # @prompt this is my special prompt
        hello = "You are a helpful assistant."
    "#};
    compare_parsers(src, "prompts.py", "detect_var_comment_dirty_str");
}

#[test]
fn compare_detect_var_comment_dirty_fstr() {
    let src = indoc! {r#"
        # @prompt this is my special prompt
        hello = f"You are a {role}."
    "#};
    compare_parsers(src, "prompts.py", "detect_var_comment_dirty_fstr");
}

#[test]
fn compare_detect_var_comment_spaced() {
    let src = indoc! {r#"
        # This is a comment
        # @prompt this is my special prompt
        # This is another comment
        hello = f"You are a {role}."
    "#};
    compare_parsers(src, "prompts.py", "detect_var_comment_spaced");
}

#[test]
fn compare_detect_var_comment_mixed() {
    let src = indoc! {r#"
        # @prompt
        hello = "You are a helpful assistant."
        goodbye = "Goodbye!"
    "#};
    compare_parsers(src, "prompts.py", "detect_var_comment_mixed");
}

#[test]
fn compare_detect_var_comment_mixed_nested() {
    let src = indoc! {r#"
        class Hello:
            def world(self):
                # @prompt
                hello = 42

                # @prompt
                hi = 42

                hi = "Hi!"

        hello = "Hello, world!"
    "#};
    compare_parsers(src, "prompts.py", "detect_var_comment_mixed_nested");
}

#[test]
fn compare_detect_var_comment_none() {
    let src = indoc! {r#"
        # @prompting
        hello = f"Hello, world!"
    "#};
    compare_parsers(src, "prompts.py", "detect_var_comment_none");
}

#[test]
fn compare_detect_assign_var_comment() {
    let src = indoc! {r#"
        # @prompt
        hello : str
        hello = f"Assigned {value}"
    "#};
    compare_parsers(src, "prompts.py", "detect_assign_var_comment");
}

#[test]
fn compare_detect_reassign_var_comment() {
    let src = indoc! {r#"
        # @prompt
        hello : str
        hello = 42
        hello = f"Assigned {value}"
    "#};
    compare_parsers(src, "prompts.py", "detect_reassign_var_comment");
}

#[test]
fn compare_reassign_with_comment() {
    let src = indoc! {r#"
        # @prompt
        hello : str
        hello = 42
        # @prompt fresh
        hello = f"Assigned {value}"
    "#};
    compare_parsers(src, "prompts.py", "reassign_with_comment");
}

#[test]
fn compare_reassign_no_comment() {
    let src = indoc! {r#"
        hello: str
        # @prompt def
        hello = 42
        hello = f"Assigned {value}"
    "#};
    compare_parsers(src, "prompts.py", "reassign_no_comment");
}

#[test]
fn compare_detect_multi_vars() {
    let src = indoc! {r#"
        user_prompt = "You are a helpful assistant."
        system_prompt = "You are an AI."
    "#};
    compare_parsers(src, "prompts.py", "detect_multi_vars");
}

#[test]
fn compare_detect_multi_assign_var_comment() {
    let src = indoc! {r#"
        # @prompt
        a, b = ["Hello", "World"]
    "#};
    compare_parsers(src, "prompts.py", "detect_multi_assign_var_comment");
}

#[test]
fn compare_detect_multi_assign_variants() {
    let src = indoc! {r#"
        # @prompt
        (c, d) = ["Tuple", "Test"]
        # @prompt
        [e, f] = ["List", "Test"]
    "#};
    compare_parsers(src, "prompts.py", "detect_multi_assign_variants");
}

#[test]
fn compare_detect_chained_assign() {
    let src = indoc! {r#"
        # @prompt
        a = b = "Chained"
    "#};
    compare_parsers(src, "prompts.py", "detect_chained_assign");
}

#[test]
fn compare_parse_spans_str() {
    let src = indoc! {r#"
        user_prompt = "You are a helpful assistant."
    "#};
    compare_parsers(src, "prompts.py", "parse_spans_str");
}

#[test]
fn compare_parse_spans_fstr() {
    let src = indoc! {r#"
        user_prompt = f"You are a {role}."
    "#};
    compare_parsers(src, "prompts.py", "parse_spans_fstr");
}

#[test]
fn compare_parse_spans_multiline_str() {
    let src = indoc! {r#"
        user_prompt = """
        You are a helpful assistant.
        """
    "#};
    compare_parsers(src, "prompts.py", "parse_spans_multiline_str");
}

#[test]
fn compare_parse_spans_multiline_fstr() {
    let src = indoc! {r#"
        user_prompt = f"""
        You are a {role}.
        """
    "#};
    compare_parsers(src, "prompts.py", "parse_spans_multiline_fstr");
}

#[test]
fn compare_parse_exp_vars() {
    let src = indoc! {r#"
        user_prompt = f"You are a {role}."
    "#};
    compare_parsers(src, "prompts.py", "parse_exp_vars");
}

#[test]
fn compare_parse_exp_complex_vars() {
    let src = indoc! {r#"
        user_prompt = f"Hello {first} {last}!"
    "#};
    compare_parsers(src, "prompts.py", "parse_exp_complex_vars");
}

#[test]
fn compare_parse_multiline_str() {
    let src = indoc! {r#"
        user_prompt = """You are a helpful assistant."""
    "#};
    compare_parsers(src, "prompts.py", "parse_multiline_str");
}

#[test]
fn compare_parse_multiline_fstr() {
    let src = indoc! {r#"
        user_prompt = f"""You are a {role}."""
    "#};
    compare_parsers(src, "prompts.py", "parse_multiline_fstr");
}

#[test]
fn compare_parse_nested() {
    let src = indoc! {r#"
        class Agent:
            def __init__(self):
                self.user_prompt = "You are a helpful assistant."
    "#};
    compare_parsers(src, "prompts.py", "parse_nested");
}

#[test]
fn compare_multiline_annotation() {
    let src = indoc! {r#"
        # This is a comment
        # @prompt
        # Another comment
        hello = "Hello, world!"
    "#};
    compare_parsers(src, "prompts.py", "multiline_annotation");
}

#[test]
fn compare_multiline_annotation_nested() {
    let src = indoc! {r#"
        def foo():
            # Comment 1
            # @prompt
            # Comment 2
            bar = "Hello, world!"
    "#};
    compare_parsers(src, "prompts.py", "multiline_annotation_nested");
}

#[test]
fn compare_handle_invalid_syntax() {
    let src = indoc! {r#"
        user_prompt = "Unterminated string
    "#};
    compare_parsers(src, "prompts.py", "handle_invalid_syntax");
}
