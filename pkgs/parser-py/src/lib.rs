use std::collections::{HashMap, HashSet};

use rustpython_ast as ast;
use rustpython_ast::Ranged;
use rustpython_ast::Visitor;
use rustpython_ast::text_size::TextRange;
use rustpython_parser::Parse;
use rustpython_parser::lexer::lex;
use rustpython_parser::{Mode, Tok};
pub use volumen_parser_core::VolumenParser;
use volumen_parser_core::*;
use volumen_types::*;

pub struct ParserPy {}

impl VolumenParser for ParserPy {
    fn parse(source: &str, filename: &str) -> ParseResult {
        let suite = match ast::Suite::parse(source, filename) {
            Ok(suite) => suite,
            Err(err) => {
                return ParseResult::ParseResultError(ParseResultError {
                    state: ParseResultErrorStateError,
                    error: format!("{}", err),
                });
            }
        };

        let comments = ParserPy::parse_comments(source);

        let mut visitor = PyPromptVisitor::new(source, filename.to_string(), comments);
        for stmt in suite {
            visitor.visit_stmt(stmt);
        }

        ParseResult::ParseResultSuccess(ParseResultSuccess {
            state: ParseResultSuccessStateSuccess,
            prompts: visitor.prompts,
        })
    }
}

impl ParserPy {
    fn parse_comments(source: &str) -> Vec<TextRange> {
        let mut comments: Vec<TextRange> = Vec::new();
        for result in lex(source, Mode::Module) {
            if let Ok((Tok::Comment(_text), range)) = result {
                comments.push(range);
            }
        }
        comments
    }
}

struct PyPromptVisitor<'a> {
    /// Source code being analyzed.
    code: &'a str,
    /// Source code file path.
    file: String,
    /// Collected prompts.
    prompts: Vec<Prompt>,
    /// Stack of identifiers that have prompt annotations.
    prompt_idents_stack: Vec<HashSet<String>>,
    /// All comment markers sorted by start.
    comments: Vec<TextRange>,
    /// Cursor to track position in comments array.
    comment_cursor: usize,
    /// Annotations for the current statement (leading comments + inline @prompt).
    stmt_annotations_stack: Vec<Vec<PromptAnnotation>>,
    /// Earliest leading annotation start for current statement.
    stmt_leading_start_stack: Vec<Option<u32>>,
    /// Current statement range stack (for enclosure end, includes trailing tokens like newlines).
    stmt_range_stack: Vec<TextRange>,
    /// Annotations captured at definition time for annotated variables.
    def_prompt_annotations: HashMap<String, Vec<PromptAnnotation>>,
    /// Set of annotated identifiers.
    annotated_idents: HashSet<String>,
}

impl<'a> PyPromptVisitor<'a> {
    fn new(code: &'a str, file: String, comments: Vec<TextRange>) -> Self {
        Self {
            code,
            file,
            prompts: Vec::new(),
            prompt_idents_stack: vec![HashSet::new()],
            comments,
            comment_cursor: 0,
            stmt_annotations_stack: Vec::new(),
            stmt_leading_start_stack: Vec::new(),
            stmt_range_stack: Vec::new(),
            def_prompt_annotations: HashMap::new(),
            annotated_idents: HashSet::new(),
        }
    }

    fn span(&self, range: TextRange) -> Span {
        Span {
            start: range.start().to_usize() as u32,
            end: range.end().to_usize() as u32,
        }
    }

    fn span_shape_string_like(&self, range: TextRange) -> SpanShape {
        // Python strings may have prefixes like f, r, u, fr, etc., and
        // single/double/triple quotes. Compute inner by finding opening quote
        // and matching its length (1 or 3 characters).
        let bytes = self.code.as_bytes();
        let start = range.start().to_usize();
        let end = range.end().to_usize();

        // Find first quote character from start.
        let mut i = start;
        while i < end {
            let c = bytes[i];
            if c == b'\'' || c == b'\"' {
                break;
            }
            i += 1;
        }

        let quote_pos = i;
        let quote_char = if quote_pos < end {
            bytes[quote_pos]
        } else {
            b'\''
        };
        let mut quote_len = 1u32;
        if quote_pos + 2 < end
            && bytes[quote_pos + 1] == quote_char
            && bytes[quote_pos + 2] == quote_char
        {
            quote_len = 3;
        }

        let outer = self.span(range);
        let inner_start = (quote_pos as u32).saturating_add(quote_len);
        let inner_end = outer.end.saturating_sub(quote_len);
        let inner = Span {
            start: inner_start,
            end: inner_end,
        };

        SpanShape { outer, inner }
    }

    fn collect_vars_from_joined_str(&self, joined: &ast::ExprJoinedStr) -> Vec<PromptVar> {
        let mut vars = Vec::new();
        for value in &joined.values {
            if let ast::Expr::FormattedValue(formatted) = value {
                // Use the inner expression range as a stable anchor.
                let inner_r = formatted.value.range();
                let mut outer_start = inner_r.start().to_usize();
                let mut outer_end = inner_r.end().to_usize();

                let bytes = self.code.as_bytes();

                // Seek left to the opening '{'
                let mut i = outer_start;
                while i > 0 {
                    i -= 1;
                    if bytes[i] == b'{' {
                        outer_start = i;
                        break;
                    }
                }

                // Seek right to the closing '}'
                let mut j = outer_end;
                while j < bytes.len() {
                    if bytes[j] == b'}' {
                        outer_end = j + 1;
                        break;
                    }
                    j += 1;
                }

                let outer = Span {
                    start: outer_start as u32,
                    end: outer_end as u32,
                };
                let inner = Span {
                    start: outer.start + 1,
                    end: outer.end.saturating_sub(1),
                };
                let exp_range = TextRange::new(
                    rustpython_ast::text_size::TextSize::from(outer.start),
                    rustpython_ast::text_size::TextSize::from(outer.end),
                );

                vars.push(PromptVar {
                    exp: self.code[exp_range].to_string(),
                    span: SpanShape { outer, inner },
                });
            }
        }
        vars
    }

    fn process_assign_target(
        &mut self,
        is_prompt: bool,
        target: &ast::Expr,
        val: Option<&ast::Expr>,
    ) {
        match &target {
            ast::Expr::Name(name_expr) => {
                self.process_assign(is_prompt, name_expr, val);
            }

            ast::Expr::Tuple(ast::ExprTuple { elts: idents, .. })
            | ast::Expr::List(ast::ExprList { elts: idents, .. }) => match val {
                Some(ast::Expr::Tuple(ast::ExprTuple { elts: vals, .. }))
                | Some(ast::Expr::List(ast::ExprList { elts: vals, .. })) => {
                    self.process_assigns(is_prompt, idents, Some(vals));
                }

                None => {
                    self.process_assigns(is_prompt, idents, None);
                }

                _ => {}
            },

            _ => {}
        }
    }

    fn process_assigns(
        &mut self,
        is_prompt: bool,
        idents: &Vec<ast::Expr>,
        vals: Option<&Vec<ast::Expr>>,
    ) {
        let vals = vals.as_ref();
        for (i, ident) in idents.iter().enumerate() {
            if let ast::Expr::Name(ident) = ident {
                self.process_assign(is_prompt, ident, vals.and_then(|v| v.get(i)));
            }
        }
    }

    fn process_assign(&mut self, is_prompt: bool, name: &ast::ExprName, val: Option<&ast::Expr>) {
        let ident = name.id.as_str();

        if is_prompt {
            self.push_prompt_ident(ident);
        }

        if let Some(val) = val {
            match val {
                // f"...{expr}..."
                ast::Expr::JoinedStr(joined) => {
                    let vars = self.collect_vars_from_joined_str(joined);
                    self.process_range(ident, joined.range(), vars);
                }
                // "..."
                ast::Expr::Constant(c) => {
                    if matches!(c.value, ast::Constant::Str(_)) {
                        self.process_range(ident, c.range(), Vec::new());
                    }
                }
                _ => {}
            }
        }
    }

    fn process_range(&mut self, ident: &str, node_range: TextRange, vars: Vec<PromptVar>) {
        let in_prompt_ident = self
            .prompt_idents_stack
            .iter()
            .rev()
            .any(|s| s.contains(ident));
        let mut annotations: Vec<PromptAnnotation> = self
            .stmt_annotations_stack
            .last()
            .cloned()
            .unwrap_or_default();
        // If reassignment without new annotation and variable was annotated with a def-annotation, reuse it.
        if annotations.is_empty()
            && self.annotated_idents.contains(ident)
            && let Some(def) = self.def_prompt_annotations.get(ident)
        {
            annotations = def.clone();
        }

        let has_prompt_annotation = annotations
            .iter()
            .any(|a| parse_annotation(&a.exp).unwrap_or(false));
        let is_prompt =
            ident.to_lowercase().contains("prompt") || in_prompt_ident || has_prompt_annotation;
        if !is_prompt {
            return;
        }

        // Enclosure: from either preceding leading annotation (if any) or the statement start, to statement end
        let stmt_range = self.stmt_range_stack.last().copied().unwrap_or(node_range);
        let leading_start = self
            .stmt_leading_start_stack
            .last()
            .copied()
            .flatten()
            .unwrap_or(stmt_range.start().to_usize() as u32);
        let enclosure = Span {
            start: leading_start,
            end: stmt_range.end().to_usize() as u32,
        };

        let prompt = Prompt {
            file: self.file.clone(),
            span: self.span_shape_string_like(node_range),
            enclosure,
            exp: self.code[node_range].to_string(),
            vars,
            annotations,
        };
        self.prompts.push(prompt);
    }

    fn collect_adjacent_leading_comments(&self, stmt: &ast::Stmt) -> Vec<PromptAnnotation> {
        let stmt_start = stmt.range().start();
        let mut block_ranges: Vec<TextRange> = Vec::new();
        let mut idx: isize = (self.comments.len() as isize) - 1;
        while idx >= 0 {
            let r = self.comments[idx as usize];
            if r.end() <= stmt_start {
                let start = r.end().to_usize();
                let end = stmt_start.to_usize();
                let between = if start <= end && end <= self.code.len() {
                    &self.code[start..end]
                } else {
                    ""
                };
                if between.trim().is_empty() {
                    let mut j = idx;
                    let mut last = stmt_start;
                    while j >= 0 {
                        let rr = self.comments[j as usize];
                        if rr.end() <= last {
                            let s = rr.end().to_usize();
                            let e = last.to_usize();
                            let between2 = if s <= e && e <= self.code.len() {
                                &self.code[s..e]
                            } else {
                                ""
                            };
                            if between2.trim().is_empty() {
                                block_ranges.push(rr);
                                last = rr.start();
                                j -= 1;
                                continue;
                            }
                        }
                        break;
                    }
                    block_ranges.reverse();
                }
                break;
            }
            idx -= 1;
        }

        if block_ranges.is_empty() {
            return Vec::new();
        }

        let first = block_ranges.first().unwrap();
        let last = block_ranges.last().unwrap();
        let start = first.start().to_usize() as u32;
        let end = last.end().to_usize() as u32;
        let block_text = &self.code[TextRange::new(first.start(), last.end())];
        vec![PromptAnnotation {
            span: Span { start, end },
            exp: block_text.to_string(),
        }]
    }

    fn collect_inline_prompt_comments(&self, stmt: &ast::Stmt) -> Vec<PromptAnnotation> {
        let r = stmt.range();
        let mut out: Vec<PromptAnnotation> = Vec::new();
        for &cr in &self.comments {
            if cr.start() >= r.start() && cr.start() < r.end() {
                let text = self.code[cr].to_string();
                if parse_annotation(&text).unwrap_or(false) {
                    out.push(PromptAnnotation {
                        span: Span {
                            start: cr.start().to_usize() as u32,
                            end: cr.end().to_usize() as u32,
                        },
                        exp: text,
                    });
                }
            }
        }
        out
    }

    fn push_prompt_ident(&mut self, ident: &str) {
        if let Some(scope) = self.prompt_idents_stack.last_mut() {
            scope.insert(ident.to_string());
        }
    }
}

impl<'a> Visitor for PyPromptVisitor<'a> {
    fn visit_stmt(&mut self, node: ast::Stmt) {
        // Prepare annotations for this statement
        let leading = self.collect_adjacent_leading_comments(&node);
        let inline = self.collect_inline_prompt_comments(&node);
        let mut annotations: Vec<PromptAnnotation> = Vec::new();
        let leading_start = leading.first().map(|a| a.span.start);
        for a in leading.into_iter().chain(inline.into_iter()) {
            annotations.push(a);
        }
        let is_prompt = annotations
            .iter()
            .any(|a| parse_annotation(&a.exp).unwrap_or(false));
        self.stmt_annotations_stack.push(annotations);
        self.stmt_leading_start_stack.push(leading_start);
        self.stmt_range_stack.push(node.range());
        self.generic_visit_stmt(node);
        self.stmt_annotations_stack.pop();
        self.stmt_leading_start_stack.pop();
        self.stmt_range_stack.pop();
    }

    fn visit_stmt_assign(&mut self, node: ast::StmtAssign) {
        let is_prompt = self
            .stmt_annotations_stack
            .last()
            .unwrap_or(&Vec::new())
            .iter()
            .any(|a| parse_annotation(&a.exp).unwrap_or(false));
        for target in &node.targets {
            self.process_assign_target(is_prompt, target, Some(&node.value));
        }
        self.generic_visit_stmt_assign(node);
    }

    fn visit_stmt_ann_assign(&mut self, node: ast::StmtAnnAssign) {
        let is_prompt = self
            .stmt_annotations_stack
            .last()
            .unwrap_or(&Vec::new())
            .iter()
            .any(|a| parse_annotation(&a.exp).unwrap_or(false));
        // Record annotated identifiers
        if let ast::Expr::Name(name) = &*node.target {
            self.annotated_idents.insert(name.id.to_string());
            // Save definition-time prompt comments, if any
            if is_prompt
                && let Some(ann) = self.stmt_annotations_stack.last()
                && !ann.is_empty()
            {
                self.def_prompt_annotations
                    .insert(name.id.to_string(), ann.clone());
            }
        }
        self.process_assign_target(is_prompt, &node.target, node.value.as_deref());
        self.generic_visit_stmt_ann_assign(node);
    }

    fn visit_stmt_function_def(&mut self, node: ast::StmtFunctionDef) {
        self.prompt_idents_stack.push(HashSet::new());
        self.generic_visit_stmt_function_def(node);
        self.prompt_idents_stack.pop();
    }

    fn visit_stmt_async_function_def(&mut self, node: ast::StmtAsyncFunctionDef) {
        self.prompt_idents_stack.push(HashSet::new());
        self.generic_visit_stmt_async_function_def(node);
        self.prompt_idents_stack.pop();
    }

    fn visit_stmt_class_def(&mut self, node: ast::StmtClassDef) {
        self.prompt_idents_stack.push(HashSet::new());
        self.generic_visit_stmt_class_def(node);
        self.prompt_idents_stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use insta::assert_debug_snapshot;
    use volumen_parser_test::*;

    #[test]
    fn detect_var_comment_dirty_str() {
        let var_comment_dirty_str_src = indoc! {r#"
            # @prompt system
            system = "You are a helpful assistant."
        "#};
        assert_debug_snapshot!(ParserPy::parse(var_comment_dirty_str_src, "prompts.py"), @r##"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.py",
                        span: SpanShape {
                            outer: Span {
                                start: 26,
                                end: 56,
                            },
                            inner: Span {
                                start: 27,
                                end: 55,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 56,
                        },
                        exp: "\"You are a helpful assistant.\"",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 16,
                                },
                                exp: "# @prompt system",
                            },
                        ],
                    },
                ],
            },
        )
        "##);
    }

    #[test]
    fn detect_var_comment_dirty_fstr() {
        let var_comment_dirty_fstr_src = indoc! {r#"
            # @prompt user
            greeting = f"Welcome {user}!"
        "#};
        assert_debug_snapshot!(ParserPy::parse(var_comment_dirty_fstr_src, "prompts.py"), @r##"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.py",
                        span: SpanShape {
                            outer: Span {
                                start: 26,
                                end: 44,
                            },
                            inner: Span {
                                start: 28,
                                end: 43,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 44,
                        },
                        exp: "f\"Welcome {user}!\"",
                        vars: [
                            PromptVar {
                                exp: "{user}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 36,
                                        end: 42,
                                    },
                                    inner: Span {
                                        start: 37,
                                        end: 41,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 14,
                                },
                                exp: "# @prompt user",
                            },
                        ],
                    },
                ],
            },
        )
        "##);
    }

    #[test]
    fn detect_multi_assign_var_comment() {
        let multi_assign_var_comment_src = indoc! {r#"
            # @prompt
            hello, world = "Hello", "World"
        "#};
        let multi_assign_var_comment_result =
            ParserPy::parse(multi_assign_var_comment_src, "prompts.py");
        assert_prompts_size(multi_assign_var_comment_result, 2);
    }

    #[test]
    fn detect_multi_assign_variants() {
        let src = indoc! {r#"
            # @prompt
            hello1, world1 = ("Hello", "World")
            # @prompt
            [hello2, world2] = ("Hello", "World")
            # @prompt
            (hello3, world3) = ("Hello", "World")
            # @prompt
            hello4, world4 = ["Hello", "World"]
            # @prompt
            [hello5, world5] = ["Hello", "World"]
            # @prompt
            (hello6, world6) = ["Hello", "World"]
        "#};
        let result = ParserPy::parse(src, "prompts.py");
        assert_prompts_size(result, 12);
    }

    #[test]
    fn detect_chained_assign() {
        let src = indoc! {r#"
            # @prompt
            hello = world = "Hi"
        "#};
        let result = ParserPy::parse(src, "prompts.py");
        assert_prompts_size(result, 2);
    }

    #[test]
    fn detect_multi_vars() {
        let multi_vars_src =
            r#"user_prompt = f"Hello, {name}! How is the weather today in {city}?""#;
        assert_debug_snapshot!(ParserPy::parse(multi_vars_src, "prompts.py"), @r#"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.py",
                        span: SpanShape {
                            outer: Span {
                                start: 14,
                                end: 67,
                            },
                            inner: Span {
                                start: 16,
                                end: 66,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 67,
                        },
                        exp: "f\"Hello, {name}! How is the weather today in {city}?\"",
                        vars: [
                            PromptVar {
                                exp: "{name}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 23,
                                        end: 29,
                                    },
                                    inner: Span {
                                        start: 24,
                                        end: 28,
                                    },
                                },
                            },
                            PromptVar {
                                exp: "{city}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 59,
                                        end: 65,
                                    },
                                    inner: Span {
                                        start: 60,
                                        end: 64,
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
    fn parse_multiline_str() {
        let multiline_str_src = indoc! {r#"
            # @prompt
            system = """You are a helpful assistant.
            You will answer the user's questions to the best of your ability.
            If you don't know the answer, just say that you don't know, don't try to make it up."""
        "#};
        let multiline_str_result = ParserPy::parse(multiline_str_src, "prompts.py");
        assert_debug_snapshot!(multiline_str_result, @r##"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.py",
                        span: SpanShape {
                            outer: Span {
                                start: 19,
                                end: 204,
                            },
                            inner: Span {
                                start: 22,
                                end: 201,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 204,
                        },
                        exp: "\"\"\"You are a helpful assistant.\nYou will answer the user's questions to the best of your ability.\nIf you don't know the answer, just say that you don't know, don't try to make it up.\"\"\"",
                        vars: [],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 9,
                                },
                                exp: "# @prompt",
                            },
                        ],
                    },
                ],
            },
        )
        "##);
    }

    #[test]
    fn parse_multiline_fstr() {
        let multiline_fstr_src = indoc! {r#"
            # @prompt
            user = f"""Hello, {name}!
            How is the weather today in {city}?
            """
        "#};
        let multiline_fstr_result = ParserPy::parse(multiline_fstr_src, "prompts.py");
        assert_debug_snapshot!(multiline_fstr_result, @r##"
        ParseResultSuccess(
            ParseResultSuccess {
                state: "success",
                prompts: [
                    Prompt {
                        file: "prompts.py",
                        span: SpanShape {
                            outer: Span {
                                start: 17,
                                end: 75,
                            },
                            inner: Span {
                                start: 21,
                                end: 72,
                            },
                        },
                        enclosure: Span {
                            start: 0,
                            end: 75,
                        },
                        exp: "f\"\"\"Hello, {name}!\nHow is the weather today in {city}?\n\"\"\"",
                        vars: [
                            PromptVar {
                                exp: "{name}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 28,
                                        end: 34,
                                    },
                                    inner: Span {
                                        start: 29,
                                        end: 33,
                                    },
                                },
                            },
                            PromptVar {
                                exp: "{city}",
                                span: SpanShape {
                                    outer: Span {
                                        start: 64,
                                        end: 70,
                                    },
                                    inner: Span {
                                        start: 65,
                                        end: 69,
                                    },
                                },
                            },
                        ],
                        annotations: [
                            PromptAnnotation {
                                span: Span {
                                    start: 0,
                                    end: 9,
                                },
                                exp: "# @prompt",
                            },
                        ],
                    },
                ],
            },
        )
        "##);
    }

    #[test]
    fn handle_invalid_syntax() {
        let invalid_syntax_src = r#"x = "unclosed"#;
        assert!(matches!(
            ParserPy::parse(invalid_syntax_src, "prompts.py"),
            ParseResult::ParseResultError(_)
        ));
    }

    //
}
