use std::collections::{HashMap, HashSet};

use ruff_python_ast as ast;
use ruff_python_ast::visitor::{self, Visitor};
use ruff_python_parser::{self as parser, TokenKind};
use ruff_text_size::{Ranged, TextRange};
pub use volumen_parser_core::VolumenParser;
use volumen_parser_core::*;
use volumen_types::*;

pub struct ParserPy {}

impl VolumenParser for ParserPy {
    fn parse(source: &str, filename: &str) -> ParseResult {
        let parsed = match parser::parse_module(source) {
            Ok(parsed) => parsed,
            Err(err) => {
                return ParseResult::ParseResultError(ParseResultError {
                    state: ParseResultErrorStateError,
                    error: format!("{}", err),
                });
            }
        };

        let comments = ParserPy::parse_comments(source, &parsed);

        let mut visitor = PyPromptVisitor::new(source, filename.to_string(), comments);
        visitor.visit_body(parsed.suite());

        ParseResult::ParseResultSuccess(ParseResultSuccess {
            state: ParseResultSuccessStateSuccess,
            prompts: visitor.prompts,
        })
    }
}

impl ParserPy {
    fn parse_comments(source: &str, parsed: &parser::Parsed<ast::ModModule>) -> Vec<TextRange> {
        // Collect ALL comments, not only those with @prompt. We need full comment
        // context to correctly group contiguous leading comment blocks (where the
        // @prompt marker may appear on any line within the block).
        let mut comments: Vec<TextRange> = Vec::new();

        for token in parsed.tokens() {
            let kind = token.kind();
            let range = token.range();
            if kind == TokenKind::Comment {
                let _ = &source[range];
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
    /// Current statement range stack.
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

    fn parse_fstr_vars(&self, fstr: &ast::ExprFString) -> Vec<PromptVar> {
        let mut vars: Vec<PromptVar> = Vec::new();
        for part in fstr.value.as_slice() {
            if let ast::FStringPart::FString(inner) = part {
                for element in &inner.elements {
                    if let ast::InterpolatedStringElement::Interpolation(interp) = element {
                        let range = interp.range();
                        vars.push(PromptVar {
                            exp: self.code[range].to_string(),
                            span: SpanShape {
                                outer: self.span(range),
                                inner: Span {
                                    start: self.span(range).start + 1,
                                    end: self.span(range).end.saturating_sub(1),
                                },
                            },
                        });
                    }
                }
            }
        }
        vars
    }

    fn parse_tstr_vars(&self, tstr: &ast::ExprTString) -> Vec<PromptVar> {
        let mut vars: Vec<PromptVar> = Vec::new();
        for element in tstr.value.elements() {
            if let ast::InterpolatedStringElement::Interpolation(interp) = element {
                let r = interp.range();
                vars.push(PromptVar {
                    exp: self.code[r].to_string(),
                    span: SpanShape {
                        outer: self.span(r),
                        inner: Span {
                            start: self.span(r).start + 1,
                            end: self.span(r).end.saturating_sub(1),
                        },
                    },
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
                ast::Expr::FString(expr) => self.process_fstr(ident, expr),
                ast::Expr::StringLiteral(expr) => self.process_str_literal(ident, expr),
                ast::Expr::TString(expr) => self.process_tstr(ident, expr),
                _ => {}
            }
        }
    }

    fn process_str_literal(&mut self, ident_name: &str, str: &ast::ExprStringLiteral) {
        self.process_range(ident_name, str.range(), Vec::new());
    }

    fn process_tstr(&mut self, ident_name: &str, tstr: &ast::ExprTString) {
        let vars = self.parse_tstr_vars(tstr);
        self.process_range(ident_name, tstr.range(), vars);
    }

    fn process_fstr(&mut self, ident_name: &str, fstr: &ast::ExprFString) {
        let vars = self.parse_fstr_vars(fstr);
        self.process_range(ident_name, fstr.range(), vars);
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

        // Check if current annotations contain at least one valid @prompt
        let has_valid_current_annotation = annotations
            .iter()
            .any(|a| parse_annotation(&a.exp).unwrap_or(false));

        // If no valid annotations in current statement, use stored definition annotations
        if !has_valid_current_annotation
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

        let stmt_range = self.stmt_range_stack.last().copied().unwrap_or(node_range);
        let leading_start = self
            .stmt_leading_start_stack
            .last()
            .copied()
            .flatten()
            .unwrap_or(self.span(stmt_range).start);
        let enclosure = Span {
            start: leading_start,
            end: self.span(stmt_range).end,
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
    fn collect_adjacent_leading_comments(&self, stmt: &'a ast::Stmt) -> Vec<PromptAnnotation> {
        let stmt_start = stmt.range().start();
        let mut block_ranges: Vec<TextRange> = Vec::new();
        let mut idx: isize = (self.comments.len() as isize) - 1;
        while idx >= 0 {
            let r = self.comments[idx as usize];
            if r.end() <= stmt_start {
                let between = &self.code[r.end().to_usize()..stmt_start.to_usize()];
                if between.trim().is_empty() {
                    let mut j = idx;
                    let mut last = stmt_start;
                    while j >= 0 {
                        let rr = self.comments[j as usize];
                        if rr.end() <= last {
                            let between2 = &self.code[rr.end().to_usize()..last.to_usize()];
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

        // Merge the contiguous block into a single annotation. Whether it contains
        // @prompt or not will be decided later when determining if the statement
        // is a prompt; we still keep the full leading block for context.
        let first = block_ranges.first().unwrap();
        let last = block_ranges.last().unwrap();
        let start = first.start().to_u32();
        let end = last.end().to_u32();
        let block_text = &self.code[TextRange::new(first.start(), last.end())];
        vec![PromptAnnotation {
            span: Span { start, end },
            exp: block_text.to_string(),
        }]
    }

    fn collect_inline_prompt_comments(&self, stmt: &'a ast::Stmt) -> Vec<PromptAnnotation> {
        let r = stmt.range();
        let mut out: Vec<PromptAnnotation> = Vec::new();
        for &cr in &self.comments {
            if cr.start() >= r.start() && cr.start() < r.end() {
                let text = self.code[cr].to_string();
                if parse_annotation(&text).unwrap_or(false) {
                    out.push(PromptAnnotation {
                        span: Span {
                            start: cr.start().to_u32(),
                            end: cr.end().to_u32(),
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

impl<'a> Visitor<'a> for PyPromptVisitor<'a> {
    fn visit_stmt(&mut self, stmt: &'a ast::Stmt) {
        let leading = self.collect_adjacent_leading_comments(stmt);
        let inline = self.collect_inline_prompt_comments(stmt);
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
        self.stmt_range_stack.push(stmt.range());

        // Process assignments.
        match stmt {
            ast::Stmt::Assign(assign) => {
                for target in &assign.targets {
                    self.process_assign_target(is_prompt, target, Some(&assign.value));
                }
            }

            ast::Stmt::AnnAssign(assign) => {
                // Record annotated identifiers
                if let ast::Expr::Name(name) = &*assign.target {
                    self.annotated_idents.insert(name.id.to_string());
                    if is_prompt
                        && let Some(ann) = self.stmt_annotations_stack.last()
                        && !ann.is_empty()
                    {
                        self.def_prompt_annotations
                            .insert(name.id.to_string(), ann.clone());
                    }
                }
                self.process_assign_target(is_prompt, &assign.target, assign.value.as_deref());
            }

            _ => {}
        }

        // Check if we are entering a new scope.
        let new_scope = matches!(stmt, ast::Stmt::FunctionDef(_) | ast::Stmt::ClassDef(_));
        if new_scope {
            self.prompt_idents_stack.push(HashSet::new());
        }

        // Visit nested statements.
        visitor::walk_stmt(self, stmt);

        // If we opened a scope, pop it off the stack.
        if new_scope {
            self.prompt_idents_stack.pop();
        }

        self.stmt_annotations_stack.pop();
        self.stmt_leading_start_stack.pop();
        self.stmt_range_stack.pop();
    }
}
