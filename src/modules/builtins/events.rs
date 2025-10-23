use serde_json::{json, Value as JsonValue};

use super::BuiltinModule;
use crate::error::HypeError;

/// Events module providing EventEmitter class
pub struct EventsModule;

impl EventsModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for EventsModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinModule for EventsModule {
    fn name(&self) -> &str {
        "events"
    }

    fn exports(&self) -> Result<JsonValue, HypeError> {
        Ok(json!({
            "EventEmitter": {
                "__class": "EventEmitter",
                "__desc": "Event emitter class",
                "prototype": {
                    "on": {
                        "__fn": "on",
                        "__desc": "Register event listener"
                    },
                    "once": {
                        "__fn": "once",
                        "__desc": "Register one-time event listener"
                    },
                    "off": {
                        "__fn": "off",
                        "__desc": "Remove event listener"
                    },
                    "emit": {
                        "__fn": "emit",
                        "__desc": "Emit event"
                    },
                    "listeners": {
                        "__fn": "listeners",
                        "__desc": "Get listeners for event"
                    },
                    "removeAllListeners": {
                        "__fn": "removeAllListeners",
                        "__desc": "Remove all listeners"
                    }
                }
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_events_module_name() {
        let module = EventsModule::new();
        assert_eq!(module.name(), "events");
    }

    #[test]
    fn test_events_module_exports() {
        let module = EventsModule::new();
        let exports = module.exports().unwrap();
        assert!(exports.is_object());
        assert!(exports.get("EventEmitter").is_some());
    }

    #[test]
    fn test_events_module_default() {
        let module = EventsModule::default();
        assert_eq!(module.name(), "events");
    }

    #[test]
    fn test_events_module_emitter_structure() {
        let module = EventsModule::new();
        let exports = module.exports().unwrap();

        let emitter = exports.get("EventEmitter").unwrap();
        assert!(emitter.get("__class").is_some());
        assert!(emitter.get("prototype").is_some());
    }

    #[test]
    fn test_events_module_emitter_methods() {
        let module = EventsModule::new();
        let exports = module.exports().unwrap();
        let emitter = exports.get("EventEmitter").unwrap();
        let prototype = emitter.get("prototype").unwrap();

        let methods = vec![
            "on",
            "once",
            "off",
            "emit",
            "listeners",
            "removeAllListeners",
        ];

        for method in methods {
            assert!(
                prototype.get(method).is_some(),
                "Missing method: {}",
                method
            );
        }
    }

    #[test]
    fn test_events_module_init() {
        let mut module = EventsModule::new();
        assert!(module.init().is_ok());
    }
}
