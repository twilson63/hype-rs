use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;

use crate::error::HypeError;

pub mod events;
pub mod fs;
#[cfg(feature = "http")]
pub mod http;
pub mod json;
pub mod path;
pub mod process;
pub mod table;
pub mod util;

/// Trait for built-in modules.
///
/// Defines the interface for built-in modules that can be loaded via require().
/// Each built-in module implements this trait and provides its API through exports.
pub trait BuiltinModule {
    /// Get the module name (e.g., "fs", "path")
    fn name(&self) -> &str;

    /// Get the module exports as a JSON object
    fn exports(&self) -> Result<JsonValue, HypeError>;

    /// Initialize the module with any necessary setup
    fn init(&mut self) -> Result<(), HypeError> {
        Ok(())
    }
}

/// Registry for built-in modules.
///
/// Manages loading and caching of built-in modules like fs, path, events, etc.
pub struct BuiltinRegistry {
    cache: HashMap<String, JsonValue>,
}

impl BuiltinRegistry {
    /// Create a new BuiltinRegistry
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Load a built-in module by name
    ///
    /// # Arguments
    /// * `name` - The module name (e.g., "fs", "path")
    ///
    /// # Returns
    /// The module exports as a JsonValue, or an error if not found
    pub fn load(&mut self, name: &str) -> Result<JsonValue, HypeError> {
        if let Some(cached) = self.cache.get(name) {
            return Ok(cached.clone());
        }

        let exports = match name {
            "fs" => fs::FsModule::new().exports()?,
            "path" => path::PathModule::new().exports()?,
            "events" => events::EventsModule::new().exports()?,
            "util" => util::UtilModule::new().exports()?,
            "table" => table::TableModule::new().exports()?,
            "json" => json::JsonModule::new().exports()?,
            "process" => process::ProcessModule::new().exports()?,
            #[cfg(feature = "http")]
            "http" => http::HttpModule::new().exports()?,
            _ => {
                return Err(HypeError::Execution(format!(
                    "Unknown built-in module: {}",
                    name
                )))
            }
        };

        self.cache.insert(name.to_string(), exports.clone());
        Ok(exports)
    }

    /// Check if a module is a built-in
    pub fn is_builtin(&self, name: &str) -> bool {
        #[cfg(feature = "http")]
        {
            matches!(
                name,
                "fs" | "path" | "events" | "util" | "table" | "json" | "process" | "http"
            )
        }
        #[cfg(not(feature = "http"))]
        {
            matches!(
                name,
                "fs" | "path" | "events" | "util" | "table" | "json" | "process"
            )
        }
    }

    /// List all available built-in modules
    pub fn list(&self) -> Vec<&'static str> {
        #[cfg(feature = "http")]
        {
            vec![
                "fs", "path", "events", "util", "table", "json", "process", "http",
            ]
        }
        #[cfg(not(feature = "http"))]
        {
            vec!["fs", "path", "events", "util", "table", "json", "process"]
        }
    }

    /// Clear the module cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Load a built-in module for Lua with function bindings
    ///
    /// For modules that need callable Lua functions (like HTTP),
    /// this returns a Lua table with actual functions instead of JSON metadata
    ///
    /// # Arguments
    /// * `lua` - The Lua context
    /// * `name` - The module name (e.g., "http")
    ///
    /// # Returns
    /// A Lua Value (typically a Table) or an error
    #[cfg(feature = "http")]
    pub fn load_with_lua<'lua>(
        &mut self,
        lua: &'lua mlua::Lua,
        name: &str,
    ) -> Result<mlua::Value<'lua>, HypeError> {
        match name {
            "fs" => fs::create_fs_module(lua)
                .map(mlua::Value::Table)
                .map_err(|e| HypeError::Execution(format!("Failed to create fs module: {}", e))),
            "json" => json::create_json_module(lua)
                .map(mlua::Value::Table)
                .map_err(|e| HypeError::Execution(format!("Failed to create json module: {}", e))),
            "process" => process::create_process_module(lua)
                .map(mlua::Value::Table)
                .map_err(|e| {
                    HypeError::Execution(format!("Failed to create process module: {}", e))
                }),
            "http" => http::lua_bindings::create_http_module(lua)
                .map(mlua::Value::Table)
                .map_err(|e| HypeError::Execution(format!("Failed to create HTTP module: {}", e))),
            _ => {
                let json_exports = self.load(name)?;
                crate::lua::require::json_to_lua(lua, &json_exports).map_err(|e| {
                    HypeError::Execution(format!("Failed to convert module to Lua: {}", e))
                })
            }
        }
    }

    #[cfg(not(feature = "http"))]
    pub fn load_with_lua<'lua>(
        &mut self,
        lua: &'lua mlua::Lua,
        name: &str,
    ) -> Result<mlua::Value<'lua>, HypeError> {
        match name {
            "fs" => fs::create_fs_module(lua)
                .map(mlua::Value::Table)
                .map_err(|e| HypeError::Execution(format!("Failed to create fs module: {}", e))),
            "json" => json::create_json_module(lua)
                .map(mlua::Value::Table)
                .map_err(|e| HypeError::Execution(format!("Failed to create json module: {}", e))),
            "process" => process::create_process_module(lua)
                .map(mlua::Value::Table)
                .map_err(|e| {
                    HypeError::Execution(format!("Failed to create process module: {}", e))
                }),
            _ => {
                let json_exports = self.load(name)?;
                crate::lua::require::json_to_lua(lua, &json_exports).map_err(|e| {
                    HypeError::Execution(format!("Failed to convert module to Lua: {}", e))
                })
            }
        }
    }
}

impl Default for BuiltinRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_registry_new() {
        let registry = BuiltinRegistry::new();
        assert_eq!(registry.cache.len(), 0);
    }

    #[test]
    fn test_builtin_registry_is_builtin() {
        let registry = BuiltinRegistry::new();
        assert!(registry.is_builtin("fs"));
        assert!(registry.is_builtin("path"));
        assert!(registry.is_builtin("events"));
        assert!(registry.is_builtin("util"));
        assert!(registry.is_builtin("table"));
        #[cfg(feature = "http")]
        assert!(registry.is_builtin("http"));
        assert!(!registry.is_builtin("unknown"));
    }

    #[test]
    fn test_builtin_registry_list() {
        let registry = BuiltinRegistry::new();
        let list = registry.list();
        #[cfg(feature = "http")]
        assert_eq!(list.len(), 6);
        #[cfg(not(feature = "http"))]
        assert_eq!(list.len(), 5);
        assert!(list.contains(&"fs"));
        assert!(list.contains(&"path"));
        assert!(list.contains(&"events"));
        assert!(list.contains(&"util"));
        assert!(list.contains(&"table"));
        #[cfg(feature = "http")]
        assert!(list.contains(&"http"));
    }

    #[test]
    fn test_builtin_registry_load_fs() {
        let mut registry = BuiltinRegistry::new();
        let exports = registry.load("fs").unwrap();
        assert!(exports.is_object());
    }

    #[test]
    fn test_builtin_registry_load_path() {
        let mut registry = BuiltinRegistry::new();
        let exports = registry.load("path").unwrap();
        assert!(exports.is_object());
    }

    #[test]
    fn test_builtin_registry_load_events() {
        let mut registry = BuiltinRegistry::new();
        let exports = registry.load("events").unwrap();
        assert!(exports.is_object());
    }

    #[test]
    fn test_builtin_registry_load_util() {
        let mut registry = BuiltinRegistry::new();
        let exports = registry.load("util").unwrap();
        assert!(exports.is_object());
    }

    #[test]
    fn test_builtin_registry_load_table() {
        let mut registry = BuiltinRegistry::new();
        let exports = registry.load("table").unwrap();
        assert!(exports.is_object());
    }

    #[cfg(feature = "http")]
    #[test]
    fn test_builtin_registry_load_http() {
        let mut registry = BuiltinRegistry::new();
        let exports = registry.load("http").unwrap();
        assert!(exports.is_object());
        assert_eq!(exports["__id"], "http");
    }

    #[test]
    fn test_builtin_registry_caching() {
        let mut registry = BuiltinRegistry::new();
        let first = registry.load("fs").unwrap();
        let second = registry.load("fs").unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn test_builtin_registry_unknown_module() {
        let mut registry = BuiltinRegistry::new();
        let result = registry.load("unknown");
        assert!(result.is_err());
    }

    #[test]
    fn test_builtin_registry_clear() {
        let mut registry = BuiltinRegistry::new();
        registry.load("fs").ok();
        registry.load("path").ok();
        assert_eq!(registry.cache.len(), 2);

        registry.clear();
        assert_eq!(registry.cache.len(), 0);
    }

    #[test]
    fn test_builtin_registry_default() {
        let registry = BuiltinRegistry::default();
        assert_eq!(registry.cache.len(), 0);
    }
}
