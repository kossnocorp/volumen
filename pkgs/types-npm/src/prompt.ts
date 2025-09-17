export interface PromptVar {
  exp: string;
  span: import("./span.js").SpanShape;
}

export interface Prompt {
  file: string;
  span: import("./span.js").SpanShape;
  enclosure: import("./span.js").Span;
  exp: string;
  vars: Array<PromptVar>;
  annotations: Array<PromptAnnotation>;
}

export interface PromptAnnotation {
  span: import("./span.js").Span;
  exp: string;
}
