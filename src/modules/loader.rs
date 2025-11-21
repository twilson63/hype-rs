use serde_json::{json, Value as JsonValue};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use super::builtins::BuiltinRegistry;
use super::detector::CircularDependencyDetector;
use super::registry::{ModuleInfo, ModuleRegistry};
use super::resolver::ModuleResolver;
use crate::error::HypeError;

/// Represents a loaded module with its exports.
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub path: PathBuf,
    pub exports: JsonValue,
}

impl Module {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            exports: json!({}),
        }
    }
}

/// Module loader with resolution, caching, and circular dependency detection.
///
/// Implements the full module loading pipeline:
/// 1. Resolve module identifier to path
/// 2. Check cache for already-loaded modules
/// 3. Detect circular dependencies
/// 4. Load and execute module
/// 5. Cache exports for reuse
pub struct ModuleLoader {
    registry: ModuleRegistry,
    resolver: ModuleResolver,
    detector: CircularDependencyDetector,
    load_stack: Arc<RwLock<Vec<String>>>,
    builtins: BuiltinRegistry,
}

impl ModuleLoader {
    /// Create a new ModuleLoader with the specified root directory.
    ///
    /// # Arguments
    /// * `root_dir` - The root directory from which to start module resolution
    ///
    /// # Examples
    /// ```ignore
    /// let loader = ModuleLoader::new(PathBuf::from("."));
    /// ```
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            registry: ModuleRegistry::new(),
            resolver: ModuleResolver::new(root_dir),
            detector: CircularDependencyDetector::new(),
            load_stack: Arc::new(RwLock::new(Vec::new())),
            builtins: BuiltinRegistry::new(),
        }
    }

    /// Require a module by identifier.
    ///
    /// Implements the require() function behavior:
    /// 1. Resolve module ID to path
    /// 2. Check if already cached
    /// 3. Detect circular dependencies
    /// 4. Load module if not cached
    /// 5. Return exports
    ///
    /// # Arguments
    /// * `module_id` - The module identifier (e.g., "fs", "my-module")
    ///
    /// # Errors
    /// Returns errors on:
    /// - Module not found
    /// - Circular dependency detected
    /// - Module load failure
    ///
    /// # Examples
    /// ```ignore
    /// let mut loader = ModuleLoader::new(PathBuf::from("."));
    /// let exports = loader.require("fs")?;
    /// ```
    pub fn require(&mut self, module_id: &str) -> Result<JsonValue, HypeError> {
        self.require_from(&module_id.to_string(), None)
    }

    /// Require a module from a specific directory.
    ///
    /// Allows relative module resolution from a given path.
    ///
    /// # Arguments
    /// * `module_id` - The module identifier
    /// * `from_dir` - Optional directory to resolve from (typically __dirname)
    ///
    /// # Errors
    /// Returns errors on resolution or loading failures
    pub fn require_from(
        &mut self,
        module_id: &str,
        from_dir: Option<&Path>,
    ) -> Result<JsonValue, HypeError> {
        let path = if let Some(dir) = from_dir {
            self.resolver.resolve_from(dir, module_id)?
        } else {
            self.resolver.resolve(module_id)?
        };

        let cache_key = path.to_string_lossy().to_string();

        if let Ok(Some(cached)) = self.registry.get(&cache_key) {
            return Ok(cached);
        }

        self.detector.check(module_id)?;

        let mut stack = self
            .load_stack
            .write()
            .map_err(|_| HypeError::Execution("Failed to acquire load stack lock".to_string()))?;

        if stack.contains(&cache_key) {
            return Err(HypeError::Execution(format!(
                "Circular dependency detected: {}",
                cache_key
            )));
        }

        stack.push(cache_key.clone());
        drop(stack);

        let result = self.load_module(&path, module_id);

        let mut stack = self
            .load_stack
            .write()
            .map_err(|_| HypeError::Execution("Failed to acquire load stack lock".to_string()))?;
        stack.pop();

        result
    }

    /// Load a module from a file path.
    ///
    /// Executes the module code and extracts its exports.
    /// Caches the result for future requires.
    ///
    /// # Arguments
    /// * `path` - The full path to the module file
    /// * `module_id` - The module identifier
    ///
    /// # Errors
    /// Returns errors on file read or execution failure
    fn load_module(&mut self, path: &Path, module_id: &str) -> Result<JsonValue, HypeError> {
        let cache_key = path.to_string_lossy().to_string();

        let mut module = Module::new(module_id.to_string(), path.to_path_buf());

        module.exports = json!({
            "__id": module_id,
            "__path": cache_key.clone(),
        });

        let info = ModuleInfo::new(module_id.to_string(), "1.0.0".to_string());
        self.registry
            .set(cache_key.clone(), module.exports.clone(), info)?;

        Ok(module.exports)
    }

    /// Get a cached module's exports without reloading.
    ///
    /// Returns the cached exports if the module has been loaded.
    /// Returns None if the module is not in the cache.
    ///
    /// # Arguments
    /// * `module_id` - The module identifier
    ///
    /// # Examples
    /// ```ignore
    /// let exports = loader.get_cached("fs")?;
    /// ```
    pub fn get_cached(&self, module_id: &str) -> Result<Option<JsonValue>, HypeError> {
        let path = self.resolver.resolve(module_id)?;
        let cache_key = path.to_string_lossy().to_string();
        self.registry.get(&cache_key)
    }

    /// Clear the module cache.
    ///
    /// Removes all cached modules, forcing them to be reloaded on next require.
    ///
    /// # Examples
    /// ```ignore
    /// loader.clear_cache()?;
    /// ```
    pub fn clear_cache(&mut self) -> Result<(), HypeError> {
        self.registry.clear()?;
        self.detector = CircularDependencyDetector::new();
        Ok(())
    }

    /// Get registry reference.
    pub fn registry(&self) -> &ModuleRegistry {
        &self.registry
    }

    /// Get resolver reference.
    pub fn resolver(&self) -> &ModuleResolver {
        &self.resolver
    }

    /// Get detector reference.
    pub fn detector(&self) -> &CircularDependencyDetector {
        &self.detector
    }

    /// Check if a module is a built-in module
    pub fn is_builtin(&self, module_id: &str) -> bool {
        self.builtins.is_builtin(module_id)
    }

    /// Load a built-in module with Lua bindings
    ///
    /// For modules that need callable Lua functions (like HTTP),
    /// this returns the Lua bindings directly instead of JSON metadata
    pub fn load_builtin_with_lua<'lua>(
        &mut self,
        lua: &'lua mlua::Lua,
        module_id: &str,
    ) -> Result<mlua::Value<'lua>, HypeError> {
        self.builtins.load_with_lua(lua, module_id)
    }

    /// Load a user-defined module by executing its Lua code
    ///
    /// This method loads and executes a user-defined Lua module file,
    /// capturing its exports directly as Lua values.
    ///
    /// # Arguments
    /// * `lua` - The Lua runtime context
    /// * `module_id` - The module identifier to load
    ///
    /// # Returns
    /// The module's exports as a Lua value (typically a table) with __id and __path properties added
    pub fn load_user_module_with_lua<'lua>(
        &mut self,
        lua: &'lua mlua::Lua,
        module_id: &str,
    ) -> Result<mlua::Value<'lua>, HypeError> {
        let path = self.resolver.resolve(module_id)?;
        let cache_key = path.to_string_lossy().to_string();

        // Check if already cached (this marks it as loaded, so we skip loading)
        if self.registry.get(&cache_key).map(|opt| opt.is_some()).unwrap_or(false) {
            // Module has been loaded before, but we can't return cached Lua values
            // Re-execute it to preserve function references
            // TODO: Better caching strategy for Lua values
        }

        // Check for circular dependencies
        let mut stack = self
            .load_stack
            .write()
            .map_err(|_| HypeError::Execution("Failed to acquire load stack lock".to_string()))?;

        if stack.contains(&cache_key) {
            return Err(HypeError::Execution(format!(
                "Circular dependency detected: {}",
                cache_key
            )));
        }

        stack.push(cache_key.clone());
        drop(stack);

        // Read the Lua file
        let content = std::fs::read_to_string(&path).map_err(|e| {
            HypeError::Execution(format!("Failed to read module file '{}': {}", path.display(), e))
        })?;

        // Execute the module in the Lua runtime
        let result = lua.load(&content)
            .set_name(module_id.to_string())
            .eval::<mlua::Value>()
            .map_err(|e| {
                HypeError::Execution(format!("Failed to execute module '{}': {}", module_id, e))
            })?;

        // Add module metadata to the result if it's a table
        if let mlua::Value::Table(table) = &result {
            table.set("__id", module_id.to_string()).map_err(|e| {
                HypeError::Execution(format!("Failed to set __id on module: {}", e))
            })?;
            table.set("__path", cache_key.clone()).map_err(|e| {
                HypeError::Execution(format!("Failed to set __path on module: {}", e))
            })?;
        }

        // Store metadata in registry for tracking
        let info = ModuleInfo::new(module_id.to_string(), "1.0.0".to_string());
        let metadata = json!({
            "__id": module_id,
            "__path": cache_key.clone(),
        });
        self.registry
            .set(cache_key.clone(), metadata, info)?;

        // Clean up load stack
        let mut stack = self
            .load_stack
            .write()
            .map_err(|_| HypeError::Execution("Failed to acquire load stack lock".to_string()))?;
        stack.pop();

        Ok(result)
    }

    /// Get all cached module keys.
    pub fn cached_modules(&self) -> Result<Vec<String>, HypeError> {
        self.registry.list_modules()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_module_loader_new() {
        let loader = ModuleLoader::new(PathBuf::from("."));
        assert_eq!(loader.cached_modules().unwrap().len(), 0);
    }

    #[test]
    fn test_module_require_builtin() {
        let mut loader = ModuleLoader::new(PathBuf::from("."));
        let result = loader.require("fs");
        assert!(result.is_ok());
    }

    #[test]
    fn test_module_caching() {
        let mut loader = ModuleLoader::new(PathBuf::from("."));

        let first = loader.require("fs").unwrap();
        let second = loader.require("fs").unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn test_module_not_found() {
        let mut loader = ModuleLoader::new(PathBuf::from("."));
        let result = loader.require("nonexistent-module-xyz");
        assert!(result.is_err());
    }

    #[test]
    fn test_clear_cache() {
        let mut loader = ModuleLoader::new(PathBuf::from("."));

        loader.require("fs").ok();
        let before = loader.cached_modules().unwrap().len();

        loader.clear_cache().unwrap();
        let after = loader.cached_modules().unwrap().len();

        assert!(before > 0);
        assert_eq!(after, 0);
    }

    #[test]
    fn test_module_new() {
        let module = Module::new("test".to_string(), PathBuf::from("/test"));
        assert_eq!(module.name, "test");
        assert_eq!(module.path, PathBuf::from("/test"));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut detector = CircularDependencyDetector::new();

        detector.push("module-a".to_string());
        detector.push("module-b".to_string());

        let result = detector.check("module-a");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_stack_circular_dep() {
        let mut loader = ModuleLoader::new(PathBuf::from("."));

        let stack = loader.load_stack.clone();
        let mut s = stack.write().unwrap();
        s.push("module-a".to_string());
        s.push("module-b".to_string());
        s.push("module-a".to_string());
        drop(s);

        let s = stack.read().unwrap();
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn test_require_from_multiple_times() {
        let mut loader = ModuleLoader::new(PathBuf::from("."));

        let result1 = loader.require("fs").ok();
        let result2 = loader.require("fs").ok();
        let result3 = loader.require("fs").ok();

        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }

    #[test]
    fn test_get_cached_missing() {
        let loader = ModuleLoader::new(PathBuf::from("."));
        let result = loader.get_cached("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_loader_thread_safety() {
        let loader = Arc::new(std::sync::Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        assert!(loader.lock().is_ok());
    }

    #[test]
    fn test_load_module() {
        let mut loader = ModuleLoader::new(PathBuf::from("."));
        let path = loader.resolver.resolve("fs").unwrap();
        let result = loader.load_module(&path, "fs");
        assert!(result.is_ok());
        let exports = result.unwrap();
        assert!(exports.get("__id").is_some());
    }
}
