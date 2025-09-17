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
    pub span: super::span::Span,
    pub exp: String,
}
