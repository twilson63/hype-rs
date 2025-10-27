pub mod error;
pub mod lua_bindings;
pub mod operations;

use serde_json::{json, Value as JsonValue};

use super::BuiltinModule;
use crate::error::HypeError;

pub use error::OsError;
pub use lua_bindings::create_os_module;
pub use operations::*;

pub struct OsModule;

impl OsModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OsModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinModule for OsModule {
    fn name(&self) -> &str {
        "os"
    }

    fn exports(&self) -> std::result::Result<JsonValue, HypeError> {
        Ok(json!({
            "__id": "os",
            "__desc": "Operating system information and utilities",
            "platform": {
                "__fn": "platform",
                "__desc": "Get operating system platform",
                "__signature": "platform() -> string"
            },
            "arch": {
                "__fn": "arch",
                "__desc": "Get CPU architecture",
                "__signature": "arch() -> string"
            },
            "hostname": {
                "__fn": "hostname",
                "__desc": "Get system hostname",
                "__signature": "hostname() -> string"
            },
            "homedir": {
                "__fn": "homedir",
                "__desc": "Get user home directory",
                "__signature": "homedir() -> string"
            },
            "tmpdir": {
                "__fn": "tmpdir",
                "__desc": "Get system temp directory",
                "__signature": "tmpdir() -> string"
            },
            "cpus": {
                "__fn": "cpus",
                "__desc": "Get CPU information",
                "__signature": "cpus() -> [{model: string, speed: number}]"
            },
            "totalmem": {
                "__fn": "totalmem",
                "__desc": "Get total system memory in bytes",
                "__signature": "totalmem() -> number"
            },
            "freemem": {
                "__fn": "freemem",
                "__desc": "Get free system memory in bytes",
                "__signature": "freemem() -> number"
            },
            "uptime": {
                "__fn": "uptime",
                "__desc": "Get system uptime in seconds",
                "__signature": "uptime() -> number"
            },
            "loadavg": {
                "__fn": "loadavg",
                "__desc": "Get load average [1, 5, 15 min]",
                "__signature": "loadavg() -> [number, number, number]"
            },
            "networkInterfaces": {
                "__fn": "networkInterfaces",
                "__desc": "Get network interfaces",
                "__signature": "networkInterfaces() -> [{name: string, mac: string}]"
            },
            "userInfo": {
                "__fn": "userInfo",
                "__desc": "Get current user information",
                "__signature": "userInfo() -> {username: string, uid?: number, gid?: number, shell?: string, homedir: string}"
            },
            "EOL": {
                "__value": "string",
                "__desc": "End of line marker for the platform"
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_module_name() {
        let module = OsModule::new();
        assert_eq!(module.name(), "os");
    }

    #[test]
    fn test_os_module_exports() {
        let module = OsModule::new();
        let exports = module.exports().unwrap();
        assert!(exports.is_object());
        assert!(exports.get("platform").is_some());
        assert!(exports.get("arch").is_some());
        assert!(exports.get("hostname").is_some());
        assert!(exports.get("homedir").is_some());
        assert!(exports.get("tmpdir").is_some());
        assert!(exports.get("cpus").is_some());
        assert!(exports.get("totalmem").is_some());
        assert!(exports.get("freemem").is_some());
        assert!(exports.get("uptime").is_some());
        assert!(exports.get("loadavg").is_some());
        assert!(exports.get("networkInterfaces").is_some());
        assert!(exports.get("userInfo").is_some());
        assert!(exports.get("EOL").is_some());
    }

    #[test]
    fn test_os_module_default() {
        let module = OsModule::new();
        assert_eq!(module.name(), "os");
    }
}
