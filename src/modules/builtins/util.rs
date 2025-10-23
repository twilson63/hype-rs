use serde_json::{json, Value as JsonValue};

use super::BuiltinModule;
use crate::error::HypeError;

/// Utility functions module
pub struct UtilModule;

impl UtilModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for UtilModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinModule for UtilModule {
    fn name(&self) -> &str {
        "util"
    }

    fn exports(&self) -> Result<JsonValue, HypeError> {
        Ok(json!({
            "inspect": {
                "__fn": "inspect",
                "__desc": "Convert value to string representation"
            },
            "format": {
                "__fn": "format",
                "__desc": "Format string with arguments"
            },
            "promisify": {
                "__fn": "promisify",
                "__desc": "Convert callback-based function to promise"
            },
            "inherits": {
                "__fn": "inherits",
                "__desc": "Set prototype inheritance"
            },
            "deprecate": {
                "__fn": "deprecate",
                "__desc": "Mark function as deprecated"
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_util_module_name() {
        let module = UtilModule::new();
        assert_eq!(module.name(), "util");
    }

    #[test]
    fn test_util_module_exports() {
        let module = UtilModule::new();
        let exports = module.exports().unwrap();
        assert!(exports.is_object());
        assert!(exports.get("inspect").is_some());
        assert!(exports.get("format").is_some());
        assert!(exports.get("promisify").is_some());
        assert!(exports.get("inherits").is_some());
        assert!(exports.get("deprecate").is_some());
    }

    #[test]
    fn test_util_module_default() {
        let module = UtilModule::default();
        assert_eq!(module.name(), "util");
    }

    #[test]
    fn test_util_module_exports_structure() {
        let module = UtilModule::new();
        let exports = module.exports().unwrap();

        let inspect = exports.get("inspect").unwrap();
        assert!(inspect.get("__fn").is_some());
        assert!(inspect.get("__desc").is_some());
    }

    #[test]
    fn test_util_module_all_functions() {
        let module = UtilModule::new();
        let exports = module.exports().unwrap();

        let functions = vec!["inspect", "format", "promisify", "inherits", "deprecate"];

        for func in functions {
            assert!(exports.get(func).is_some(), "Missing function: {}", func);
        }
    }

    #[test]
    fn test_util_module_init() {
        let mut module = UtilModule::new();
        assert!(module.init().is_ok());
    }
}
