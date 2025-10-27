pub mod error;
pub mod lua_bindings;
pub mod operations;

use serde_json::{json, Value as JsonValue};

use super::BuiltinModule;
use crate::error::HypeError;

pub use error::StringError;
pub use lua_bindings::create_string_module;
pub use operations::*;

pub struct StringModule;

impl StringModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StringModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinModule for StringModule {
    fn name(&self) -> &str {
        "string"
    }

    fn exports(&self) -> std::result::Result<JsonValue, HypeError> {
        Ok(json!({
            "__id": "string",
            "__desc": "Enhanced string manipulation utilities",
            "split": {
                "__fn": "split",
                "__desc": "Split string into array by delimiter",
                "__signature": "split(str: string, delimiter: string) -> string[]"
            },
            "trim": {
                "__fn": "trim",
                "__desc": "Remove whitespace from both ends",
                "__signature": "trim(str: string) -> string"
            },
            "trimStart": {
                "__fn": "trimStart",
                "__desc": "Remove leading whitespace",
                "__signature": "trimStart(str: string) -> string"
            },
            "trimEnd": {
                "__fn": "trimEnd",
                "__desc": "Remove trailing whitespace",
                "__signature": "trimEnd(str: string) -> string"
            },
            "startsWith": {
                "__fn": "startsWith",
                "__desc": "Check if string starts with prefix",
                "__signature": "startsWith(str: string, prefix: string) -> boolean"
            },
            "endsWith": {
                "__fn": "endsWith",
                "__desc": "Check if string ends with suffix",
                "__signature": "endsWith(str: string, suffix: string) -> boolean"
            },
            "contains": {
                "__fn": "contains",
                "__desc": "Check if string contains substring",
                "__signature": "contains(str: string, substring: string) -> boolean"
            },
            "padStart": {
                "__fn": "padStart",
                "__desc": "Pad start of string to length",
                "__signature": "padStart(str: string, length: number, fill?: string) -> string"
            },
            "padEnd": {
                "__fn": "padEnd",
                "__desc": "Pad end of string to length",
                "__signature": "padEnd(str: string, length: number, fill?: string) -> string"
            },
            "repeat": {
                "__fn": "repeat",
                "__desc": "Repeat string count times",
                "__signature": "repeat(str: string, count: number) -> string"
            },
            "replace": {
                "__fn": "replace",
                "__desc": "Replace occurrences of pattern",
                "__signature": "replace(str: string, pattern: string, replacement: string, count?: number) -> string"
            },
            "replaceAll": {
                "__fn": "replaceAll",
                "__desc": "Replace all occurrences of pattern",
                "__signature": "replaceAll(str: string, pattern: string, replacement: string) -> string"
            },
            "toUpperCase": {
                "__fn": "toUpperCase",
                "__desc": "Convert to uppercase",
                "__signature": "toUpperCase(str: string) -> string"
            },
            "toLowerCase": {
                "__fn": "toLowerCase",
                "__desc": "Convert to lowercase",
                "__signature": "toLowerCase(str: string) -> string"
            },
            "capitalize": {
                "__fn": "capitalize",
                "__desc": "Capitalize first letter",
                "__signature": "capitalize(str: string) -> string"
            },
            "lines": {
                "__fn": "lines",
                "__desc": "Split string into lines",
                "__signature": "lines(str: string) -> string[]"
            },
            "chars": {
                "__fn": "chars",
                "__desc": "Split string into characters",
                "__signature": "chars(str: string) -> string[]"
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_module_name() {
        let module = StringModule::new();
        assert_eq!(module.name(), "string");
    }

    #[test]
    fn test_string_module_exports() {
        let module = StringModule::new();
        let exports = module.exports().unwrap();
        assert!(exports.is_object());
        assert!(exports.get("split").is_some());
        assert!(exports.get("trim").is_some());
        assert!(exports.get("trimStart").is_some());
        assert!(exports.get("trimEnd").is_some());
        assert!(exports.get("startsWith").is_some());
        assert!(exports.get("endsWith").is_some());
        assert!(exports.get("contains").is_some());
        assert!(exports.get("padStart").is_some());
        assert!(exports.get("padEnd").is_some());
        assert!(exports.get("repeat").is_some());
        assert!(exports.get("replace").is_some());
        assert!(exports.get("replaceAll").is_some());
        assert!(exports.get("toUpperCase").is_some());
        assert!(exports.get("toLowerCase").is_some());
        assert!(exports.get("capitalize").is_some());
        assert!(exports.get("lines").is_some());
        assert!(exports.get("chars").is_some());
    }

    #[test]
    fn test_string_module_default() {
        let module = StringModule::new();
        assert_eq!(module.name(), "string");
    }
}
