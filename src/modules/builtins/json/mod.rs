pub mod error;
pub mod lua_bindings;
pub mod operations;

use serde_json::{json, Value as JsonValue};

use super::BuiltinModule;
use crate::error::HypeError;

pub use error::JsonError;
pub use lua_bindings::create_json_module;
pub use operations::*;

pub struct JsonModule;

impl JsonModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinModule for JsonModule {
    fn name(&self) -> &str {
        "json"
    }

    fn exports(&self) -> std::result::Result<JsonValue, HypeError> {
        Ok(json!({
            "__id": "json",
            "__desc": "JSON encoding and decoding module",
            "encode": {
                "__fn": "encode",
                "__desc": "Encode Lua value to JSON string",
                "__signature": "encode(value, pretty?: boolean) -> string"
            },
            "decode": {
                "__fn": "decode",
                "__desc": "Decode JSON string to Lua value",
                "__signature": "decode(jsonString: string) -> any"
            },
            "stringify": {
                "__fn": "stringify",
                "__desc": "Alias for encode",
                "__signature": "stringify(value, pretty?: boolean) -> string"
            },
            "parse": {
                "__fn": "parse",
                "__desc": "Alias for decode",
                "__signature": "parse(jsonString: string) -> any"
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_module_name() {
        let module = JsonModule::new();
        assert_eq!(module.name(), "json");
    }

    #[test]
    fn test_json_module_exports() {
        let module = JsonModule::new();
        let exports = module.exports().unwrap();
        assert!(exports.is_object());
        assert!(exports.get("encode").is_some());
        assert!(exports.get("decode").is_some());
        assert!(exports.get("stringify").is_some());
        assert!(exports.get("parse").is_some());
    }

    #[test]
    fn test_json_module_default() {
        let module = JsonModule::default();
        assert_eq!(module.name(), "json");
    }
}
