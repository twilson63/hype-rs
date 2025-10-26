use serde_json::{json, Value as JsonValue};

use super::BuiltinModule;
use crate::error::HypeError;

pub mod client;
pub mod error;
pub mod response;

#[cfg(feature = "http")]
pub mod lua_bindings;

pub use client::HttpClient;
pub use error::HttpError;
pub use response::HttpResponse;

pub type Result<T> = std::result::Result<T, HttpError>;

pub struct HttpModule;

impl HttpModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HttpModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinModule for HttpModule {
    fn name(&self) -> &str {
        "http"
    }

    fn exports(&self) -> std::result::Result<JsonValue, HypeError> {
        Ok(json!({
            "__id": "http",
            "__desc": "HTTP client module for making web requests",
            "get": {
                "__fn": "get",
                "__desc": "HTTP GET request",
                "__signature": "get(url: string) -> Response"
            },
            "post": {
                "__fn": "post",
                "__desc": "HTTP POST request",
                "__signature": "post(url: string, options?: {body?: string, headers?: table}) -> Response"
            },
            "put": {
                "__fn": "put",
                "__desc": "HTTP PUT request",
                "__signature": "put(url: string, options?: {body?: string, headers?: table}) -> Response"
            },
            "delete": {
                "__fn": "delete",
                "__desc": "HTTP DELETE request",
                "__signature": "delete(url: string, options?: {headers?: table}) -> Response"
            },
            "patch": {
                "__fn": "patch",
                "__desc": "HTTP PATCH request",
                "__signature": "patch(url: string, options?: {body?: string, headers?: table}) -> Response"
            },
            "head": {
                "__fn": "head",
                "__desc": "HTTP HEAD request",
                "__signature": "head(url: string, options?: {headers?: table}) -> Response"
            },
            "fetch": {
                "__fn": "fetch",
                "__desc": "Universal fetch API for HTTP requests",
                "__signature": "fetch(url: string, options?: {method?: string, body?: string, headers?: table, timeout?: number}) -> Response"
            },
            "postJson": {
                "__fn": "postJson",
                "__desc": "POST request with JSON body",
                "__signature": "postJson(url: string, data: table) -> Response"
            },
            "putJson": {
                "__fn": "putJson",
                "__desc": "PUT request with JSON body",
                "__signature": "putJson(url: string, data: table) -> Response"
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_module_name() {
        let module = HttpModule::new();
        assert_eq!(module.name(), "http");
    }

    #[test]
    fn test_http_module_exports() {
        let module = HttpModule::new();
        let exports = module.exports().unwrap();
        assert!(exports.is_object());
        assert_eq!(exports["__id"], "http");
        assert!(exports.get("get").is_some());
        assert!(exports.get("post").is_some());
        assert!(exports.get("put").is_some());
        assert!(exports.get("delete").is_some());
        assert!(exports.get("fetch").is_some());
        assert!(exports.get("postJson").is_some());
    }

    #[test]
    fn test_http_module_default() {
        let module = HttpModule::default();
        assert_eq!(module.name(), "http");
    }
}
