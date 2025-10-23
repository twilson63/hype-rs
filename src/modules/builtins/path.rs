use serde_json::{json, Value as JsonValue};

use super::BuiltinModule;
use crate::error::HypeError;

/// Path utilities module for path manipulation
pub struct PathModule;

impl PathModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PathModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinModule for PathModule {
    fn name(&self) -> &str {
        "path"
    }

    fn exports(&self) -> Result<JsonValue, HypeError> {
        Ok(json!({
            "join": {
                "__fn": "join",
                "__desc": "Join path segments"
            },
            "dirname": {
                "__fn": "dirname",
                "__desc": "Get directory name"
            },
            "basename": {
                "__fn": "basename",
                "__desc": "Get base name"
            },
            "extname": {
                "__fn": "extname",
                "__desc": "Get file extension"
            },
            "resolve": {
                "__fn": "resolve",
                "__desc": "Resolve absolute path"
            },
            "relative": {
                "__fn": "relative",
                "__desc": "Get relative path"
            },
            "normalize": {
                "__fn": "normalize",
                "__desc": "Normalize path"
            },
            "sep": {
                "__value": std::path::MAIN_SEPARATOR,
                "__desc": "Path separator"
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_module_name() {
        let module = PathModule::new();
        assert_eq!(module.name(), "path");
    }

    #[test]
    fn test_path_module_exports() {
        let module = PathModule::new();
        let exports = module.exports().unwrap();
        assert!(exports.is_object());
        assert!(exports.get("join").is_some());
        assert!(exports.get("dirname").is_some());
        assert!(exports.get("basename").is_some());
        assert!(exports.get("extname").is_some());
        assert!(exports.get("resolve").is_some());
        assert!(exports.get("relative").is_some());
        assert!(exports.get("normalize").is_some());
        assert!(exports.get("sep").is_some());
    }

    #[test]
    fn test_path_module_default() {
        let module = PathModule::default();
        assert_eq!(module.name(), "path");
    }

    #[test]
    fn test_path_module_exports_structure() {
        let module = PathModule::new();
        let exports = module.exports().unwrap();

        let join = exports.get("join").unwrap();
        assert!(join.get("__fn").is_some());
        assert!(join.get("__desc").is_some());
    }

    #[test]
    fn test_path_module_all_functions() {
        let module = PathModule::new();
        let exports = module.exports().unwrap();

        let functions = vec![
            "join",
            "dirname",
            "basename",
            "extname",
            "resolve",
            "relative",
            "normalize",
            "sep",
        ];

        for func in functions {
            assert!(exports.get(func).is_some(), "Missing function: {}", func);
        }
    }

    #[test]
    fn test_path_module_init() {
        let mut module = PathModule::new();
        assert!(module.init().is_ok());
    }

    #[test]
    fn test_path_module_sep_value() {
        let module = PathModule::new();
        let exports = module.exports().unwrap();
        let sep = exports.get("sep").unwrap();
        assert!(sep.get("__value").is_some());
    }
}
