pub mod error;
pub mod lua_bindings;
pub mod operations;

use serde_json::{json, Value as JsonValue};

use super::BuiltinModule;
use crate::error::HypeError;

pub use error::TimeError;
pub use lua_bindings::create_time_module;
pub use operations::*;

pub struct TimeModule;

impl TimeModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TimeModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinModule for TimeModule {
    fn name(&self) -> &str {
        "time"
    }

    fn exports(&self) -> std::result::Result<JsonValue, HypeError> {
        Ok(json!({
            "__id": "time",
            "__desc": "Date and time operations",
            "now": {
                "__fn": "now",
                "__desc": "Get current timestamp in milliseconds",
                "__signature": "now() -> number"
            },
            "nowSeconds": {
                "__fn": "nowSeconds",
                "__desc": "Get current timestamp in seconds",
                "__signature": "nowSeconds() -> number"
            },
            "nowNanos": {
                "__fn": "nowNanos",
                "__desc": "Get current timestamp in nanoseconds",
                "__signature": "nowNanos() -> number"
            },
            "format": {
                "__fn": "format",
                "__desc": "Format timestamp using custom format string",
                "__signature": "format(timestamp: number, format: string) -> string"
            },
            "parse": {
                "__fn": "parse",
                "__desc": "Parse date string using custom format",
                "__signature": "parse(dateString: string, format: string) -> number"
            },
            "toISO": {
                "__fn": "toISO",
                "__desc": "Convert timestamp to ISO 8601 string",
                "__signature": "toISO(timestamp: number) -> string"
            },
            "fromISO": {
                "__fn": "fromISO",
                "__desc": "Parse ISO 8601 string to timestamp",
                "__signature": "fromISO(isoString: string) -> number"
            },
            "date": {
                "__fn": "date",
                "__desc": "Get date components as table",
                "__signature": "date(timestamp?: number) -> {year, month, day, hour, minute, second, weekday}"
            },
            "year": {
                "__fn": "year",
                "__desc": "Get year from timestamp",
                "__signature": "year(timestamp?: number) -> number"
            },
            "month": {
                "__fn": "month",
                "__desc": "Get month (1-12) from timestamp",
                "__signature": "month(timestamp?: number) -> number"
            },
            "day": {
                "__fn": "day",
                "__desc": "Get day (1-31) from timestamp",
                "__signature": "day(timestamp?: number) -> number"
            },
            "hour": {
                "__fn": "hour",
                "__desc": "Get hour (0-23) from timestamp",
                "__signature": "hour(timestamp?: number) -> number"
            },
            "minute": {
                "__fn": "minute",
                "__desc": "Get minute (0-59) from timestamp",
                "__signature": "minute(timestamp?: number) -> number"
            },
            "second": {
                "__fn": "second",
                "__desc": "Get second (0-59) from timestamp",
                "__signature": "second(timestamp?: number) -> number"
            },
            "sleep": {
                "__fn": "sleep",
                "__desc": "Sleep for specified milliseconds",
                "__signature": "sleep(ms: number) -> nil"
            },
            "elapsed": {
                "__fn": "elapsed",
                "__desc": "Calculate elapsed time since start timestamp",
                "__signature": "elapsed(start: number) -> number"
            },
            "duration": {
                "__fn": "duration",
                "__desc": "Format duration in human-readable form",
                "__signature": "duration(ms: number) -> string"
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_module_name() {
        let module = TimeModule::new();
        assert_eq!(module.name(), "time");
    }

    #[test]
    fn test_time_module_exports() {
        let module = TimeModule::new();
        let exports = module.exports().unwrap();
        assert!(exports.is_object());
        assert!(exports.get("now").is_some());
        assert!(exports.get("nowSeconds").is_some());
        assert!(exports.get("nowNanos").is_some());
        assert!(exports.get("format").is_some());
        assert!(exports.get("parse").is_some());
        assert!(exports.get("toISO").is_some());
        assert!(exports.get("fromISO").is_some());
        assert!(exports.get("date").is_some());
        assert!(exports.get("year").is_some());
        assert!(exports.get("month").is_some());
        assert!(exports.get("day").is_some());
        assert!(exports.get("hour").is_some());
        assert!(exports.get("minute").is_some());
        assert!(exports.get("second").is_some());
        assert!(exports.get("sleep").is_some());
        assert!(exports.get("elapsed").is_some());
        assert!(exports.get("duration").is_some());
    }

    #[test]
    fn test_time_module_default() {
        let module = TimeModule::new();
        assert_eq!(module.name(), "time");
    }
}
