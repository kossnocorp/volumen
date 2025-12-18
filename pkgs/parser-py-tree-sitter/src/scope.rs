use std::collections::{HashMap, HashSet};
use volumen_types::PromptAnnotation;

/// Tracks scope-aware state for prompt identifier detection.
/// Manages stacks for nested scopes (functions, classes) and annotation tracking.
pub struct ScopeTracker {
    /// Stack of prompt identifier sets per scope.
    /// Each scope has a set of identifiers that were marked as prompts.
    prompt_idents_stack: Vec<HashSet<String>>,

    /// Stack of definition-time annotations per scope.
    /// Maps identifier names to their original @prompt annotations.
    def_annotations_stack: Vec<HashMap<String, Vec<PromptAnnotation>>>,

    /// Set of identifiers that have type annotations (e.g., `name: str = value`).
    /// Used to track type-annotated variables across reassignments.
    annotated_idents: HashSet<String>,
}

impl ScopeTracker {
    /// Create a new ScopeTracker with an initial global scope.
    pub fn new() -> Self {
        Self {
            prompt_idents_stack: vec![HashSet::new()],
            def_annotations_stack: vec![HashMap::new()],
            annotated_idents: HashSet::new(),
        }
    }

    /// Enter a new scope (e.g., when entering a function or class definition).
    pub fn enter_scope(&mut self) {
        self.prompt_idents_stack.push(HashSet::new());
        self.def_annotations_stack.push(HashMap::new());
    }

    /// Exit the current scope (e.g., when leaving a function or class definition).
    pub fn exit_scope(&mut self) {
        if self.prompt_idents_stack.len() > 1 {
            self.prompt_idents_stack.pop();
        }
        if self.def_annotations_stack.len() > 1 {
            self.def_annotations_stack.pop();
        }
    }

    /// Mark an identifier as a prompt variable in the current scope.
    /// This identifier will be tracked for reassignments.
    pub fn mark_prompt_ident(&mut self, ident: &str) {
        if let Some(scope) = self.prompt_idents_stack.last_mut() {
            scope.insert(ident.to_string());
        }
    }

    /// Check if an identifier is marked as a prompt in any parent scope.
    /// Searches from innermost to outermost scope.
    pub fn is_prompt_ident(&self, ident: &str) -> bool {
        self.prompt_idents_stack
            .iter()
            .any(|scope| scope.contains(ident))
    }

    /// Store definition-time annotations for an identifier.
    /// These annotations are preserved across reassignments.
    pub fn store_def_annotation(&mut self, ident: &str, annotations: Vec<PromptAnnotation>) {
        if let Some(scope) = self.def_annotations_stack.last_mut() {
            scope.insert(ident.to_string(), annotations);
        }
        self.annotated_idents.insert(ident.to_string());
    }

    /// Retrieve definition-time annotations for an identifier.
    /// Searches from innermost to outermost scope.
    pub fn get_def_annotation(&self, ident: &str) -> Option<Vec<PromptAnnotation>> {
        for scope in self.def_annotations_stack.iter().rev() {
            if let Some(ann) = scope.get(ident) {
                return Some(ann.clone());
            }
        }
        None
    }

    /// Check if an identifier has a type annotation.
    pub fn is_annotated(&self, ident: &str) -> bool {
        self.annotated_idents.contains(ident)
    }

    /// Mark an identifier as having a type annotation.
    pub fn mark_annotated(&mut self, ident: &str) {
        self.annotated_idents.insert(ident.to_string());
    }
}

impl Default for ScopeTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use volumen_types::Span;

    #[test]
    fn test_scope_tracking() {
        let mut tracker = ScopeTracker::new();

        // Mark identifier in global scope
        tracker.mark_prompt_ident("global_prompt");
        assert!(tracker.is_prompt_ident("global_prompt"));

        // Enter new scope
        tracker.enter_scope();
        tracker.mark_prompt_ident("local_prompt");

        // Should find both global and local
        assert!(tracker.is_prompt_ident("global_prompt"));
        assert!(tracker.is_prompt_ident("local_prompt"));

        // Exit scope
        tracker.exit_scope();

        // Should still find global, but not local
        assert!(tracker.is_prompt_ident("global_prompt"));
        assert!(!tracker.is_prompt_ident("local_prompt"));
    }

    #[test]
    fn test_annotation_storage() {
        let mut tracker = ScopeTracker::new();

        let annotation = PromptAnnotation {
            span: Span { start: 0, end: 10 },
            exp: "# @prompt".to_string(),
        };

        tracker.store_def_annotation("test_var", vec![annotation.clone()]);

        let retrieved = tracker.get_def_annotation("test_var");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().len(), 1);
    }

    #[test]
    fn test_annotated_idents() {
        let mut tracker = ScopeTracker::new();

        tracker.mark_annotated("typed_var");
        assert!(tracker.is_annotated("typed_var"));
        assert!(!tracker.is_annotated("untyped_var"));
    }

    #[test]
    fn test_nested_scopes() {
        let mut tracker = ScopeTracker::new();

        tracker.mark_prompt_ident("outer");

        tracker.enter_scope();
        tracker.mark_prompt_ident("middle");

        tracker.enter_scope();
        tracker.mark_prompt_ident("inner");

        // All should be visible
        assert!(tracker.is_prompt_ident("outer"));
        assert!(tracker.is_prompt_ident("middle"));
        assert!(tracker.is_prompt_ident("inner"));

        tracker.exit_scope();
        assert!(tracker.is_prompt_ident("outer"));
        assert!(tracker.is_prompt_ident("middle"));
        assert!(!tracker.is_prompt_ident("inner"));

        tracker.exit_scope();
        assert!(tracker.is_prompt_ident("outer"));
        assert!(!tracker.is_prompt_ident("middle"));
        assert!(!tracker.is_prompt_ident("inner"));
    }
}
