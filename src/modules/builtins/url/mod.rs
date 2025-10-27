pub mod error;
pub mod lua_bindings;
pub mod operations;

use serde_json::{json, Value as JsonValue};

use super::BuiltinModule;
use crate::error::HypeError;

pub use error::UrlError;
pub use lua_bindings::create_url_module;
pub use operations::*;

pub struct UrlModule;

impl UrlModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for UrlModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinModule for UrlModule {
    fn name(&self) -> &str {
        "url"
    }

    fn exports(&self) -> std::result::Result<JsonValue, HypeError> {
        Ok(json!({
            "__id": "url",
            "__desc": "URL parsing and manipulation",
            "parse": {
                "__fn": "parse",
                "__desc": "Parse URL into components",
                "__signature": "parse(urlString: string) -> {protocol, host, hostname, port, path, query, hash, username, password}"
            },
            "format": {
                "__fn": "format",
                "__desc": "Build URL from components",
                "__signature": "format(components: table) -> string"
            },
            "resolve": {
                "__fn": "resolve",
                "__desc": "Resolve relative URL against base URL",
                "__signature": "resolve(base: string, relative: string) -> string"
            },
            "encode": {
                "__fn": "encode",
                "__desc": "URL encode string",
                "__signature": "encode(string: string) -> string"
            },
            "decode": {
                "__fn": "decode",
                "__desc": "URL decode string",
                "__signature": "decode(string: string) -> string"
            },
            "encodeComponent": {
                "__fn": "encodeComponent",
                "__desc": "Encode URL component",
                "__signature": "encodeComponent(string: string) -> string"
            },
            "decodeComponent": {
                "__fn": "decodeComponent",
                "__desc": "Decode URL component",
                "__signature": "decodeComponent(string: string) -> string"
            },
            "parseQuery": {
                "__fn": "parseQuery",
                "__desc": "Parse query string to table",
                "__signature": "parseQuery(queryString: string) -> table"
            },
            "formatQuery": {
                "__fn": "formatQuery",
                "__desc": "Format table as query string",
                "__signature": "formatQuery(params: table) -> string"
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_module_name() {
        let module = UrlModule::new();
        assert_eq!(module.name(), "url");
    }

    #[test]
    fn test_url_module_exports() {
        let module = UrlModule::new();
        let exports = module.exports().unwrap();
        assert!(exports.is_object());
        assert!(exports.get("parse").is_some());
        assert!(exports.get("format").is_some());
        assert!(exports.get("resolve").is_some());
        assert!(exports.get("encode").is_some());
        assert!(exports.get("decode").is_some());
        assert!(exports.get("encodeComponent").is_some());
        assert!(exports.get("decodeComponent").is_some());
        assert!(exports.get("parseQuery").is_some());
        assert!(exports.get("formatQuery").is_some());
    }

    #[test]
    fn test_url_module_default() {
        let module = UrlModule::new();
        assert_eq!(module.name(), "url");
    }
}
