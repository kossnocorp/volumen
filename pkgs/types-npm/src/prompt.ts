export interface PromptVar {
  span: import("./span.js").SpanShape;
}

export interface Prompt {
  file: string;
  /** Enclosure span pointing to the prompt with associated expression, e.g.,
   * for `const prompt = "Hi!";`, the enclosure spans from `const` to `;`.
   * It allows to find prompts related to a specific cursor position. */
  enclosure: import("./span.js").Span;
  /** Prompt expression span shape, e.g., for `const prompt = "Hi!";`,
   * the outer span includes `"Hi!"` and the inner spans the string content
   * `Hi!` without quotes. The inner span musn't be used to extract the
   * expression text, and only serves to locate content surroundings, i.e.,
   * `"` characters in this case. */
  span: import("./span.js").SpanShape;
  /** Content tokens that defines the prompt chunks. Simple prompts will have
   * a single string token. More complex prompts with variable interpolations
   * or prompts defined as joined array, or indentation-altered multi-line
   * strings (e.g., heredoc in Ruby) will have multiple tokens. The tokens
   * can be used to render the prompt accurately as defined in source. */
  content: Array<PromptContentToken>;
  /** Expression joint span shape, e.g., for `["foo", "bar"].join(", ")`, the
   * joint is `", "` with outer covering the entire expression and inner the
   * span of the joint itself i.e., `, `. The internal span can be used to
   * render the prompt content joint tokens. */
  joint: import("./span.js").SpanShape;
  /** Variables used in the prompt expression. The order corresponds to
   * the order of appearance in the prompt content tokens. */
  vars: Array<PromptVar>;
  /** Associated annotations found in comments related to the prompt. Each
   * annotation represents a continuous block of comment lines or a single
   * comment chunk. The order corresponds to the order of appearance in
   * the source code. */
  annotations: Array<PromptAnnotation>;
}

export type PromptContentToken = PromptContentTokenStr | PromptContentTokenVar | PromptContentTokenJoint;

export interface PromptContentTokenStr {
  type: "str";
  span: import("./span.js").Span;
}

export interface PromptContentTokenVar {
  type: "var";
  span: import("./span.js").Span;
  index: number;
}

export interface PromptContentTokenJoint {
  type: "joint";
}

export interface PromptAnnotation {
  /** Prompt annotation span shapes. Each span shape represents a line as it
   * appears in source code. The outer span covers the entire comment line
   * e.g., `// @prompt hello`, and the inner span covers just the content,
   * e.g., ` @prompt hello`. */
  spans: Array<import("./span.js").SpanShape>;
}
