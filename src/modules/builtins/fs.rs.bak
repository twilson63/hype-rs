use serde_json::{json, Value as JsonValue};
use std::fs;
use std::path::Path;

use super::BuiltinModule;
use crate::error::HypeError;

/// File system module providing file I/O operations
pub struct FsModule;

impl FsModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FsModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinModule for FsModule {
    fn name(&self) -> &str {
        "fs"
    }

    fn exports(&self) -> Result<JsonValue, HypeError> {
        Ok(json!({
            "readFileSync": {
                "__fn": "readFileSync",
                "__desc": "Read file synchronously"
            },
            "writeFileSync": {
                "__fn": "writeFileSync",
                "__desc": "Write file synchronously"
            },
            "existsSync": {
                "__fn": "existsSync",
                "__desc": "Check if file exists"
            },
            "statSync": {
                "__fn": "statSync",
                "__desc": "Get file statistics"
            },
            "readdirSync": {
                "__fn": "readdirSync",
                "__desc": "Read directory contents"
            },
            "unlinkSync": {
                "__fn": "unlinkSync",
                "__desc": "Delete file"
            },
            "mkdirSync": {
                "__fn": "mkdirSync",
                "__desc": "Create directory"
            },
            "rmdirSync": {
                "__fn": "rmdirSync",
                "__desc": "Remove directory"
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fs_module_name() {
        let module = FsModule::new();
        assert_eq!(module.name(), "fs");
    }

    #[test]
    fn test_fs_module_exports() {
        let module = FsModule::new();
        let exports = module.exports().unwrap();
        assert!(exports.is_object());
        assert!(exports.get("readFileSync").is_some());
        assert!(exports.get("writeFileSync").is_some());
        assert!(exports.get("existsSync").is_some());
        assert!(exports.get("statSync").is_some());
        assert!(exports.get("readdirSync").is_some());
        assert!(exports.get("unlinkSync").is_some());
        assert!(exports.get("mkdirSync").is_some());
        assert!(exports.get("rmdirSync").is_some());
    }

    #[test]
    fn test_fs_module_default() {
        let module = FsModule::default();
        assert_eq!(module.name(), "fs");
    }

    #[test]
    fn test_fs_module_exports_structure() {
        let module = FsModule::new();
        let exports = module.exports().unwrap();

        let read_file = exports.get("readFileSync").unwrap();
        assert!(read_file.get("__fn").is_some());
        assert!(read_file.get("__desc").is_some());
    }

    #[test]
    fn test_fs_module_all_functions() {
        let module = FsModule::new();
        let exports = module.exports().unwrap();

        let functions = vec![
            "readFileSync",
            "writeFileSync",
            "existsSync",
            "statSync",
            "readdirSync",
            "unlinkSync",
            "mkdirSync",
            "rmdirSync",
        ];

        for func in functions {
            assert!(exports.get(func).is_some(), "Missing function: {}", func);
        }
    }

    #[test]
    fn test_fs_module_init() {
        let mut module = FsModule::new();
        assert!(module.init().is_ok());
    }
}
