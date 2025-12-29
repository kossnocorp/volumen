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
        (
            range.start().to_usize() as u32,
            range.end().to_usize() as u32,
        )
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
        let inner_end = outer.1.saturating_sub(quote_len);
        let inner = (inner_start, inner_end);

        SpanShape { outer, inner }
    }

    fn build_content_tokens(&self, span: &SpanShape, vars: &[PromptVar]) -> Vec<PromptContentToken> {
        if vars.is_empty() {
            // Simple case: single str token
            return vec![PromptContentToken::PromptContentTokenStr(
                PromptContentTokenStr {
                    r#type: PromptContentTokenStrTypeStr,
                    span: span.inner,
                },
            )];
        }

        let mut tokens = Vec::new();
        let mut pos = span.inner.0;

        for (var_idx, var) in vars.iter().enumerate() {
            // Add str token before variable (if any content)
            if pos < var.span.outer.0 {
                tokens.push(PromptContentToken::PromptContentTokenStr(
                    PromptContentTokenStr {
                        r#type: PromptContentTokenStrTypeStr,
                        span: (pos, var.span.outer.0),
                    },
                ));
            }

            // Add var token
            tokens.push(PromptContentToken::PromptContentTokenVar(
                PromptContentTokenVar {
                    r#type: PromptContentTokenVarTypeVar,
                    span: var.span.outer,
                    index: var_idx as u32,
                },
            ));

            pos = var.span.outer.1;
        }

        // Add trailing str token (if any content)
        if pos < span.inner.1 {
            tokens.push(PromptContentToken::PromptContentTokenStr(
                PromptContentTokenStr {
                    r#type: PromptContentTokenStrTypeStr,
                    span: (pos, span.inner.1),
                },
            ));
        }

        tokens
    }

    fn parse_fstr_vars(&self, fstr: &ast::ExprFString) -> Vec<PromptVar> {
        let mut vars: Vec<PromptVar> = Vec::new();
        for part in fstr.value.as_slice() {
            if let ast::FStringPart::FString(inner) = part {
                for element in &inner.elements {
                    if let ast::InterpolatedStringElement::Interpolation(interp) = element {
                        let range = interp.range();
                        vars.push(PromptVar {
                            span: SpanShape {
                                outer: self.span(range),
                                inner: (
                                    self.span(range).0 + 1,
                                    self.span(range).1.saturating_sub(1),
                                ),
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
                    span: SpanShape {
                        outer: self.span(r),
                        inner: (self.span(r).0 + 1, self.span(r).1.saturating_sub(1)),
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
                ast::Expr::BinOp(expr) => {
                    // Handle concatenation: "Hello, " + name + "!"
                    if let Some(prompt) = self.process_concatenation(ident, expr) {
                        self.prompts.push(prompt);
                    }
                }
                ast::Expr::Call(call_expr) => {
                    // First check if it's a join call: "\n".join([...])
                    if let Some(prompt) = self.process_join_call(ident, call_expr) {
                        self.prompts.push(prompt);
                    // Otherwise check if it's a format method: "Hello {}".format(name)
                    } else if let Some(prompt) = self.process_format_call(ident, call_expr) {
                        self.prompts.push(prompt);
                    }
                }
                ast::Expr::List(list_expr) => {
                    // Handle array: ["Hello ", user, "!"]
                    if let Some(prompt) = self.process_array(ident, list_expr) {
                        self.prompts.push(prompt);
                    }
                }
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

        // Annotations from comment tracker are already validated to contain @prompt
        // If no annotations in current statement, use stored definition annotations
        if annotations.is_empty()
            && self.annotated_idents.contains(ident)
            && let Some(def) = self.def_prompt_annotations.get(ident)
        {
            annotations = def.clone();
        }
        // Annotations are already validated to contain @prompt
        let has_prompt_annotation = !annotations.is_empty();
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
            .unwrap_or(self.span(stmt_range).0);
        let enclosure = (leading_start, self.span(stmt_range).1);

        let span = self.span_shape_string_like(node_range);
        let content = self.build_content_tokens(&span, &vars);

        let prompt = Prompt {
            file: self.file.clone(),
            span,
            enclosure,
            vars,
            annotations,
            content,
            joint: SpanShape {
                outer: (0, 0),
                inner: (0, 0),
            },
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

        // Check if any comment in the block contains @prompt
        let has_prompt = block_ranges
            .iter()
            .any(|cr| {
                let text = &self.code[*cr];
                parse_annotation(text).unwrap_or(false)
            });
        if !has_prompt {
            return Vec::new();
        }

        // Merge the contiguous block into a single annotation with multiple span shapes
        let first = block_ranges.first().unwrap();
        let last = block_ranges.last().unwrap();
        let start = first.start().to_u32();
        let end = last.end().to_u32();
        let block_text = &self.code[TextRange::new(first.start(), last.end())];

        let spans: Vec<SpanShape> = block_ranges
            .iter()
            .map(|cr| {
                let text = &self.code[*cr];
                let (inner_start_offset, inner_end_offset) = compute_comment_inner_offsets(text);
                let s = cr.start().to_u32();
                let e = cr.end().to_u32();
                SpanShape {
                    outer: (s, e),
                    inner: (s + inner_start_offset, s + inner_end_offset),
                }
            })
            .collect();

        vec![PromptAnnotation {
            spans,
        }]
    }

    fn collect_inline_prompt_comments(&self, stmt: &'a ast::Stmt) -> Vec<PromptAnnotation> {
        let r = stmt.range();
        let mut out: Vec<PromptAnnotation> = Vec::new();
        for &cr in &self.comments {
            if cr.start() >= r.start() && cr.start() < r.end() {
                let text = self.code[cr].to_string();
                if parse_annotation(&text).unwrap_or(false) {
                    let s = cr.start().to_u32();
                    let e = cr.end().to_u32();
                    let (inner_start_offset, inner_end_offset) = compute_comment_inner_offsets(&text);
                    out.push(PromptAnnotation {
                        spans: vec![SpanShape {
                            outer: (s, e),
                            inner: (s + inner_start_offset, s + inner_end_offset),
                        }],
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
        let leading_start = leading.first().and_then(|a| a.spans.first()).map(|s| s.outer.0);
        for a in leading.into_iter().chain(inline.into_iter()) {
            annotations.push(a);
        }
        // Annotations are already validated to contain @prompt
        let is_prompt = !annotations.is_empty();
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

// Concatenation support

#[derive(Debug)]
enum ConcatSegment {
    String(SpanShape),
    Variable(SpanShape),
    Primitive(SpanShape),
    Other,
}

impl<'a> PyPromptVisitor<'a> {
    /// Process a binary operator for string concatenation
    fn process_concatenation(&mut self, ident_name: &str, binop: &ast::ExprBinOp) -> Option<Prompt> {
        // Check if it's an Add operator
        if !matches!(binop.op, ast::Operator::Add) {
            return None;
        }

        // Check if ident is a prompt variable
        let in_prompt_ident = self
            .prompt_idents_stack
            .iter()
            .rev()
            .any(|s| s.contains(ident_name));
        let mut annotations: Vec<PromptAnnotation> = self
            .stmt_annotations_stack
            .last()
            .cloned()
            .unwrap_or_default();

        if annotations.is_empty()
            && self.annotated_idents.contains(ident_name)
            && let Some(def) = self.def_prompt_annotations.get(ident_name)
        {
            annotations = def.clone();
        }
        let has_prompt_annotation = !annotations.is_empty();
        let is_prompt =
            ident_name.to_lowercase().contains("prompt") || in_prompt_ident || has_prompt_annotation;
        if !is_prompt {
            return None;
        }

        // Extract segments recursively
        let segments = self.extract_concat_segments(&ast::Expr::BinOp(binop.clone()));

        // Reject if no strings or contains complex objects
        let has_string = segments.iter().any(|s| matches!(s, ConcatSegment::String(_)));
        let has_other = segments.iter().any(|s| matches!(s, ConcatSegment::Other));

        if !has_string || has_other {
            return None;
        }

        // Build prompt outer span (entire concatenation expression)
        let prompt_outer = self.span(binop.range());

        // Find first and last string segments to determine inner span
        let first_string_pos = segments.iter().find_map(|s| match s {
            ConcatSegment::String(span) => Some(span.inner.0),
            _ => None,
        })?;

        let last_string_end = segments.iter().rev().find_map(|s| match s {
            ConcatSegment::String(span) => Some(span.inner.1),
            _ => None,
        })?;

        let prompt_inner = (first_string_pos, last_string_end);
        let span = SpanShape {
            outer: prompt_outer,
            inner: prompt_inner,
        };

        // Build vars and content tokens
        let mut vars = Vec::new();
        let mut content = Vec::new();
        let mut var_idx = 0u32;

        for segment in &segments {
            match segment {
                ConcatSegment::String(s_span) => {
                    // Add string token
                    content.push(PromptContentToken::PromptContentTokenStr(
                        PromptContentTokenStr {
                            r#type: PromptContentTokenStrTypeStr,
                            span: (s_span.inner.0, s_span.inner.1),
                        },
                    ));
                }
                ConcatSegment::Variable(v_span) | ConcatSegment::Primitive(v_span) => {
                    // Expand to include operators
                    let mut var_outer = v_span.clone();
                    self.expand_to_operators(&mut var_outer, prompt_outer);

                    let var = PromptVar {
                        span: SpanShape {
                            outer: var_outer.outer,
                            inner: v_span.inner,
                        },
                    };

                    // Add variable token (before pushing var)
                    content.push(PromptContentToken::PromptContentTokenVar(
                        PromptContentTokenVar {
                            r#type: PromptContentTokenVarTypeVar,
                            span: var.span.inner,
                            index: var_idx,
                        },
                    ));

                    vars.push(var);
                    var_idx += 1;
                }
                ConcatSegment::Other => {}
            }
        }

        // Calculate enclosure
        let stmt_range = self.stmt_range_stack.last().copied().unwrap_or(binop.range());
        let leading_start = self
            .stmt_leading_start_stack
            .last()
            .copied()
            .flatten()
            .unwrap_or(self.span(stmt_range).0);
        let enclosure = (leading_start, self.span(stmt_range).1);

        Some(Prompt {
            file: self.file.clone(),
            span,
            enclosure,
            vars,
            annotations,
            content,
            joint: SpanShape {
                outer: (0, 0),
                inner: (0, 0),
            },
        })
    }

    /// Extract segments from a concatenation expression
    fn extract_concat_segments(&self, expr: &ast::Expr) -> Vec<ConcatSegment> {
        match expr {
            ast::Expr::BinOp(binop) if matches!(binop.op, ast::Operator::Add) => {
                // Recursively process left and right
                let mut segments = Vec::new();
                segments.extend(self.extract_concat_segments(&binop.left));
                segments.extend(self.extract_concat_segments(&binop.right));
                segments
            }
            _ => self.classify_single_node(expr),
        }
    }

    /// Classify a single node as a segment
    fn classify_single_node(&self, expr: &ast::Expr) -> Vec<ConcatSegment> {
        match expr {
            ast::Expr::StringLiteral(_) | ast::Expr::FString(_) | ast::Expr::TString(_) => {
                // String literal
                let span = self.span_shape_string_like(expr.range());
                vec![ConcatSegment::String(span)]
            }
            ast::Expr::Name(_) => {
                // Variable
                let outer = self.span(expr.range());
                let inner = outer;
                vec![ConcatSegment::Variable(SpanShape { outer, inner })]
            }
            ast::Expr::Call(_) => {
                // Function call - treat as variable
                let outer = self.span(expr.range());
                let inner = outer;
                vec![ConcatSegment::Variable(SpanShape { outer, inner })]
            }
            ast::Expr::Attribute(_) => {
                // Member access (obj.prop or obj.method()) - treat as variable
                let outer = self.span(expr.range());
                let inner = outer;
                vec![ConcatSegment::Variable(SpanShape { outer, inner })]
            }
            ast::Expr::NumberLiteral(_) | ast::Expr::BooleanLiteral(_) => {
                // Primitives - Python requires str() conversion, so this would be a type error
                // But we can still parse it as a primitive for completeness
                let outer = self.span(expr.range());
                let inner = outer;
                vec![ConcatSegment::Primitive(SpanShape { outer, inner })]
            }
            ast::Expr::List(_) | ast::Expr::Tuple(_) | ast::Expr::Dict(_) | ast::Expr::Set(_) => {
                // Complex objects - reject
                vec![ConcatSegment::Other]
            }
            _ => {
                // Unknown - reject
                vec![ConcatSegment::Other]
            }
        }
    }

    /// Expand variable span to include surrounding operators
    fn expand_to_operators(&self, var_span: &mut SpanShape, prompt_outer: (u32, u32)) {
        let bytes = self.code.as_bytes();
        let mut new_start = var_span.outer.0;
        let mut new_end = var_span.outer.1;

        // Expand left to include " + "
        let mut i = new_start as usize;
        while i > prompt_outer.0 as usize {
            i -= 1;
            let c = bytes[i];
            if c == b'+' {
                // Include the + and any spaces before it
                while i > prompt_outer.0 as usize && (bytes[i - 1] == b' ' || bytes[i - 1] == b'\t') {
                    i -= 1;
                }
                new_start = i as u32;
                break;
            } else if c != b' ' && c != b'\t' {
                break;
            }
        }

        // Expand right to include " + "
        let mut i = new_end as usize;
        while i < prompt_outer.1 as usize {
            let c = bytes[i];
            if c == b'+' {
                // Include the + and any spaces after it
                i += 1;
                while i < prompt_outer.1 as usize && (bytes[i] == b' ' || bytes[i] == b'\t') {
                    i += 1;
                }
                new_end = i as u32;
                break;
            } else if c != b' ' && c != b'\t' {
                break;
            }
            i += 1;
        }

        var_span.outer = (new_start, new_end);
    }

    /// Process format method call: "Hello {}".format(name)
    fn process_format_call(&mut self, ident: &str, call: &ast::ExprCall) -> Option<Prompt> {
        // Check if this is a .format() method call on a string
        let attr = call.func.as_attribute_expr()?;
        if attr.attr.as_str() != "format" {
            return None;
        }

        // The value should be a string literal
        let format_str = attr.value.as_string_literal_expr()?;
        
        // Check if this should be treated as a prompt
        let in_prompt_ident = self
            .prompt_idents_stack
            .iter()
            .rev()
            .any(|s| s.contains(ident));
        let annotations: Vec<PromptAnnotation> = self
            .stmt_annotations_stack
            .last()
            .cloned()
            .unwrap_or_default();
        
        if !in_prompt_ident && annotations.is_empty() && !ident.to_lowercase().contains("prompt") {
            return None;
        }

        // Extract the format string content
        let format_str_value = format_str.value.to_str();
        let format_str_range = format_str.range;
        
        // Parse placeholders in the format string
        let placeholders = self.parse_format_placeholders(&format_str_value);
        
        // Build vars from the format() arguments
        let mut vars = Vec::new();
        for (arg_idx, arg) in call.arguments.args.iter().enumerate() {
            // Skip if we have more args than placeholders (shouldn't happen in valid code)
            if arg_idx >= placeholders.len() {
                break;
            }
            
            let arg_range = arg.range();
            vars.push(PromptVar {
                span: SpanShape {
                    outer: self.span(arg_range),
                    inner: self.span(arg_range),
                },
            });
        }
        
        // Build content tokens with placeholders as var positions
        let mut content = Vec::new();
        let format_inner_start = self.span(format_str_range).0 + 1;
        let format_inner_end = self.span(format_str_range).1 - 1;
        let mut pos = 0usize;
        
        for (placeholder_idx, (start, end)) in placeholders.iter().enumerate() {
            // Add string token before placeholder
            if pos < *start {
                content.push(PromptContentToken::PromptContentTokenStr(
                    PromptContentTokenStr {
                        r#type: PromptContentTokenStrTypeStr,
                        span: (format_inner_start + pos as u32, format_inner_start + *start as u32),
                    },
                ));
            }
            
            // Add var token for placeholder (if we have a corresponding arg)
            if placeholder_idx < vars.len() {
                content.push(PromptContentToken::PromptContentTokenVar(
                    PromptContentTokenVar {
                        r#type: PromptContentTokenVarTypeVar,
                        span: (format_inner_start + *start as u32, format_inner_start + *end as u32),
                        index: placeholder_idx as u32,
                    },
                ));
            }
            
            pos = *end;
        }
        
        // Add trailing string token
        if pos < format_str_value.len() {
            content.push(PromptContentToken::PromptContentTokenStr(
                PromptContentTokenStr {
                    r#type: PromptContentTokenStrTypeStr,
                    span: (format_inner_start + pos as u32, format_inner_end),
                },
            ));
        }
        
        let call_range = call.range();
        let stmt_range = self.stmt_range_stack.last().copied().unwrap_or(call_range);
        let leading_start = self
            .stmt_leading_start_stack
            .last()
            .copied()
            .flatten()
            .unwrap_or(self.span(stmt_range).0);
        
        Some(Prompt {
            file: self.file.clone(),
            span: SpanShape {
                outer: self.span(call_range),
                inner: (format_inner_start, format_inner_end),
            },
            enclosure: (leading_start, self.span(stmt_range).1),
            vars,
            annotations,
            content,
            joint: SpanShape {
                outer: (0, 0),
                inner: (0, 0),
            },
        })
    }
    
    /// Parse format string placeholders: {}, {0}, {1}, {name}, etc.
    /// Returns vec of (start, end) positions in the string
    fn parse_format_placeholders(&self, format_str: &str) -> Vec<(usize, usize)> {
        let mut placeholders = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = format_str.chars().collect();
        
        while i < chars.len() {
            if chars[i] == '{' {
                // Check for escaped {{ 
                if i + 1 < chars.len() && chars[i + 1] == '{' {
                    i += 2; // Skip escaped {{
                    continue;
                }
                
                let start = i;
                i += 1;
                
                // Find closing }
                while i < chars.len() && chars[i] != '}' {
                    i += 1;
                }
                
                if i < chars.len() && chars[i] == '}' {
                    // Check for escaped }}
                    if i + 1 < chars.len() && chars[i + 1] == '}' {
                        i += 2;
                        continue;
                    }
                    
                    i += 1; // Include the }
                    placeholders.push((start, i));
                    continue;
                }
            }
            i += 1;
        }
        
        placeholders
    }

    /// Process an array/list assignment: ["Hello ", user, "!"]
    fn process_array(&mut self, ident: &str, list: &ast::ExprList) -> Option<Prompt> {
        // Check if this should be treated as a prompt
        let in_prompt_ident = self
            .prompt_idents_stack
            .iter()
            .rev()
            .any(|s| s.contains(ident));
        let annotations: Vec<PromptAnnotation> = self
            .stmt_annotations_stack
            .last()
            .cloned()
            .unwrap_or_default();
        
        if !in_prompt_ident && annotations.is_empty() && !ident.to_lowercase().contains("prompt") {
            return None;
        }

        // Extract array elements
        let mut vars = Vec::new();
        let mut content = Vec::new();
        let mut var_idx = 0u32;

        for element in &list.elts {
            match element {
                ast::Expr::StringLiteral(string) => {
                    // String literal
                    let span = self.span_shape_string_like(string.range);
                    content.push(PromptContentToken::PromptContentTokenStr(
                        PromptContentTokenStr {
                            r#type: PromptContentTokenStrTypeStr,
                            span: span.inner,
                        },
                    ));
                }
                _ => {
                    // Variables and other expressions
                    let elem_range = element.range();
                    vars.push(PromptVar {
                        span: SpanShape {
                            outer: self.span(elem_range),
                            inner: self.span(elem_range),
                        },
                    });
                    content.push(PromptContentToken::PromptContentTokenVar(
                        PromptContentTokenVar {
                            r#type: PromptContentTokenVarTypeVar,
                            span: self.span(elem_range),
                            index: var_idx,
                        },
                    ));
                    var_idx += 1;
                }
            }
        }

        // Build span: outer is entire array including brackets, inner is content without brackets
        let list_range = list.range();
        let outer = self.span(list_range);
        let inner = (outer.0 + 1, outer.1.saturating_sub(1));
        let span = SpanShape { outer, inner };

        // Calculate enclosure
        let stmt_range = self.stmt_range_stack.last().copied().unwrap_or(list_range);
        let leading_start = self
            .stmt_leading_start_stack
            .last()
            .copied()
            .flatten()
            .unwrap_or(self.span(stmt_range).0);
        let enclosure = (leading_start, self.span(stmt_range).1);

        Some(Prompt {
            file: self.file.clone(),
            span,
            enclosure,
            vars,
            annotations,
            content,
            joint: SpanShape {
                outer: (0, 0),
                inner: (0, 0),
            },
        })
    }

    /// Process a join call: "\n".join(["Hello", user, "!"])
    fn process_join_call(&mut self, ident: &str, call: &ast::ExprCall) -> Option<Prompt> {
        // Check if this is a .join() method call
        let attr = call.func.as_attribute_expr()?;
        if attr.attr.as_str() != "join" {
            return None;
        }

        // The value should be a string literal (separator)
        let sep_str = attr.value.as_string_literal_expr()?;
        
        // First argument should be a list
        let list_arg = call.arguments.args.first()?;
        let list = list_arg.as_list_expr()?;

        // Check if this should be treated as a prompt
        let in_prompt_ident = self
            .prompt_idents_stack
            .iter()
            .rev()
            .any(|s| s.contains(ident));
        let annotations: Vec<PromptAnnotation> = self
            .stmt_annotations_stack
            .last()
            .cloned()
            .unwrap_or_default();
        
        if !in_prompt_ident && annotations.is_empty() && !ident.to_lowercase().contains("prompt") {
            return None;
        }

        // Extract separator span
        let joint = self.span_shape_string_like(sep_str.range);

        // Extract array elements
        let mut vars = Vec::new();
        let mut content = Vec::new();
        let mut var_idx = 0u32;
        let mut first = true;

        for element in &list.elts {
            match element {
                ast::Expr::StringLiteral(string) => {
                    // Insert joint token between elements (not before first)
                    if !first && (joint.outer.0 != 0 || joint.outer.1 != 0) {
                        content.push(PromptContentToken::PromptContentTokenJoint(
                            PromptContentTokenJoint {
                                r#type: PromptContentTokenJointTypeJoint,
                            },
                        ));
                    }
                    first = false;
                    // String literal
                    let span = self.span_shape_string_like(string.range);
                    content.push(PromptContentToken::PromptContentTokenStr(
                        PromptContentTokenStr {
                            r#type: PromptContentTokenStrTypeStr,
                            span: span.inner,
                        },
                    ));
                }
                _ => {
                    // Insert joint token between elements (not before first)
                    if !first && (joint.outer.0 != 0 || joint.outer.1 != 0) {
                        content.push(PromptContentToken::PromptContentTokenJoint(
                            PromptContentTokenJoint {
                                r#type: PromptContentTokenJointTypeJoint,
                            },
                        ));
                    }
                    first = false;
                    // Variables and other expressions
                    let elem_range = element.range();
                    vars.push(PromptVar {
                        span: SpanShape {
                            outer: self.span(elem_range),
                            inner: self.span(elem_range),
                        },
                    });
                    content.push(PromptContentToken::PromptContentTokenVar(
                        PromptContentTokenVar {
                            r#type: PromptContentTokenVarTypeVar,
                            span: self.span(elem_range),
                            index: var_idx,
                        },
                    ));
                    var_idx += 1;
                }
            }
        }

        // Build span: outer is entire call expression, inner is array content without brackets
        let call_range = call.range();
        let list_range = list.range();
        let outer = self.span(call_range);
        let list_outer = self.span(list_range);
        let inner = (list_outer.0 + 1, list_outer.1.saturating_sub(1));
        let span = SpanShape { outer, inner };

        // Calculate enclosure
        let stmt_range = self.stmt_range_stack.last().copied().unwrap_or(call_range);
        let leading_start = self
            .stmt_leading_start_stack
            .last()
            .copied()
            .flatten()
            .unwrap_or(self.span(stmt_range).0);
        let enclosure = (leading_start, self.span(stmt_range).1);

        Some(Prompt {
            file: self.file.clone(),
            span,
            enclosure,
            vars,
            annotations,
            content,
            joint,
        })
    }
}
