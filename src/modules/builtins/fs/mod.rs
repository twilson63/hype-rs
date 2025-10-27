pub mod error;
pub mod lua_bindings;
pub mod operations;

use serde_json::{json, Value as JsonValue};

use super::BuiltinModule;
use crate::error::HypeError;

pub use error::FsError;
pub use lua_bindings::create_fs_module;
pub use operations::*;

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

    fn exports(&self) -> std::result::Result<JsonValue, HypeError> {
        Ok(json!({
            "__id": "fs",
            "__desc": "Filesystem operations module",
            "readFileSync": {
                "__fn": "readFileSync",
                "__desc": "Read file synchronously",
                "__signature": "readFileSync(path: string) -> string"
            },
            "writeFileSync": {
                "__fn": "writeFileSync",
                "__desc": "Write file synchronously",
                "__signature": "writeFileSync(path: string, data: string) -> nil"
            },
            "existsSync": {
                "__fn": "existsSync",
                "__desc": "Check if file exists",
                "__signature": "existsSync(path: string) -> boolean"
            },
            "statSync": {
                "__fn": "statSync",
                "__desc": "Get file statistics",
                "__signature": "statSync(path: string) -> {size, isFile, isDirectory, isSymlink, mtime}"
            },
            "readdirSync": {
                "__fn": "readdirSync",
                "__desc": "Read directory contents",
                "__signature": "readdirSync(path: string) -> string[]"
            },
            "unlinkSync": {
                "__fn": "unlinkSync",
                "__desc": "Delete file",
                "__signature": "unlinkSync(path: string) -> nil"
            },
            "mkdirSync": {
                "__fn": "mkdirSync",
                "__desc": "Create directory (recursive)",
                "__signature": "mkdirSync(path: string) -> nil"
            },
            "rmdirSync": {
                "__fn": "rmdirSync",
                "__desc": "Remove directory",
                "__signature": "rmdirSync(path: string) -> nil"
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
}
