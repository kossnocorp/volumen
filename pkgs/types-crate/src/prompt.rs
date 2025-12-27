use serde::{Deserialize, Serialize};
use litty::literal;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptVar {
    pub span: super::span::SpanShape,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
    pub file: String,
    /// Enclosure span pointing to the prompt with associated expression, e.g.,
    /// for `const prompt = "Hi!";`, the enclosure spans from `const` to `;`.
    /// It allows to find prompts related to a specific cursor position.
    pub enclosure: super::span::Span,
    /// Prompt expression span shape, e.g., for `const prompt = "Hi!";`,
    /// the outer span includes `"Hi!"` and the inner spans the string content
    /// `Hi!` without quotes. The inner span musn't be used to extract the
    /// expression text, and only serves to locate content surroundings, i.e.,
    /// `"` characters in this case.
    pub span: super::span::SpanShape,
    /// Content tokens that defines the prompt chunks. Simple prompts will have
    /// a single string token. More complex prompts with variable interpolations
    /// or prompts defined as joined array, or indentation-altered multi-line
    /// strings (e.g., heredoc in Ruby) will have multiple tokens. The tokens
    /// can be used to render the prompt accurately as defined in source.
    pub content: Vec<PromptContentToken>,
    /// Expression joint span shape, e.g., for `["foo", "bar"].join(", ")`, the
    /// joint is `", "` with outer covering the entire expression and inner the
    /// span of the joint itself i.e., `, `. The internal span can be used to
    /// render the prompt content joint tokens.
    pub joint: super::span::SpanShape,
    /// Variables used in the prompt expression. The order corresponds to
    /// the order of appearance in the prompt content tokens.
    pub vars: Vec<PromptVar>,
    /// Associated annotations found in comments related to the prompt. Each
    /// annotation represents a continuous block of comment lines or a single
    /// comment chunk. The order corresponds to the order of appearance in
    /// the source code.
    pub annotations: Vec<PromptAnnotation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PromptContentToken {
    PromptContentTokenStr(PromptContentTokenStr),
    PromptContentTokenVar(PromptContentTokenVar),
    PromptContentTokenJoint(PromptContentTokenJoint),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptContentTokenStr {
    pub r#type: PromptContentTokenStrTypeStr,
    pub span: super::span::Span,
}

#[literal("str")]
pub struct PromptContentTokenStrTypeStr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptContentTokenVar {
    pub r#type: PromptContentTokenVarTypeVar,
    pub span: super::span::Span,
}

#[literal("var")]
pub struct PromptContentTokenVarTypeVar;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptContentTokenJoint {
    pub r#type: PromptContentTokenJointTypeJoint,
}

#[literal("joint")]
pub struct PromptContentTokenJointTypeJoint;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptAnnotation {
    /// Prompt annotation span shapes. Each span shape represents a line as it
    /// appears in source code. The outer span covers the entire comment line
    /// e.g., `// @prompt hello`, and the inner span covers just the content,
    /// e.g., ` @prompt hello`.
    pub spans: Vec<super::span::SpanShape>,
}
