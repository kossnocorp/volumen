use indoc::indoc;
use pretty_assertions::assert_eq;
use volumen_types::*;
use volumen_wasm::parse_prompts;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
#[allow(dead_code)]
fn parse_js_value() {
    let js_source = indoc! {r#"
        const msg = /** @prompt */ "Hello, world!";
    "#};
    let js_result = parse_test_code_wasm(js_source, "prompts.js");
    assert_eq!(
        js_result,
        ParseResult::ParseResultSuccess(ParseResultSuccess {
            state: ParseResultSuccessStateSuccess,
            prompts: vec![Prompt {
                file: "prompts.js".into(),
                span: SpanShape {
                    outer: Span { start: 27, end: 40 },
                    inner: Span { start: 28, end: 39 },
                },
                enclosure: Span { start: 0, end: 41 },
                exp: "\"Hello, world\"".into(),
                vars: vec![],
                annotations: vec![PromptAnnotation {
                    span: Span { start: 12, end: 26 },
                    exp: "/** @prompt */".into(),
                }]
            }]
        })
    );
}

#[wasm_bindgen_test]
#[allow(dead_code)]
fn parse_py_value() {
    let py_source = indoc! {r#"
        # @prompt
        msg = f"Hello {name}!"
    "# };
    let py_result = parse_test_code_wasm(py_source, "prompts.py");
    assert_eq!(
        py_result,
        ParseResult::ParseResultSuccess(ParseResultSuccess {
            state: ParseResultSuccessStateSuccess,
            prompts: vec![Prompt {
                file: "prompts.py".into(),
                span: SpanShape {
                    outer: Span { start: 16, end: 32 },
                    inner: Span { start: 18, end: 31 }
                },
                enclosure: Span { start: 0, end: 32 },
                exp: "f\"Hello {name}!\"".into(),
                vars: vec![PromptVar {
                    exp: "{name}".into(),
                    span: SpanShape {
                        outer: Span { start: 24, end: 30 },
                        inner: Span { start: 25, end: 29 },
                    }
                }],
                annotations: vec![PromptAnnotation {
                    span: Span { start: 0, end: 9 },
                    exp: "# @prompt".into(),
                }]
            }]
        })
    );
}

#[wasm_bindgen_test]
#[allow(dead_code)]
fn parse_ts_value() {
    let ts_source = indoc! {r#"
        const prompt = "Hello, {name}!";
    "#};
    let result = parse_test_code_wasm(ts_source, "prompts.ts");
    assert_prompt_count(result, 1);
}

#[wasm_bindgen_test]
#[allow(dead_code)]
fn parse_rb_value() {
    let rb_source = indoc! {r#"
        prompt = "Hello, {name}!"
    "#};
    let result = parse_test_code_wasm(rb_source, "prompts.rb");
    assert_prompt_count(result, 1);
}

#[wasm_bindgen_test]
#[allow(dead_code)]
fn parse_php_value() {
    let php_source = indoc! {r#"
        <?php
        $prompt = "Hello, {name}!";
    "#};
    let result = parse_test_code_wasm(php_source, "prompts.php");
    assert_prompt_count(result, 1);
}

#[wasm_bindgen_test]
#[allow(dead_code)]
fn parse_cs_value() {
    let cs_source = indoc! {r#"
        using System;

        class Example {
            string prompt = "Hello, {name}!";
        }
    "#};
    let result = parse_test_code_wasm(cs_source, "prompts.cs");
    assert_prompt_count(result, 0);
}

#[wasm_bindgen_test]
#[allow(dead_code)]
fn parse_go_value() {
    let go_source = indoc! {r#"
        package main

        var prompt = "Hello, {name}!"
    "#};
    let result = parse_test_code_wasm(go_source, "prompts.go");
    assert_prompt_count(result, 0);
}

#[wasm_bindgen_test]
#[allow(dead_code)]
fn parse_java_value() {
    let java_source = indoc! {r#"
        class Example {
            String prompt = "Hello, {name}!";
        }
    "#};
    let result = parse_test_code_wasm(java_source, "Prompts.java");
    assert_prompt_count(result, 0);
}

#[wasm_bindgen_test]
#[allow(dead_code)]
fn parse_unsupported_extension() {
    let result = parse_test_code_wasm("prompt = \"Hello\"", "prompts.txt");
    match result {
        ParseResult::ParseResultError(ParseResultError { state, error }) => {
            assert_eq!(state, ParseResultErrorStateError);
            assert_eq!(error, "Unsupported file extension for file: prompts.txt");
        }
        _ => panic!("Expected ParseResultError for unsupported extension"),
    }
}

fn assert_prompt_count(result: ParseResult, expected: usize) {
    match result {
        ParseResult::ParseResultSuccess(ParseResultSuccess { prompts, .. }) => {
            assert_eq!(prompts.len(), expected);
        }
        ParseResult::ParseResultError(ParseResultError { error, .. }) => {
            panic!("Parsing failed: {error}");
        }
    }
}

fn parse_test_code_wasm(source: &str, filename: &str) -> ParseResult {
    let result = parse_prompts(source, filename).expect("Parsing should succeed");
    let parse_result: ParseResult =
        serde_wasm_bindgen::from_value(result).expect("Deserialization should succeed");
    parse_result
}
