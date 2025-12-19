use std::collections::{HashMap, HashSet};
use volumen_types::PromptAnnotation;

/// Tracks scope information for identifiers.
pub struct ScopeTracker {
    /// Stack of prompt identifier sets (one per scope).
    prompt_idents_stack: Vec<HashSet<String>>,
    /// Annotations stored at definition time for annotated variables.
    def_annotations: HashMap<String, Vec<PromptAnnotation>>,
}

impl ScopeTracker {
    /// Create a new scope tracker with an initial global scope.
    pub fn new() -> Self {
        Self {
            prompt_idents_stack: vec![HashSet::new()],
            def_annotations: HashMap::new(),
        }
    }

    /// Enter a new scope (e.g., function, method).
    pub fn enter_scope(&mut self) {
        self.prompt_idents_stack.push(HashSet::new());
    }

    /// Exit the current scope.
    pub fn exit_scope(&mut self) {
        if self.prompt_idents_stack.len() > 1 {
            self.prompt_idents_stack.pop();
        }
    }

    /// Mark an identifier as a prompt identifier in the current scope.
    pub fn mark_prompt_ident(&mut self, ident: &str) {
        if let Some(scope) = self.prompt_idents_stack.last_mut() {
            scope.insert(ident.to_string());
        }
    }

    /// Check if an identifier is marked as a prompt identifier in any parent scope.
    pub fn is_prompt_ident(&self, ident: &str) -> bool {
        self.prompt_idents_stack
            .iter()
            .rev()
            .any(|scope| scope.contains(ident))
    }

    /// Store definition-time annotations for an identifier.
    pub fn store_def_annotation(&mut self, ident: &str, annotations: Vec<PromptAnnotation>) {
        self.def_annotations
            .insert(ident.to_string(), annotations);
    }

    /// Get stored definition annotations for an identifier.
    pub fn get_def_annotation(&self, ident: &str) -> Option<Vec<PromptAnnotation>> {
        self.def_annotations.get(ident).cloned()
    }
}
