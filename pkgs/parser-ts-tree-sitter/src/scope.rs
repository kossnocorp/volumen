use std::collections::{HashMap, HashSet};
use volumen_types::PromptAnnotation;

/// Tracks prompt identifiers and annotations across scopes.
pub struct ScopeTracker {
    /// Stack of sets containing identifiers marked as prompts in each scope.
    prompt_idents_stack: Vec<HashSet<String>>,
    /// Stack of maps storing definition-time annotations for identifiers.
    def_annotations_stack: Vec<HashMap<String, Vec<PromptAnnotation>>>,
    /// Stack of sets containing identifiers that have type annotations.
    annotated_idents_stack: Vec<HashSet<String>>,
}

impl ScopeTracker {
    pub fn new() -> Self {
        Self {
            prompt_idents_stack: vec![HashSet::new()],
            def_annotations_stack: vec![HashMap::new()],
            annotated_idents_stack: vec![HashSet::new()],
        }
    }

    /// Enter a new scope.
    pub fn enter_scope(&mut self) {
        self.prompt_idents_stack.push(HashSet::new());
        self.def_annotations_stack.push(HashMap::new());
        self.annotated_idents_stack.push(HashSet::new());
    }

    /// Exit the current scope.
    pub fn exit_scope(&mut self) {
        if self.prompt_idents_stack.len() > 1 {
            self.prompt_idents_stack.pop();
        }
        if self.def_annotations_stack.len() > 1 {
            self.def_annotations_stack.pop();
        }
        if self.annotated_idents_stack.len() > 1 {
            self.annotated_idents_stack.pop();
        }
    }

    /// Mark an identifier as a prompt variable.
    pub fn mark_prompt_ident(&mut self, ident: &str) {
        if let Some(current) = self.prompt_idents_stack.last_mut() {
            current.insert(ident.to_string());
        }
    }

    /// Check if an identifier is marked as a prompt in any scope.
    pub fn is_prompt_ident(&self, ident: &str) -> bool {
        self.prompt_idents_stack
            .iter()
            .any(|set| set.contains(ident))
    }

    /// Store definition-time annotations for an identifier.
    pub fn store_def_annotation(&mut self, ident: &str, annotations: Vec<PromptAnnotation>) {
        if let Some(current) = self.def_annotations_stack.last_mut() {
            current.insert(ident.to_string(), annotations);
        }
    }

    /// Get definition-time annotations for an identifier from any scope.
    pub fn get_def_annotation(&self, ident: &str) -> Option<Vec<PromptAnnotation>> {
        for map in self.def_annotations_stack.iter().rev() {
            if let Some(annotations) = map.get(ident) {
                return Some(annotations.clone());
            }
        }
        None
    }

    /// Mark an identifier as having a type annotation.
    pub fn mark_annotated(&mut self, ident: &str) {
        if let Some(current) = self.annotated_idents_stack.last_mut() {
            current.insert(ident.to_string());
        }
    }

    #[allow(dead_code)]
    pub fn is_annotated(&self, ident: &str) -> bool {
        self.annotated_idents_stack
            .iter()
            .any(|set| set.contains(ident))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use volumen_types::SpanShape;

    #[test]
    fn test_scope_tracking() {
        let mut tracker = ScopeTracker::new();

        tracker.mark_prompt_ident("user_prompt");
        assert!(tracker.is_prompt_ident("user_prompt"));
        assert!(!tracker.is_prompt_ident("other_var"));

        tracker.enter_scope();
        assert!(tracker.is_prompt_ident("user_prompt")); // Visible from parent
        tracker.mark_prompt_ident("local_prompt");
        assert!(tracker.is_prompt_ident("local_prompt"));

        tracker.exit_scope();
        assert!(tracker.is_prompt_ident("user_prompt"));
        assert!(!tracker.is_prompt_ident("local_prompt")); // No longer visible
    }

    #[test]
    fn test_nested_scopes() {
        let mut tracker = ScopeTracker::new();

        tracker.mark_prompt_ident("global");
        tracker.enter_scope();
        tracker.mark_prompt_ident("outer");
        tracker.enter_scope();
        tracker.mark_prompt_ident("inner");

        assert!(tracker.is_prompt_ident("global"));
        assert!(tracker.is_prompt_ident("outer"));
        assert!(tracker.is_prompt_ident("inner"));

        tracker.exit_scope();
        assert!(tracker.is_prompt_ident("global"));
        assert!(tracker.is_prompt_ident("outer"));
        assert!(!tracker.is_prompt_ident("inner"));

        tracker.exit_scope();
        assert!(tracker.is_prompt_ident("global"));
        assert!(!tracker.is_prompt_ident("outer"));
    }

    #[test]
    fn test_annotation_storage() {
        let mut tracker = ScopeTracker::new();

        let annotations = vec![PromptAnnotation {
            spans: vec![SpanShape {
                outer: (0, 10),
                inner: (2, 10),
            }],
            exp: "// @prompt".to_string(),
        }];

        tracker.store_def_annotation("hello", annotations.clone());
        assert_eq!(
            tracker.get_def_annotation("hello").unwrap()[0].exp,
            "// @prompt"
        );
        assert!(tracker.get_def_annotation("nonexistent").is_none());
    }

    #[test]
    fn test_annotated_idents() {
        let mut tracker = ScopeTracker::new();

        tracker.mark_annotated("typed_var");
        assert!(tracker.is_annotated("typed_var"));
        assert!(!tracker.is_annotated("untyped_var"));

        tracker.enter_scope();
        assert!(tracker.is_annotated("typed_var")); // Visible from parent
        tracker.exit_scope();
    }
}
