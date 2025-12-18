const PROMPT_LENGTH: usize = "@prompt".len();

/// Parses annotation text to determine if it contains a valid @prompt marker.
pub fn parse_annotation(annotation: &str) -> Option<bool> {
    let comment = annotation.to_lowercase();
    if let Some(pos) = comment.find("@prompt") {
        let before_char = if pos == 0 {
            None
        } else {
            comment.chars().nth(pos - 1)
        };

        let after_pos = pos + PROMPT_LENGTH;
        let after_char = comment.chars().nth(after_pos);

        let valid_before = before_char.map_or(true, |c| !c.is_alphanumeric() && c != '_');
        let valid_after = after_char.map_or(true, |c| !c.is_alphanumeric() && c != '_');

        Some(valid_before && valid_after)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic() {
        assert_eq!(parse_annotation("@prompt"), Some(true));
        assert_eq!(parse_annotation(" @prompt "), Some(true));
        assert_eq!(parse_annotation("@prompt greeting"), Some(true));
        assert_eq!(parse_annotation("greeting @prompt"), Some(true));
    }

    #[test]
    fn extra() {
        assert_eq!(parse_annotation("@prompt for user greeting"), Some(true));
        assert_eq!(parse_annotation("This is a @prompt comment"), Some(true));
        assert_eq!(parse_annotation("* @prompt"), Some(true));
        assert_eq!(parse_annotation("*@prompt"), Some(true));
        assert_eq!(parse_annotation("* @prompt greeting"), Some(true));
        assert_eq!(parse_annotation("  * @prompt  "), Some(true));
    }

    #[test]
    fn case() {
        assert_eq!(parse_annotation("@PROMPT"), Some(true));
        assert_eq!(parse_annotation("@Prompt"), Some(true));
        assert_eq!(parse_annotation("@PrOmPt"), Some(true));
        assert_eq!(parse_annotation("* @PROMPT"), Some(true));
        assert_eq!(parse_annotation("@PROMPT for testing"), Some(true));
        assert_eq!(parse_annotation("@Prompt with mixed case"), Some(true));
    }

    #[test]
    fn inexact() {
        assert_eq!(parse_annotation("@prompting"), Some(false));
        assert_eq!(parse_annotation("my@prompt"), Some(false));
        assert_eq!(parse_annotation("@prompter"), Some(false));
        assert_eq!(parse_annotation("@prompt_var"), Some(false));
        assert_eq!(parse_annotation("* @prompting"), Some(false));
        assert_eq!(parse_annotation("* my@prompt"), Some(false));
    }

    #[test]
    fn unrelated_text() {
        assert_eq!(parse_annotation("regular comment"), None);
        assert_eq!(parse_annotation("* This is documentation"), None);
        assert_eq!(parse_annotation("TODO: fix this"), None);
        assert_eq!(parse_annotation(""), None);
        assert_eq!(parse_annotation("   "), None);
    }

    #[test]
    fn punctuation() {
        assert_eq!(parse_annotation("@prompt!"), Some(true));
        assert_eq!(parse_annotation("@prompt."), Some(true));
        assert_eq!(parse_annotation("@prompt,"), Some(true));
        assert_eq!(parse_annotation("(@prompt)"), Some(true));
    }

    #[test]
    fn repeating() {
        assert_eq!(parse_annotation("@prompt for @prompt usage"), Some(true));
    }
}
