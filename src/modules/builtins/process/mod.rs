pub mod error;
pub mod lua_bindings;
pub mod operations;

use serde_json::{json, Value as JsonValue};

use super::BuiltinModule;
use crate::error::HypeError;

pub use error::ProcessError;
pub use lua_bindings::create_process_module;
pub use operations::*;

pub struct ProcessModule;

impl ProcessModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ProcessModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinModule for ProcessModule {
    fn name(&self) -> &str {
        "process"
    }

    fn exports(&self) -> std::result::Result<JsonValue, HypeError> {
        Ok(json!({
            "__id": "process",
            "__desc": "Process and environment management module",
            "cwd": {
                "__fn": "cwd",
                "__desc": "Get current working directory",
                "__signature": "cwd() -> string"
            },
            "chdir": {
                "__fn": "chdir",
                "__desc": "Change current working directory",
                "__signature": "chdir(path: string) -> nil"
            },
            "env": {
                "__table": "env",
                "__desc": "Environment variables (readable/writable table)",
                "__signature": "env[key] = value"
            },
            "getenv": {
                "__fn": "getenv",
                "__desc": "Get environment variable",
                "__signature": "getenv(key: string) -> string?"
            },
            "setenv": {
                "__fn": "setenv",
                "__desc": "Set environment variable",
                "__signature": "setenv(key: string, value: string) -> nil"
            },
            "exit": {
                "__fn": "exit",
                "__desc": "Exit process with code",
                "__signature": "exit(code?: number) -> never"
            },
            "pid": {
                "__value": "number",
                "__desc": "Process ID",
            },
            "platform": {
                "__value": "string",
                "__desc": "Operating system platform",
            },
            "arch": {
                "__value": "string",
                "__desc": "CPU architecture",
            },
            "argv": {
                "__table": "argv",
                "__desc": "Command-line arguments",
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_module_name() {
        let module = ProcessModule::new();
        assert_eq!(module.name(), "process");
    }

    #[test]
    fn test_process_module_exports() {
        let module = ProcessModule::new();
        let exports = module.exports().unwrap();
        assert!(exports.is_object());
        assert!(exports.get("cwd").is_some());
        assert!(exports.get("chdir").is_some());
        assert!(exports.get("env").is_some());
        assert!(exports.get("getenv").is_some());
        assert!(exports.get("setenv").is_some());
        assert!(exports.get("exit").is_some());
        assert!(exports.get("pid").is_some());
        assert!(exports.get("platform").is_some());
        assert!(exports.get("arch").is_some());
        assert!(exports.get("argv").is_some());
    }

    #[test]
    fn test_process_module_default() {
        let module = ProcessModule::default();
        assert_eq!(module.name(), "process");
    }
}
