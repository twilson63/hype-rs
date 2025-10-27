pub mod error;
pub mod lua_bindings;
pub mod operations;

use serde_json::{json, Value as JsonValue};

use super::BuiltinModule;
use crate::error::HypeError;

pub use error::QueryStringError;
pub use lua_bindings::create_querystring_module;
pub use operations::*;

pub struct QueryStringModule;

impl QueryStringModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for QueryStringModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinModule for QueryStringModule {
    fn name(&self) -> &str {
        "querystring"
    }

    fn exports(&self) -> std::result::Result<JsonValue, HypeError> {
        Ok(json!({
            "__id": "querystring",
            "__desc": "Query string parsing and formatting",
            "parse": {
                "__fn": "parse",
                "__desc": "Parse query string into key-value pairs",
                "__signature": "parse(queryString: string) -> table"
            },
            "stringify": {
                "__fn": "stringify",
                "__desc": "Convert table to query string",
                "__signature": "stringify(params: table) -> string"
            },
            "escape": {
                "__fn": "escape",
                "__desc": "URL-encode string for query strings",
                "__signature": "escape(string: string) -> string"
            },
            "unescape": {
                "__fn": "unescape",
                "__desc": "Decode URL-encoded query string component",
                "__signature": "unescape(string: string) -> string"
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_querystring_module_name() {
        let module = QueryStringModule::new();
        assert_eq!(module.name(), "querystring");
    }

    #[test]
    fn test_querystring_module_exports() {
        let module = QueryStringModule::new();
        let exports = module.exports().unwrap();
        assert!(exports.is_object());
        assert!(exports.get("parse").is_some());
        assert!(exports.get("stringify").is_some());
        assert!(exports.get("escape").is_some());
        assert!(exports.get("unescape").is_some());
    }

    #[test]
    fn test_querystring_module_default() {
        let module = QueryStringModule::new();
        assert_eq!(module.name(), "querystring");
    }
}
