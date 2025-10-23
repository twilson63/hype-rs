use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::error::ModuleError;
use crate::error::HypeError;

/// Information about a cached module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// Module name
    pub name: String,
    /// Module version
    pub version: String,
    /// Module description
    pub description: Option<String>,
    /// Whether the module is loaded
    pub loaded: bool,
}

impl ModuleInfo {
    /// Create a new ModuleInfo struct.
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            description: None,
            loaded: false,
        }
    }

    /// Set module description.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Mark module as loaded.
    pub fn mark_loaded(mut self) -> Self {
        self.loaded = true;
        self
    }
}

/// Thread-safe module registry with caching support.
///
/// Uses Arc<RwLock<>> for thread-safe access to cached module data.
/// Stores module data as JSON values to avoid lifetime constraints.
/// Allows concurrent reads while ensuring exclusive write access.
pub struct ModuleRegistry {
    cache: Arc<RwLock<HashMap<String, JsonValue>>>,
    metadata: Arc<RwLock<HashMap<String, ModuleInfo>>>,
}

impl ModuleRegistry {
    /// Create a new empty module registry.
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get a module from the registry.
    ///
    /// Returns the cached JSON value if the module exists.
    /// Returns None if the module is not in the registry.
    pub fn get(&self, key: &str) -> Result<Option<JsonValue>, HypeError> {
        let cache = self
            .cache
            .read()
            .map_err(|_| HypeError::Execution(ModuleError::LockPoisoned.to_string()))?;

        Ok(cache.get(key).cloned())
    }

    /// Set a module in the registry.
    ///
    /// Stores the module data as a JSON value with the associated module name.
    /// If a module with the same name exists, it will be overwritten.
    pub fn set(&self, key: String, value: JsonValue, info: ModuleInfo) -> Result<(), HypeError> {
        let mut cache = self
            .cache
            .write()
            .map_err(|_| HypeError::Execution(ModuleError::LockPoisoned.to_string()))?;

        cache.insert(key.clone(), value);

        let mut metadata = self
            .metadata
            .write()
            .map_err(|_| HypeError::Execution(ModuleError::LockPoisoned.to_string()))?;

        metadata.insert(key, info);

        Ok(())
    }

    /// Check if a module exists in the registry.
    pub fn contains(&self, key: &str) -> Result<bool, HypeError> {
        let cache = self
            .cache
            .read()
            .map_err(|_| HypeError::Execution(ModuleError::LockPoisoned.to_string()))?;

        Ok(cache.contains_key(key))
    }

    /// Get module metadata information.
    pub fn get_info(&self, key: &str) -> Result<Option<ModuleInfo>, HypeError> {
        let metadata = self
            .metadata
            .read()
            .map_err(|_| HypeError::Execution(ModuleError::LockPoisoned.to_string()))?;

        Ok(metadata.get(key).cloned())
    }

    /// Clear all modules from the registry.
    pub fn clear(&self) -> Result<(), HypeError> {
        let mut cache = self
            .cache
            .write()
            .map_err(|_| HypeError::Execution(ModuleError::LockPoisoned.to_string()))?;

        cache.clear();

        let mut metadata = self
            .metadata
            .write()
            .map_err(|_| HypeError::Execution(ModuleError::LockPoisoned.to_string()))?;

        metadata.clear();

        Ok(())
    }

    /// Get the number of modules in the registry.
    pub fn len(&self) -> Result<usize, HypeError> {
        let cache = self
            .cache
            .read()
            .map_err(|_| HypeError::Execution(ModuleError::LockPoisoned.to_string()))?;

        Ok(cache.len())
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> Result<bool, HypeError> {
        let cache = self
            .cache
            .read()
            .map_err(|_| HypeError::Execution(ModuleError::LockPoisoned.to_string()))?;

        Ok(cache.is_empty())
    }

    /// List all module names in the registry.
    pub fn list_modules(&self) -> Result<Vec<String>, HypeError> {
        let cache = self
            .cache
            .read()
            .map_err(|_| HypeError::Execution(ModuleError::LockPoisoned.to_string()))?;

        Ok(cache.keys().cloned().collect())
    }

    /// Remove a module from the registry.
    pub fn remove(&self, key: &str) -> Result<Option<JsonValue>, HypeError> {
        let mut cache = self
            .cache
            .write()
            .map_err(|_| HypeError::Execution(ModuleError::LockPoisoned.to_string()))?;

        let removed = cache.remove(key);

        if removed.is_some() {
            let mut metadata = self
                .metadata
                .write()
                .map_err(|_| HypeError::Execution(ModuleError::LockPoisoned.to_string()))?;

            metadata.remove(key);
        }

        Ok(removed)
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ModuleRegistry {
    fn clone(&self) -> Self {
        Self {
            cache: Arc::clone(&self.cache),
            metadata: Arc::clone(&self.metadata),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_new() {
        let registry = ModuleRegistry::new();
        assert!(registry.is_empty().unwrap());
    }

    #[test]
    fn test_registry_set_and_get() {
        let registry = ModuleRegistry::new();
        let info = ModuleInfo::new("test".to_string(), "1.0.0".to_string());

        let json_val = JsonValue::Bool(true);
        registry
            .set("test".to_string(), json_val.clone(), info)
            .unwrap();

        let retrieved = registry.get("test").unwrap();
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_registry_contains() {
        let registry = ModuleRegistry::new();
        let info = ModuleInfo::new("test".to_string(), "1.0.0".to_string());

        let json_val = JsonValue::Bool(true);
        registry.set("test".to_string(), json_val, info).unwrap();

        assert!(registry.contains("test").unwrap());
        assert!(!registry.contains("nonexistent").unwrap());
    }

    #[test]
    fn test_registry_clear() {
        let registry = ModuleRegistry::new();
        let info = ModuleInfo::new("test".to_string(), "1.0.0".to_string());

        let json_val = JsonValue::Bool(true);
        registry.set("test".to_string(), json_val, info).unwrap();

        assert_eq!(registry.len().unwrap(), 1);

        registry.clear().unwrap();
        assert_eq!(registry.len().unwrap(), 0);
    }

    #[test]
    fn test_registry_len() {
        let registry = ModuleRegistry::new();
        assert_eq!(registry.len().unwrap(), 0);

        let info = ModuleInfo::new("test1".to_string(), "1.0.0".to_string());
        registry
            .set("test1".to_string(), JsonValue::Bool(true), info.clone())
            .unwrap();

        assert_eq!(registry.len().unwrap(), 1);

        let info2 = ModuleInfo::new("test2".to_string(), "1.0.0".to_string());
        registry
            .set("test2".to_string(), JsonValue::Bool(false), info2)
            .unwrap();

        assert_eq!(registry.len().unwrap(), 2);
    }

    #[test]
    fn test_registry_list_modules() {
        let registry = ModuleRegistry::new();

        let info1 = ModuleInfo::new("module1".to_string(), "1.0.0".to_string());
        registry
            .set("module1".to_string(), JsonValue::Bool(true), info1)
            .unwrap();

        let info2 = ModuleInfo::new("module2".to_string(), "2.0.0".to_string());
        registry
            .set("module2".to_string(), JsonValue::Bool(false), info2)
            .unwrap();

        let modules = registry.list_modules().unwrap();
        assert_eq!(modules.len(), 2);
        assert!(modules.contains(&"module1".to_string()));
        assert!(modules.contains(&"module2".to_string()));
    }

    #[test]
    fn test_registry_remove() {
        let registry = ModuleRegistry::new();
        let info = ModuleInfo::new("test".to_string(), "1.0.0".to_string());

        let json_val = JsonValue::Bool(true);
        registry.set("test".to_string(), json_val, info).unwrap();

        assert!(registry.contains("test").unwrap());

        let removed = registry.remove("test").unwrap();
        assert!(removed.is_some());
        assert!(!registry.contains("test").unwrap());
    }

    #[test]
    fn test_registry_get_info() {
        let registry = ModuleRegistry::new();
        let info = ModuleInfo::new("test".to_string(), "1.0.0".to_string())
            .with_description("Test module".to_string());

        registry
            .set("test".to_string(), JsonValue::Bool(true), info)
            .unwrap();

        let retrieved_info = registry.get_info("test").unwrap();
        assert!(retrieved_info.is_some());

        let info = retrieved_info.unwrap();
        assert_eq!(info.name, "test");
        assert_eq!(info.version, "1.0.0");
        assert_eq!(info.description, Some("Test module".to_string()));
    }

    #[test]
    fn test_registry_clone() {
        let registry = ModuleRegistry::new();
        let info = ModuleInfo::new("test".to_string(), "1.0.0".to_string());

        registry
            .set("test".to_string(), JsonValue::Bool(true), info)
            .unwrap();

        let cloned = registry.clone();
        assert!(cloned.contains("test").unwrap());
        assert_eq!(cloned.len().unwrap(), 1);
    }

    #[test]
    fn test_module_info_builder() {
        let info = ModuleInfo::new("test".to_string(), "1.0.0".to_string())
            .with_description("A test module".to_string())
            .mark_loaded();

        assert_eq!(info.name, "test");
        assert_eq!(info.version, "1.0.0");
        assert_eq!(info.description, Some("A test module".to_string()));
        assert!(info.loaded);
    }
}
