use serde::{Deserialize, Serialize};

pub type Span = (u32, u32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpanShape {
    pub outer: Span,
    pub inner: Span,
}
