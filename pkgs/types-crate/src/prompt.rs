use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptVar {
    pub exp: String,
    pub span: super::span::SpanShape,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
    pub file: String,
    pub span: super::span::SpanShape,
    pub enclosure: super::span::Span,
    pub exp: String,
    pub vars: Vec<PromptVar>,
    pub annotations: Vec<PromptAnnotation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptAnnotation {
    /// Prompt annotation span shapes. Each span shape represents a line as it
    /// appears in source code. The outer span covers the entire comment line
    /// e.g., `// @prompt hello`, and the inner span covers just the content,
    /// e.g., ` @prompt hello`.
    pub spans: Vec<super::span::SpanShape>,
    /// @deprecated, use `span` tokens to render the prompt content.
    pub exp: String,
}
