use super::error::ModuleError;
use crate::error::HypeError;

/// Detects circular dependencies during module loading.
///
/// Uses a stack-based approach to track modules currently being loaded.
/// When a module is loaded, it's pushed onto the stack. If the same module
/// is encountered again while loading, a circular dependency is detected.
///
/// # Examples
/// ```ignore
/// let mut detector = CircularDependencyDetector::new();
/// detector.push("module_a".to_string());
/// detector.check("module_b")?;  // Ok
/// detector.push("module_b".to_string());
/// detector.check("module_a")?;  // Error: circular dependency
/// ```
pub struct CircularDependencyDetector {
    loaded_stack: Vec<String>,
}

impl CircularDependencyDetector {
    /// Create a new CircularDependencyDetector with an empty stack.
    ///
    /// # Examples
    /// ```ignore
    /// let detector = CircularDependencyDetector::new();
    /// ```
    pub fn new() -> Self {
        Self {
            loaded_stack: Vec::new(),
        }
    }

    /// Check if a module would create a circular dependency.
    ///
    /// Returns an error if the module_id is already in the loaded_stack,
    /// indicating a circular dependency. The error message shows the
    /// complete chain: "a -> b -> c -> a"
    ///
    /// # Arguments
    /// * `module_id` - The module identifier to check
    ///
    /// # Errors
    /// Returns `ModuleError::CircularDependency` if the module is already
    /// in the loading stack
    ///
    /// # Examples
    /// ```ignore
    /// let mut detector = CircularDependencyDetector::new();
    /// detector.push("a".to_string());
    /// detector.push("b".to_string());
    /// detector.check("a")?;  // Error: a -> b -> a
    /// ```
    pub fn check(&self, module_id: &str) -> Result<(), HypeError> {
        if self.loaded_stack.contains(&module_id.to_string()) {
            let chain = self
                .loaded_stack
                .iter()
                .chain(std::iter::once(&module_id.to_string()))
                .cloned()
                .collect::<Vec<_>>()
                .join(" -> ");

            return Err(HypeError::Execution(
                ModuleError::InvalidManifest {
                    reason: format!("Circular dependency detected: {}", chain),
                }
                .to_string(),
            ));
        }
        Ok(())
    }

    /// Push a module onto the loading stack.
    ///
    /// This is called when a module starts loading and should be paired
    /// with a corresponding call to `pop()` when loading completes.
    ///
    /// # Arguments
    /// * `module_id` - The module identifier being loaded
    ///
    /// # Examples
    /// ```ignore
    /// let mut detector = CircularDependencyDetector::new();
    /// detector.push("module_a".to_string());
    /// assert!(detector.is_loading("module_a"));
    /// ```
    pub fn push(&mut self, module_id: String) {
        self.loaded_stack.push(module_id);
    }

    /// Pop a module from the loading stack.
    ///
    /// This is called when a module finishes loading. Should be paired
    /// with a corresponding `push()` call.
    ///
    /// # Returns
    /// The module identifier that was popped, or None if the stack is empty
    ///
    /// # Examples
    /// ```ignore
    /// let mut detector = CircularDependencyDetector::new();
    /// detector.push("module_a".to_string());
    /// let popped = detector.pop();
    /// assert_eq!(popped, Some("module_a".to_string()));
    /// ```
    pub fn pop(&mut self) -> Option<String> {
        self.loaded_stack.pop()
    }

    /// Clear the loading stack.
    ///
    /// This removes all modules from the stack. Useful for resetting
    /// the detector state after an error or for testing.
    ///
    /// # Examples
    /// ```ignore
    /// let mut detector = CircularDependencyDetector::new();
    /// detector.push("module_a".to_string());
    /// detector.clear();
    /// assert!(detector.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.loaded_stack.clear();
    }

    /// Check if a module is currently being loaded.
    ///
    /// # Arguments
    /// * `module_id` - The module identifier to check
    ///
    /// # Returns
    /// true if the module is in the loading stack, false otherwise
    ///
    /// # Examples
    /// ```ignore
    /// let mut detector = CircularDependencyDetector::new();
    /// detector.push("module_a".to_string());
    /// assert!(detector.is_loading("module_a"));
    /// assert!(!detector.is_loading("module_b"));
    /// ```
    pub fn is_loading(&self, module_id: &str) -> bool {
        self.loaded_stack.iter().any(|m| m == module_id)
    }

    /// Get a copy of the current loading stack.
    ///
    /// # Returns
    /// A vector containing the module identifiers currently being loaded,
    /// in the order they were pushed
    ///
    /// # Examples
    /// ```ignore
    /// let mut detector = CircularDependencyDetector::new();
    /// detector.push("a".to_string());
    /// detector.push("b".to_string());
    /// let stack = detector.get_stack();
    /// assert_eq!(stack, vec!["a", "b"]);
    /// ```
    pub fn get_stack(&self) -> Vec<String> {
        self.loaded_stack.clone()
    }

    /// Check if the loading stack is empty.
    ///
    /// # Returns
    /// true if no modules are currently being loaded, false otherwise
    ///
    /// # Examples
    /// ```ignore
    /// let mut detector = CircularDependencyDetector::new();
    /// assert!(detector.is_empty());
    /// detector.push("module_a".to_string());
    /// assert!(!detector.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.loaded_stack.is_empty()
    }

    /// Get the current stack depth.
    ///
    /// # Returns
    /// The number of modules currently being loaded
    ///
    /// # Examples
    /// ```ignore
    /// let mut detector = CircularDependencyDetector::new();
    /// detector.push("a".to_string());
    /// detector.push("b".to_string());
    /// assert_eq!(detector.depth(), 2);
    /// ```
    pub fn depth(&self) -> usize {
        self.loaded_stack.len()
    }
}

impl Default for CircularDependencyDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_detector() {
        let detector = CircularDependencyDetector::new();
        assert!(detector.is_empty());
        assert_eq!(detector.depth(), 0);
    }

    #[test]
    fn test_no_circular_deps() {
        let mut detector = CircularDependencyDetector::new();
        detector.push("module_a".to_string());
        let result = detector.check("module_b");
        assert!(result.is_ok());
    }

    #[test]
    fn test_simple_circular_dep() {
        let mut detector = CircularDependencyDetector::new();
        detector.push("module_a".to_string());
        let result = detector.check("module_a");
        assert!(result.is_err());
    }

    #[test]
    fn test_complex_circular_chain() {
        let mut detector = CircularDependencyDetector::new();
        detector.push("a".to_string());
        detector.push("b".to_string());
        detector.push("c".to_string());
        let result = detector.check("a");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("a -> b -> c -> a"));
    }

    #[test]
    fn test_push_and_pop() {
        let mut detector = CircularDependencyDetector::new();
        detector.push("module_a".to_string());
        assert_eq!(detector.depth(), 1);
        assert!(detector.is_loading("module_a"));

        let popped = detector.pop();
        assert_eq!(popped, Some("module_a".to_string()));
        assert_eq!(detector.depth(), 0);
        assert!(!detector.is_loading("module_a"));
    }

    #[test]
    fn test_stack_management() {
        let mut detector = CircularDependencyDetector::new();

        detector.push("a".to_string());
        detector.push("b".to_string());
        detector.push("c".to_string());

        assert_eq!(detector.depth(), 3);

        let stack = detector.get_stack();
        assert_eq!(stack, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_clear_stack() {
        let mut detector = CircularDependencyDetector::new();
        detector.push("a".to_string());
        detector.push("b".to_string());

        assert_eq!(detector.depth(), 2);
        detector.clear();
        assert_eq!(detector.depth(), 0);
        assert!(detector.is_empty());
    }

    #[test]
    fn test_is_loading() {
        let mut detector = CircularDependencyDetector::new();
        detector.push("module_a".to_string());
        detector.push("module_b".to_string());

        assert!(detector.is_loading("module_a"));
        assert!(detector.is_loading("module_b"));
        assert!(!detector.is_loading("module_c"));
    }

    #[test]
    fn test_get_stack() {
        let mut detector = CircularDependencyDetector::new();
        detector.push("alpha".to_string());
        detector.push("beta".to_string());
        detector.push("gamma".to_string());

        let stack = detector.get_stack();
        assert_eq!(stack.len(), 3);
        assert_eq!(stack[0], "alpha");
        assert_eq!(stack[1], "beta");
        assert_eq!(stack[2], "gamma");
    }

    #[test]
    fn test_circular_dep_error_message() {
        let mut detector = CircularDependencyDetector::new();
        detector.push("module_x".to_string());
        detector.push("module_y".to_string());

        let result = detector.check("module_x");
        assert!(result.is_err());

        let err = result.unwrap_err().to_string();
        assert!(err.contains("module_x"));
        assert!(err.contains("module_y"));
    }

    #[test]
    fn test_multiple_levels_circular() {
        let mut detector = CircularDependencyDetector::new();

        detector.push("a".to_string());
        assert!(detector.check("a").is_err());

        detector.pop();
        detector.push("b".to_string());
        assert!(detector.check("b").is_err());
    }

    #[test]
    fn test_pop_from_empty_stack() {
        let mut detector = CircularDependencyDetector::new();
        let result = detector.pop();
        assert_eq!(result, None);
    }

    #[test]
    fn test_check_before_push() {
        let mut detector = CircularDependencyDetector::new();
        let result = detector.check("first_module");
        assert!(result.is_ok());
        detector.push("first_module".to_string());
        let result = detector.check("first_module");
        assert!(result.is_err());
    }

    #[test]
    fn test_detector_default() {
        let detector = CircularDependencyDetector::default();
        assert!(detector.is_empty());
    }

    #[test]
    fn test_nested_stack_operations() {
        let mut detector = CircularDependencyDetector::new();

        detector.push("module1".to_string());
        detector.push("module2".to_string());
        detector.push("module3".to_string());

        assert_eq!(detector.depth(), 3);

        detector.pop();
        assert_eq!(detector.depth(), 2);
        assert!(!detector.is_loading("module3"));

        detector.pop();
        assert_eq!(detector.depth(), 1);
        assert!(detector.is_loading("module1"));
    }

    #[test]
    fn test_circular_at_different_depths() {
        let mut detector = CircularDependencyDetector::new();

        detector.push("root".to_string());
        assert!(detector.check("child1").is_ok());

        detector.push("child1".to_string());
        assert!(detector.check("root").is_err());

        detector.pop();
        assert!(detector.check("root").is_err());
    }
}
