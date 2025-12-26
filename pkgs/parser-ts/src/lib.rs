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
        (span.start, span.end)
    }

    fn span_shape_literal(&self, span: &oxc_span::Span) -> SpanShape {
        let outer = self.span_outer(span);
        let inner_start = outer.0.saturating_add(1);
        let inner_end = outer.1.saturating_sub(1);
        let inner = (inner_start, inner_end);
        SpanShape { outer, inner }
    }

    fn process_variable_declarator(
        &mut self,
        declarator: &ast::VariableDeclarator<'a>,
        has_stmt_prompt: bool,
    ) {
        // Check if this is a destructuring pattern (object or array)
        let is_destructuring = matches!(
            &declarator.id.kind,
            ast::BindingPatternKind::ObjectPattern(_) | ast::BindingPatternKind::ArrayPattern(_)
        );

        if is_destructuring && has_stmt_prompt {
            // Handle destructuring patterns
            if let Some(init) = &declarator.init {
                // Extract all identifiers from the left side
                let mut identifiers = Vec::new();
                self.extract_binding_identifiers(&declarator.id, &mut identifiers);

                // Extract all string/template values from the right side
                let mut values = Vec::new();
                self.extract_expression_values(init, &mut values);

                // Match identifiers with values by position
                for (i, ident_name) in identifiers.iter().enumerate() {
                    if let Some((value_span, is_template)) = values.get(i) {
                        if self.is_prompt(ident_name, has_stmt_prompt) {
                            // Mark this identifier as a prompt variable
                            if let Some(scope) = self.prompt_idents_stack.last_mut() {
                                scope.insert(ident_name.clone());
                            }

                            // Store definition-time annotations
                            if let Some(ann) = self.stmt_annotations_stack.last()
                                && !ann.is_empty()
                                && let Some(scope) = self.def_prompt_annotations_stack.last_mut()
                            {
                                scope.insert(ident_name.clone(), ann.clone());
                            }

                            // Create a prompt for this value
                            let (has_prompt, annotations, enclosure) =
                                self.resolve_prompt_meta(ident_name, value_span);

                            if has_prompt {
                                let span = self.span_shape_literal(value_span);
                                let exp = value_span.source_text(self.code).to_string();

                                let vars = if *is_template {
                                    // For template literals, we need to extract the original template
                                    // from the span. Since we only have a span, we'll create an empty
                                    // vars list for now. A proper implementation would parse the template.
                                    Vec::new()
                                } else {
                                    Vec::new()
                                };

                                let prompt = Prompt {
                                    file: self.file.clone(),
                                    span,
                                    enclosure,
                                    exp,
                                    vars,
                                    annotations,
                                };
                                self.prompts.push(prompt);
                            }
                        }
                    }
                }
            }
        } else if let ast::BindingPatternKind::BindingIdentifier(ident) = &declarator.id.kind {
            // Handle simple identifier (existing code)
            if has_stmt_prompt {
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

                    ast::Expression::AssignmentExpression(assign_expr) => {
                        // Handle chained assignment in initializer: const hello = world = "Hi"
                        // Walk the chain to find the ultimate value
                        let mut current = assign_expr.as_ref();
                        loop {
                            match &current.right {
                                ast::Expression::AssignmentExpression(inner) => {
                                    // Continue walking the chain
                                    current = inner.as_ref();
                                }
                                ast::Expression::TemplateLiteral(template) => {
                                    // Found the ultimate value - process it for current identifier
                                    self.process_template_literal(&ident.name, template);
                                    break;
                                }
                                ast::Expression::StringLiteral(string_literal) => {
                                    // Found the ultimate value - process it for current identifier
                                    self.process_string_literal(&ident.name, string_literal);
                                    break;
                                }
                                _ => {
                                    // Not a string/template value, stop
                                    break;
                                }
                            }
                        }

                        // Note: Don't manually call process_assignment_expression here
                        // The visitor will automatically handle it via enter_node
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

                ast::Expression::AssignmentExpression(nested_assign) => {
                    // Handle chained assignment: a = b = "value"
                    // Walk the chain to find the ultimate value
                    let mut current = nested_assign.as_ref();
                    loop {
                        match &current.right {
                            ast::Expression::AssignmentExpression(inner) => {
                                // Continue walking the chain
                                current = inner.as_ref();
                            }
                            ast::Expression::TemplateLiteral(template) => {
                                // Found the ultimate value - process it for current identifier
                                self.process_template_literal(&ident.name, template);
                                break;
                            }
                            ast::Expression::StringLiteral(string_literal) => {
                                // Found the ultimate value - process it for current identifier
                                self.process_string_literal(&ident.name, string_literal);
                                break;
                            }
                            _ => {
                                // Not a string/template value, stop
                                break;
                            }
                        }
                    }

                    // Recursively process the nested assignment
                    self.process_assignment_expression(nested_assign.as_ref());
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
            let outer = (start, end);
            let inner = (expr_span.start, expr_span.end);
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

    /// Recursively extract all binding identifiers from a binding pattern
    fn extract_binding_identifiers(
        &self,
        pattern: &ast::BindingPattern<'a>,
        identifiers: &mut Vec<String>,
    ) {
        match &pattern.kind {
            ast::BindingPatternKind::BindingIdentifier(ident) => {
                identifiers.push(ident.name.to_string());
            }
            ast::BindingPatternKind::ObjectPattern(obj_pattern) => {
                for prop in &obj_pattern.properties {
                    self.extract_binding_identifiers(&prop.value, identifiers);
                }
                if let Some(rest) = &obj_pattern.rest {
                    self.extract_binding_identifiers(&rest.argument, identifiers);
                }
            }
            ast::BindingPatternKind::ArrayPattern(array_pattern) => {
                for element in array_pattern.elements.iter().flatten() {
                    self.extract_binding_identifiers(element, identifiers);
                }
                if let Some(rest) = &array_pattern.rest {
                    self.extract_binding_identifiers(&rest.argument, identifiers);
                }
            }
            ast::BindingPatternKind::AssignmentPattern(assign_pattern) => {
                self.extract_binding_identifiers(&assign_pattern.left, identifiers);
            }
        }
    }

    /// Helper to extract values from an array expression
    fn extract_values_from_array(
        &self,
        array: &ast::ArrayExpression<'a>,
        values: &mut Vec<(oxc_span::Span, bool)>,
    ) {
        for element in array.elements.iter() {
            match element {
                ast::ArrayExpressionElement::SpreadElement(_) => {}
                ast::ArrayExpressionElement::Elision(_) => {}
                ast::ArrayExpressionElement::StringLiteral(string) => {
                    values.push((string.span, false));
                }
                ast::ArrayExpressionElement::TemplateLiteral(template) => {
                    values.push((template.span, true));
                }
                ast::ArrayExpressionElement::ArrayExpression(nested_array) => {
                    self.extract_values_from_array(nested_array, values);
                }
                ast::ArrayExpressionElement::ObjectExpression(obj) => {
                    self.extract_values_from_object(obj, values);
                }
                _ => {}
            }
        }
    }

    /// Helper to extract values from an object expression
    fn extract_values_from_object(
        &self,
        obj: &ast::ObjectExpression<'a>,
        values: &mut Vec<(oxc_span::Span, bool)>,
    ) {
        for prop in &obj.properties {
            match prop {
                ast::ObjectPropertyKind::ObjectProperty(obj_prop) => {
                    self.extract_expression_values(&obj_prop.value, values);
                }
                ast::ObjectPropertyKind::SpreadProperty(_) => {}
            }
        }
    }

    /// Recursively extract all string/template literal values from an expression
    fn extract_expression_values(
        &self,
        expr: &ast::Expression<'a>,
        values: &mut Vec<(oxc_span::Span, bool)>,
    ) {
        match expr {
            ast::Expression::StringLiteral(string) => {
                values.push((string.span, false)); // false = string literal
            }
            ast::Expression::TemplateLiteral(template) => {
                values.push((template.span, true)); // true = template literal
            }
            ast::Expression::ArrayExpression(array) => {
                self.extract_values_from_array(array, values);
            }
            ast::Expression::ObjectExpression(obj) => {
                self.extract_values_from_object(obj, values);
            }
            _ => {
                // For other expression types, we don't extract values
            }
        }
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
            span: (start, end),
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
                        span: (c.span.start, c.span.end),
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

        // Check if current annotations contain at least one valid @prompt
        let has_valid_current_annotation = annotations
            .iter()
            .any(|a| parse_annotation(&a.exp).unwrap_or(false));

        // If no valid annotations in current statement, use stored definition annotations
        if !has_valid_current_annotation {
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
        let enclosure = (leading_start, stmt_span.end);

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
                let leading_start = leading.first().map(|first| first.span.0);
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
                let leading_start = leading.first().map(|first| first.span.0);
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
