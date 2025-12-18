use std::collections::{HashMap, HashSet};

use oxc_allocator::Allocator;
use oxc_allocator::Vec as OxcVec;
use oxc_ast::{Comment, Visit, ast};
use oxc_parser::{ParseOptions, Parser};
use oxc_span::{GetSpan, SourceType};
pub use volumen_parser_core::VolumenParser;
use volumen_parser_core::*;
use volumen_types::*;

pub struct ParserTs {}

impl VolumenParser for ParserTs {
    fn parse(source: &str, filename: &str) -> ParseResult {
        let allocator = Allocator::default();

        let source_type =
            SourceType::from_path(filename).unwrap_or(SourceType::default().with_typescript(true));

        let parser_return = Parser::new(&allocator, source, source_type)
            .with_options(ParseOptions::default())
            .parse();

        if !parser_return.errors.is_empty() {
            let error_messages: Vec<String> = parser_return
                .errors
                .iter()
                .map(|e| format!("{}", e))
                .collect();

            return ParseResult::ParseResultError(ParseResultError {
                state: ParseResultErrorStateError,
                error: error_messages.join("; "),
            });
        }

        let mut visitor = PromptVisitor::new(
            source,
            filename.to_string(),
            &parser_return.program.comments,
        );
        visitor.visit_program(&parser_return.program);

        ParseResult::ParseResultSuccess(ParseResultSuccess {
            state: ParseResultSuccessStateSuccess,
            prompts: visitor.prompts,
        })
    }
}

struct PromptVisitor<'a> {
    /// Source code being analyzed.
    code: &'a str,
    /// Source code file path.
    file: String,
    /// Collected prompts.
    prompts: Vec<Prompt>,
    /// Stack of identifier sets with prompt annotations.
    prompt_idents_stack: Vec<HashSet<String>>,
    /// Parsed comments.
    comments: &'a OxcVec<'a, Comment>,
    /// Stack of current statement spans (VariableDeclaration/ExpressionStatement)
    stmt_span_stack: Vec<oxc_span::Span>,
    /// Stack of annotations collected for the current statement
    stmt_annotations_stack: Vec<Vec<PromptAnnotation>>,
    /// Earliest leading annotation start for current statement
    stmt_leading_start_stack: Vec<Option<u32>>,
    /// Per-scope map of identifier -> def-time annotations
    def_prompt_annotations_stack: Vec<HashMap<String, Vec<PromptAnnotation>>>,
}

impl<'a> PromptVisitor<'a> {
    fn new(code: &'a str, file: String, comments: &'a OxcVec<'a, Comment>) -> Self {
        Self {
            code,
            file,
            prompts: Vec::new(),
            prompt_idents_stack: vec![HashSet::new()],
            comments,
            stmt_span_stack: Vec::new(),
            stmt_annotations_stack: Vec::new(),
            stmt_leading_start_stack: Vec::new(),
            def_prompt_annotations_stack: vec![HashMap::new()],
        }
    }

    fn span_outer(&self, span: &oxc_span::Span) -> Span {
        Span {
            start: span.start,
            end: span.end,
        }
    }

    fn span_shape_literal(&self, span: &oxc_span::Span) -> SpanShape {
        let outer = self.span_outer(span);
        let inner_start = outer.start.saturating_add(1);
        let inner_end = outer.end.saturating_sub(1);
        let inner = Span {
            start: inner_start,
            end: inner_end,
        };
        SpanShape { outer, inner }
    }

    fn process_variable_declarator(
        &mut self,
        declarator: &ast::VariableDeclarator<'a>,
        has_stmt_prompt: bool,
    ) {
        if let ast::BindingPatternKind::BindingIdentifier(ident) = &declarator.id.kind
            && has_stmt_prompt
        {
            if let Some(scope) = self.prompt_idents_stack.last_mut() {
                scope.insert(ident.name.to_string());
            }
            // Persist definition-time annotations for later reassignments
            if let Some(ann) = self.stmt_annotations_stack.last()
                && !ann.is_empty()
                && let Some(scope) = self.def_prompt_annotations_stack.last_mut()
            {
                scope.insert(ident.name.to_string(), ann.clone());
            }
        }

        // Detect type annotation regardless of initializer presence
        if let ast::BindingPatternKind::BindingIdentifier(ident) = &declarator.id.kind {
            let id_span = declarator.id.span();
            let decl_span = self
                .stmt_span_stack
                .last()
                .copied()
                .unwrap_or(declarator.span);
            let mut has_type_annotation = false;
            if id_span.start < id_span.end && decl_span.start < decl_span.end {
                let start = id_span.start as usize;
                let end = std::cmp::min(declarator.span.end as usize, self.code.len());
                if start < end {
                    let slice = &self.code[start..end];
                    if slice.contains(':') {
                        has_type_annotation = true;
                    }
                }
            }

            if has_type_annotation
                && let Some(ann) = self.stmt_annotations_stack.last()
                && !ann.is_empty()
                && let Some(scope) = self.def_prompt_annotations_stack.last_mut()
            {
                scope.insert(ident.name.to_string(), ann.clone());
            }

            if let Some(init) = &declarator.init {
                match init {
                    ast::Expression::TemplateLiteral(template) => {
                        self.process_template_literal(&ident.name, template);
                    }

                    ast::Expression::StringLiteral(string_literal) => {
                        self.process_string_literal(&ident.name, string_literal);
                    }

                    _ => {}
                }
            }
        }
    }

    fn process_assignment_expression(&mut self, expr: &ast::AssignmentExpression<'a>) {
        if let ast::AssignmentTarget::AssignmentTargetIdentifier(ident) = &expr.left {
            match &expr.right {
                ast::Expression::TemplateLiteral(template) => {
                    self.process_template_literal(&ident.name, template);
                }

                ast::Expression::StringLiteral(string_literal) => {
                    self.process_string_literal(&ident.name, string_literal);
                }

                _ => {}
            }
        }
    }

    fn extract_template_vars(&self, template: &ast::TemplateLiteral<'a>) -> Vec<PromptVar> {
        let mut vars = Vec::new();

        for expr in &template.expressions {
            let expr_span = expr.span();
            let mut start = expr_span.start.saturating_sub(2);
            let mut end = expr_span.end + 1;

            // Validate we actually have a "${" before and a "}" after; otherwise fallback
            let code_bytes = self.code.as_bytes();
            let valid = (start as usize + 1) < code_bytes.len()
                && (end as usize) <= code_bytes.len()
                && &self.code[start as usize..(start + 2) as usize] == "${"
                && code_bytes[(end - 1) as usize] == b'}';

            if !valid {
                start = expr_span.start;
                end = expr_span.end;
            }

            let exp = &self.code[start as usize..end as usize];
            let outer = Span { start, end };
            let inner = Span {
                start: expr_span.start,
                end: expr_span.end,
            };
            vars.push(PromptVar {
                exp: exp.to_string(),
                span: SpanShape { outer, inner },
            });
        }

        vars
    }

    fn get_template_text(&self, template: &ast::TemplateLiteral<'a>) -> String {
        template.span().source_text(self.code).to_string()
    }

    fn process_template_literal(&mut self, ident_name: &str, template: &ast::TemplateLiteral<'a>) {
        let (has_prompt, annotations, enclosure) =
            self.resolve_prompt_meta(ident_name, &template.span);
        if has_prompt {
            let prompt = Prompt {
                file: self.file.clone(),
                span: self.span_shape_literal(&template.span),
                enclosure,
                exp: self.get_template_text(template),
                vars: self.extract_template_vars(template),
                annotations,
            };
            self.prompts.push(prompt);
        }
    }

    fn process_string_literal(&mut self, ident_name: &str, string: &ast::StringLiteral<'a>) {
        let (has_prompt, annotations, enclosure) =
            self.resolve_prompt_meta(ident_name, &string.span);
        if has_prompt {
            let prompt = Prompt {
                file: self.file.clone(),
                span: self.span_shape_literal(&string.span),
                enclosure,
                exp: string.span().source_text(self.code).to_string(),
                vars: Vec::new(),
                annotations,
            };
            self.prompts.push(prompt);
        }
    }

    fn is_prompt(&self, ident_name: &str, has_stmt_prompt: bool) -> bool {
        if ident_name.to_lowercase().contains("prompt") || has_stmt_prompt {
            return true;
        }

        for scope in self.prompt_idents_stack.iter().rev() {
            if scope.contains(ident_name) {
                return true;
            }
        }

        false
    }

    /// Collect the block of leading comments immediately adjacent to the statement,
    /// and merge them into a single annotation if any line contains @prompt.
    fn collect_adjacent_leading_comments(
        &self,
        stmt_span: &oxc_span::Span,
    ) -> Vec<PromptAnnotation> {
        let mut block: Vec<&Comment> = Vec::new();
        let mut comment_idx: isize = (self.comments.len() as isize) - 1;
        while comment_idx >= 0 {
            let comment = &self.comments.get(comment_idx as usize);
            let comment = match comment {
                Some(comment) => comment,
                None => panic!("Unexpected missing comment at index {}", comment_idx),
            };

            if comment.span.end <= stmt_span.start {
                let start = comment.span.end as usize;
                let end = stmt_span.start as usize;
                let between = if start <= end && end <= self.code.len() {
                    &self.code[start..end]
                } else {
                    ""
                };

                if between.trim().is_empty() {
                    let mut j = comment_idx;
                    let mut last_start = stmt_span.start;
                    while j >= 0 {
                        let cj = &self.comments[j as usize];
                        if cj.span.end <= last_start {
                            let s = cj.span.end as usize;
                            let e = last_start as usize;
                            let between_ce = if s <= e && e <= self.code.len() {
                                &self.code[s..e]
                            } else {
                                ""
                            };
                            if between_ce.trim().is_empty() {
                                block.push(cj);
                                last_start = cj.span.start;
                                j -= 1;
                                continue;
                            }
                        }
                        break;
                    }
                    block.reverse();
                }
                break;
            }

            comment_idx -= 1;
        }

        if block.is_empty() {
            return Vec::new();
        }

        let first = block.first().unwrap();
        let last = block.last().unwrap();
        let start = first.span.start;
        let end = last.span.end;
        let block_text = &self.code[start as usize..end as usize];

        vec![PromptAnnotation {
            span: Span { start, end },
            exp: block_text.to_string(),
        }]
    }

    /// Collect inline @prompt comments within the statement range, optionally before a node.
    fn collect_inline_prompt_comments(
        &self,
        stmt_span: &oxc_span::Span,
        before: Option<&oxc_span::Span>,
    ) -> Vec<PromptAnnotation> {
        let mut out: Vec<PromptAnnotation> = Vec::new();
        let end_limit = before.map(|s| s.start).unwrap_or(stmt_span.end);
        for c in self.comments.iter() {
            if c.span.start >= stmt_span.start && c.span.start < end_limit {
                let full = c.span.source_text(self.code);
                if parse_annotation(full).unwrap_or(false) {
                    out.push(PromptAnnotation {
                        span: Span {
                            start: c.span.start,
                            end: c.span.end,
                        },
                        exp: full.to_string(),
                    });
                }
            }
        }
        out
    }

    fn current_stmt_span(&self) -> Option<oxc_span::Span> {
        self.stmt_span_stack.last().copied()
    }

    /// Resolve if we should treat expression as a prompt, capture comments and enclosure span.
    fn resolve_prompt_meta(
        &mut self,
        ident_name: &str,
        node_span: &oxc_span::Span,
    ) -> (bool, Vec<PromptAnnotation>, Span) {
        let mut annotations: Vec<PromptAnnotation> = self
            .stmt_annotations_stack
            .last()
            .cloned()
            .unwrap_or_default();
        if annotations.is_empty() {
            for scope in self.def_prompt_annotations_stack.iter().rev() {
                if let Some(def) = scope.get(ident_name) {
                    annotations = def.clone();
                    break;
                }
            }
        }
        let has_stmt_prompt = annotations
            .iter()
            .any(|a| parse_annotation(&a.exp).unwrap_or(false));
        let is_prompt = self.is_prompt(ident_name, has_stmt_prompt);

        let stmt_span = self.current_stmt_span().unwrap_or(*node_span);
        let leading_start = self
            .stmt_leading_start_stack
            .last()
            .copied()
            .flatten()
            .unwrap_or(stmt_span.start);
        let enclosure = Span {
            start: leading_start,
            end: stmt_span.end,
        };

        (is_prompt, annotations, enclosure)
    }
}

impl<'a> Visit<'a> for PromptVisitor<'a> {
    fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
        match kind {
            oxc_ast::AstKind::ExpressionStatement(expr) => {
                self.stmt_span_stack.push(expr.span);
                let leading = self.collect_adjacent_leading_comments(&expr.span);
                let inline = self.collect_inline_prompt_comments(&expr.span, None);
                let mut annotations: Vec<PromptAnnotation> = Vec::new();
                let leading_start = leading.first().map(|first| first.span.start);
                for a in leading.into_iter().chain(inline.into_iter()) {
                    annotations.push(a);
                }
                self.stmt_annotations_stack.push(annotations);
                self.stmt_leading_start_stack.push(leading_start);
            }

            oxc_ast::AstKind::Function(_) | oxc_ast::AstKind::ArrowFunctionExpression(_) => {
                self.prompt_idents_stack.push(HashSet::new());
                self.def_prompt_annotations_stack.push(HashMap::new());
            }

            oxc_ast::AstKind::VariableDeclaration(decl) => {
                self.stmt_span_stack.push(decl.span);
                let leading = self.collect_adjacent_leading_comments(&decl.span);
                let inline = self.collect_inline_prompt_comments(&decl.span, None);
                let mut annotations: Vec<PromptAnnotation> = Vec::new();
                let leading_start = leading.first().map(|first| first.span.start);
                for a in leading.into_iter().chain(inline.into_iter()) {
                    annotations.push(a);
                }
                let has_stmt_prompt = annotations
                    .iter()
                    .any(|a| parse_annotation(&a.exp).unwrap_or(false));
                self.stmt_annotations_stack.push(annotations);
                self.stmt_leading_start_stack.push(leading_start);
                for declarator in &decl.declarations {
                    self.process_variable_declarator(declarator, has_stmt_prompt);
                }
            }

            oxc_ast::AstKind::AssignmentExpression(expr) => {
                self.process_assignment_expression(expr);
            }

            _ => {}
        }
    }

    fn leave_node(&mut self, kind: oxc_ast::AstKind<'a>) {
        match kind {
            oxc_ast::AstKind::ExpressionStatement(_) => {
                self.stmt_span_stack.pop();
                self.stmt_annotations_stack.pop();
                self.stmt_leading_start_stack.pop();
            }
            oxc_ast::AstKind::Function(_) | oxc_ast::AstKind::ArrowFunctionExpression(_) => {
                self.prompt_idents_stack.pop();
                self.def_prompt_annotations_stack.pop();
            }
            oxc_ast::AstKind::VariableDeclaration(_) => {
                self.stmt_span_stack.pop();
                self.stmt_annotations_stack.pop();
                self.stmt_leading_start_stack.pop();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use insta::assert_debug_snapshot;
    use volumen_parser_test::*;

    #[test]
    fn detect_inline() {
        let inline_src = r#"const greeting = /* @prompt */ `Welcome ${user}!`;"#;
        assert_debug_snapshot!(ParserTs::parse(inline_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 31,
                                end: 49,
                            },
                            inner: Span {
                                start: 32,
                                end: 48,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 50,
                        },
                        exp: "`Welcome ${user}!`",
                        vars: [
                            PromptVar {
                                exp: "${user}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 40,
                                        end: 47,
                                    },
                                    inner: Span {
                                        start: 42,
                                        end: 46,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 17,
                                    end: 30,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn detect_inline_jsdoc() {
        let inline_jsdoc_src = r#"const msg = /** @prompt */ "Hello world";"#;
        assert_debug_snapshot!(ParserTs::parse(inline_jsdoc_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 27,
                                end: 40,
                            },
                            inner: Span {
                                start: 28,
                                end: 39,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 41,
                        },
                        exp: "\"Hello world\"",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 12,
                                    end: 26,
                                },
                                exp: "/** @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn detect_inline_dirty() {
        let inline_dirty_src = r#"const greeting = /* @prompt greeting */ `Welcome ${user}!`;"#;
        assert_prompts_size(ParserTs::parse(inline_dirty_src, "prompts.ts"), 1)
    }

    #[test]
    fn detect_inline_none() {
        let inline_none_src = r#"const greeting = /* @prompting */ `Welcome ${user}!`;
const whatever = /* wrong@prompt */ "That's not it!";"#;
        assert_prompts_size(ParserTs::parse(inline_none_src, "prompts.ts"), 0)
    }

    #[test]
    fn detect_var_comment() {
        let var_comment_src = indoc! {r#"
            // @prompt
            const hello = `Hello, world!`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(var_comment_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 25,
                                end: 40,
                            },
                            inner: Span {
                                start: 26,
                                end: 39,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 41,
                        },
                        exp: "`Hello, world!`",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 10,
                                },
                                exp: "// @prompt",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn detect_var_inline() {
        let var_comment_src = indoc! {r#"
            /* @prompt */
            const hello = `Hello, world!`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(var_comment_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 28,
                                end: 43,
                            },
                            inner: Span {
                                start: 29,
                                end: 42,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 44,
                        },
                        exp: "`Hello, world!`",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 13,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn detect_var_jsdoc() {
        let var_jsdoc_src = indoc! {r#"
            /** @prompt */
            const hello = `Hello, world!`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(var_jsdoc_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 29,
                                end: 44,
                            },
                            inner: Span {
                                start: 30,
                                end: 43,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 45,
                        },
                        exp: "`Hello, world!`",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 14,
                                },
                                exp: "/** @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn detect_var_comment_spaced() {
        let var_comment_src = indoc! {r#"
            // @prompt


            const hello = `Hello, world!`;

            // @prompt
            nope()

            const world = "Hello!";
        "#};
        assert_debug_snapshot!(ParserTs::parse(var_comment_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 27,
                                end: 42,
                            },
                            inner: Span {
                                start: 28,
                                end: 41,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 43,
                        },
                        exp: "`Hello, world!`",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 10,
                                },
                                exp: "// @prompt",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn multiline_annotation() {
        let src = indoc! {r#"
            // Hello
            // @prompt
            // world
            const msg = "Hello";
        "#};
        match ParserTs::parse(src, "prompts.ts") {
            ParseResult::ParseResultSuccess(ParseResultSuccess { prompts, .. }) => {
                assert_eq!(prompts.len(), 1, "expected exactly one prompt");
                let ann = &prompts[0].annotations;
                assert_eq!(ann.len(), 1, "expected a single combined annotation block");
                assert_eq!(ann[0].exp, "// Hello\n// @prompt\n// world");
            }
            _ => panic!("Expected ParseResultSuccess"),
        }
    }

    #[test]
    fn detect_multi() {
        let multi_src = indoc! {r#"
            const userPrompt = `Hello, ${name}!`;
            const greeting = /* @prompt */ `Welcome ${user}!`;
            // @prompt
            const farewell = `Goodbye ${user.name}!`;
            /** @prompt */
            const system = "You are an AI assistant";
            const regular = `Not a prompt ${value}`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(multi_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 19,
                                end: 36,
                            },
                            inner: Span {
                                start: 20,
                                end: 35,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 37,
                        },
                        exp: "`Hello, ${name}!`",
                        vars: [
                            PromptVar {
                                exp: "${name}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 27,
                                        end: 34,
                                    },
                                    inner: Span {
                                        start: 29,
                                        end: 33,
                                    },
                                },
                            },
                        ],
                        annotations: [],
                    },
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 69,
                                end: 87,
                            },
                            inner: Span {
                                start: 70,
                                end: 86,
                            },
                        },
                        enclosure: Span {
                            start: 38,
                            end: 88,
                        },
                        exp: "`Welcome ${user}!`",
                        vars: [
                            PromptVar {
                                exp: "${user}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 78,
                                        end: 85,
                                    },
                                    inner: Span {
                                        start: 80,
                                        end: 84,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 55,
                                    end: 68,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 117,
                                end: 140,
                            },
                            inner: Span {
                                start: 118,
                                end: 139,
                            },
                        },
                        enclosure: Span {
                            start: 89,
                            end: 141,
                        },
                        exp: "`Goodbye ${user.name}!`",
                        vars: [
                            PromptVar {
                                exp: "${user.name}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 126,
                                        end: 138,
                                    },
                                    inner: Span {
                                        start: 128,
                                        end: 137,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 89,
                                    end: 99,
                                },
                                exp: "// @prompt",
                            },
                        ],
                    },
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 172,
                                end: 197,
                            },
                            inner: Span {
                                start: 173,
                                end: 196,
                            },
                        },
                        enclosure: Span {
                            start: 142,
                            end: 198,
                        },
                        exp: "\"You are an AI assistant\"",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 142,
                                    end: 156,
                                },
                                exp: "/** @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn detect_single_var() {
        let single_var_src = r#"const userPrompt = `Hello, ${name}!`;"#;
        assert_debug_snapshot!(ParserTs::parse(single_var_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 19,
                                end: 36,
                            },
                            inner: Span {
                                start: 20,
                                end: 35,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 37,
                        },
                        exp: "`Hello, ${name}!`",
                        vars: [
                            PromptVar {
                                exp: "${name}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 27,
                                        end: 34,
                                    },
                                    inner: Span {
                                        start: 29,
                                        end: 33,
                                    },
                                },
                            },
                        ],
                        annotations: [],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn detect_multi_vars() {
        let multi_vars_src =
            r#"const userPrompt = `Hello, ${name}! How is the weather today in ${city}?`;"#;
        assert_debug_snapshot!(ParserTs::parse(multi_vars_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 19,
                                end: 73,
                            },
                            inner: Span {
                                start: 20,
                                end: 72,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 74,
                        },
                        exp: "`Hello, ${name}! How is the weather today in ${city}?`",
                        vars: [
                            PromptVar {
                                exp: "${name}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 27,
                                        end: 34,
                                    },
                                    inner: Span {
                                        start: 29,
                                        end: 33,
                                    },
                                },
                            },
                            PromptVar {
                                exp: "${city}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 64,
                                        end: 71,
                                    },
                                    inner: Span {
                                        start: 66,
                                        end: 70,
                                    },
                                },
                            },
                        ],
                        annotations: [],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn parse_exp_vars() {
        let exp_vars_src = r#"const userPrompt = `Hello, ${user.name}! How is the weather today in ${user.location.city}?`;"#;
        assert_debug_snapshot!(ParserTs::parse(exp_vars_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 19,
                                end: 92,
                            },
                            inner: Span {
                                start: 20,
                                end: 91,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 93,
                        },
                        exp: "`Hello, ${user.name}! How is the weather today in ${user.location.city}?`",
                        vars: [
                            PromptVar {
                                exp: "${user.name}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 27,
                                        end: 39,
                                    },
                                    inner: Span {
                                        start: 29,
                                        end: 38,
                                    },
                                },
                            },
                            PromptVar {
                                exp: "${user.location.city}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 69,
                                        end: 90,
                                    },
                                    inner: Span {
                                        start: 71,
                                        end: 89,
                                    },
                                },
                            },
                        ],
                        annotations: [],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn parse_exp_vars_complex() {
        let exp_vars_src =
            r#"const userPrompt = `Hello, ${User.fullName({ ...user.name, last: null })}!`;"#;
        assert_debug_snapshot!(ParserTs::parse(exp_vars_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 19,
                                end: 75,
                            },
                            inner: Span {
                                start: 20,
                                end: 74,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 76,
                        },
                        exp: "`Hello, ${User.fullName({ ...user.name, last: null })}!`",
                        vars: [
                            PromptVar {
                                exp: "${User.fullName({ ...user.name, last: null })}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 27,
                                        end: 73,
                                    },
                                    inner: Span {
                                        start: 29,
                                        end: 72,
                                    },
                                },
                            },
                        ],
                        annotations: [],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn handle_invalid_syntax() {
        let invalid_src = r#"const invalid = `unclosed template"#;
        assert_debug_snapshot!(ParserTs::parse(invalid_src, "prompts.ts"), @r#"
        ParseResultError(
            ParseResultError {
                state: "error",
                error: "Unterminated string",
            },
        )
        "#);
    }

    #[test]
    fn parse_js_code() {
        let js_src = r#"const prompt = /* @prompt */ `Hello ${world}!`;"#;
        assert_debug_snapshot!(ParserTs::parse(js_src, "test.js"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "test.js",
                        span: SpanShape {
                            outer: Span {
                                start: 29,
                                end: 46,
                            },
                            inner: Span {
                                start: 30,
                                end: 45,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 47,
                        },
                        exp: "`Hello ${world}!`",
                        vars: [
                            PromptVar {
                                exp: "${world}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 36,
                                        end: 44,
                                    },
                                    inner: Span {
                                        start: 38,
                                        end: 43,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 15,
                                    end: 28,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn parse_jsx_code() {
        let jsx_src = indoc! {r#"
            const prompt = /* @prompt */ `Hello ${world}!`;
            const element = <div>{prompt}</div>;
        "#};
        assert_debug_snapshot!(ParserTs::parse(jsx_src, "test.jsx"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "test.jsx",
                        span: SpanShape {
                            outer: Span {
                                start: 29,
                                end: 46,
                            },
                            inner: Span {
                                start: 30,
                                end: 45,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 47,
                        },
                        exp: "`Hello ${world}!`",
                        vars: [
                            PromptVar {
                                exp: "${world}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 36,
                                        end: 44,
                                    },
                                    inner: Span {
                                        start: 38,
                                        end: 43,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 15,
                                    end: 28,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn parse_ts_code() {
        let ts_src = r#"const prompt : string = /* @prompt */ `Hello ${world}!`;"#;
        assert_debug_snapshot!(ParserTs::parse(ts_src, "test.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "test.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 38,
                                end: 55,
                            },
                            inner: Span {
                                start: 39,
                                end: 54,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 56,
                        },
                        exp: "`Hello ${world}!`",
                        vars: [
                            PromptVar {
                                exp: "${world}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 45,
                                        end: 53,
                                    },
                                    inner: Span {
                                        start: 47,
                                        end: 52,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 24,
                                    end: 37,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn parse_tsx_code() {
        let tsx_src = indoc! {r#"
            const prompt : string = /* @prompt */ `Hello ${world}!`;
            const element = <div>{prompt}</div>;
        "#};
        assert_debug_snapshot!(ParserTs::parse(tsx_src, "test.tsx"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "test.tsx",
                        span: SpanShape {
                            outer: Span {
                                start: 38,
                                end: 55,
                            },
                            inner: Span {
                                start: 39,
                                end: 54,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 56,
                        },
                        exp: "`Hello ${world}!`",
                        vars: [
                            PromptVar {
                                exp: "${world}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 45,
                                        end: 53,
                                    },
                                    inner: Span {
                                        start: 47,
                                        end: 52,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 24,
                                    end: 37,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn parse_spans_str() {
        let span_str_src = indoc! {r#"
            const systemPrompt = "You are a helpful assistant.";
        "#};
        let span_str_result = ParserTs::parse(span_str_src, "prompts.ts");
        assert_prompt_spans(span_str_src, span_str_result);
    }

    #[test]
    fn parse_spans_tmpl() {
        let span_tmpl_src = indoc! {r#"
            const userPrompt = `Hello, ${name}! How is the weather today in ${city}?`;
        "#};
        let span_tmpl_result = ParserTs::parse(span_tmpl_src, "prompts.ts");
        assert_prompt_spans(span_tmpl_src, span_tmpl_result);
    }

    #[test]
    fn parse_nested() {
        let nested_src = indoc! {r#"
            class Hello {
                world(self) {
                    const fn = () => {
                        const helloPrompt = `Hello, ${name}!`;

                        // @prompt
                        const alsoPrmpt = "Hi!";
                    };

                    return fn;
                }
            }
        "#};
        assert_debug_snapshot!(ParserTs::parse(nested_src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 91,
                                end: 108,
                            },
                            inner: Span {
                                start: 92,
                                end: 107,
                            },
                        },
                        enclosure: Span {
                            start: 71,
                            end: 109,
                        },
                        exp: "`Hello, ${name}!`",
                        vars: [
                            PromptVar {
                                exp: "${name}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 99,
                                        end: 106,
                                    },
                                    inner: Span {
                                        start: 101,
                                        end: 105,
                                    },
                                },
                            },
                        ],
                        annotations: [],
                    },
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 164,
                                end: 169,
                            },
                            inner: Span {
                                start: 165,
                                end: 168,
                            },
                        },
                        enclosure: Span {
                            start: 123,
                            end: 170,
                        },
                        exp: "\"Hi!\"",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 123,
                                    end: 133,
                                },
                                exp: "// @prompt",
                            },
                        ],
                    },
                ],
            },
        )
        "#)
    }

    #[test]
    fn multi_annotations() {
        let src = indoc! {r#"
            // Hello, world
            const hello = /* @prompt */ "asd";
        "#};
        assert_debug_snapshot!(ParserTs::parse(src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 44,
                                end: 49,
                            },
                            inner: Span {
                                start: 45,
                                end: 48,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 50,
                        },
                        exp: "\"asd\"",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 15,
                                },
                                exp: "// Hello, world",
                            },
                            PromptAnnotation {
                                span: Span {
                                    start: 30,
                                    end: 43,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn multiline_annotations() {
        let src = indoc! {r#"
            /*
             Multi
             Line
             Block
            */
            const hello = /* @prompt */ `x`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 54,
                                end: 57,
                            },
                            inner: Span {
                                start: 55,
                                end: 56,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 58,
                        },
                        exp: "`x`",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 25,
                                },
                                exp: "/*\n Multi\n Line\n Block\n*/",
                            },
                            PromptAnnotation {
                                span: Span {
                                    start: 40,
                                    end: 53,
                                },
                                exp: "/* @prompt */",
                            },
                        ],
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn multiline_annotation_nested() {
        let src = indoc! {r#"
            function fn() {
                // Hello
                // @prompt
                // world
                const msg = "Hello";
            }
        "#};
        assert_debug_snapshot!(ParserTs::parse(src, "prompts.ts"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 73,
                                end: 80,
                            },
                            inner: Span {
                                start: 74,
                                end: 79,
                            },
                        },
                        enclosure: Span {
                            start: 20,
                            end: 81,
                        },
                        exp: "\"Hello\"",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 20,
                                    end: 56,
                                },
                                exp: "// Hello\n    // @prompt\n    // world",
                            },
                        ],
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn reassign_no_comment() {
        let src = indoc! {r#"
            // @prompt
            let hello: string;
            hello = `Hi`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(src, "prompts.ts"),@r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 38,
                                end: 42,
                            },
                            inner: Span {
                                start: 39,
                                end: 41,
                            },
                        },
                        enclosure: Span {
                            start: 30,
                            end: 43,
                        },
                        exp: "`Hi`",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 10,
                                },
                                exp: "// @prompt",
                            },
                        ],
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn reassign_with_comment() {
        let src = indoc! {r#"
            // @prompt def
            let hello: string;
            // @prompt fresh
            hello = `Hi`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(src, "prompts.ts"),@r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 59,
                                end: 63,
                            },
                            inner: Span {
                                start: 60,
                                end: 62,
                            },
                        },
                        enclosure: Span {
                            start: 34,
                            end: 64,
                        },
                        exp: "`Hi`",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 34,
                                    end: 50,
                                },
                                exp: "// @prompt fresh",
                            },
                        ],
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn reassign_with_comment_multi() {
        let src = indoc! {r#"
            // @prompt def
            let hello: string;
            // hello
            hello = /* @prompt fresh */ `Hi`;
        "#};
        assert_debug_snapshot!(ParserTs::parse(src, "prompts.ts"),@r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.ts",
                        span: SpanShape {
                            outer: Span {
                                start: 71,
                                end: 75,
                            },
                            inner: Span {
                                start: 72,
                                end: 74,
                            },
                        },
                        enclosure: Span {
                            start: 34,
                            end: 76,
                        },
                        exp: "`Hi`",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 34,
                                    end: 42,
                                },
                                exp: "// hello",
                            },
                            PromptAnnotation {
                                span: Span {
                                    start: 51,
                                    end: 70,
                                },
                                exp: "/* @prompt fresh */",
                            },
                        ],
                    },
                ],
            },
        )
        "#);
    }
}
