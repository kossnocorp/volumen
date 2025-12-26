use pretty_assertions::assert_eq;
use volumen_types::*;

pub fn assert_prompts_size(result: ParseResult, expected_size: usize) {
    match result {
        ParseResult::ParseResultSuccess(ParseResultSuccess { prompts, .. }) => {
            assert_eq!(prompts.len(), expected_size);
        }
        ParseResult::ParseResultError(ParseResultError { error, .. }) => {
            panic!("Parsing failed: {}", error)
        }
    }
}

pub fn assert_prompt_spans(src: &str, result: ParseResult) {
    match result {
        ParseResult::ParseResultSuccess(ParseResultSuccess { prompts, .. }) => {
            assert!(
                prompts.len() > 0,
                "Expected at least one prompt inside ParseResult"
            );

            for prompt in &prompts {
                assert_eq!(
                    &src[prompt.span.outer.0 as usize..prompt.span.outer.1 as usize],
                    prompt.exp
                );

                for var in &prompt.vars {
                    assert_eq!(
                        &src[var.span.outer.0 as usize..var.span.outer.1 as usize],
                        var.exp
                    );
                }
            }
        }

        _ => panic!("Expected ParseResultSuccess"),
    }
}
