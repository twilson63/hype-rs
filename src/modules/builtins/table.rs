use serde_json::{json, Value as JsonValue};

use super::BuiltinModule;
use crate::error::HypeError;

/// Table manipulation module for Lua tables
pub struct TableModule;

impl TableModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TableModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinModule for TableModule {
    fn name(&self) -> &str {
        "table"
    }

    fn exports(&self) -> Result<JsonValue, HypeError> {
        Ok(json!({
            "merge": {
                "__fn": "merge",
                "__desc": "Merge two tables"
            },
            "clone": {
                "__fn": "clone",
                "__desc": "Deep clone a table"
            },
            "keys": {
                "__fn": "keys",
                "__desc": "Get all keys from table"
            },
            "values": {
                "__fn": "values",
                "__desc": "Get all values from table"
            },
            "filter": {
                "__fn": "filter",
                "__desc": "Filter table by predicate"
            },
            "map": {
                "__fn": "map",
                "__desc": "Map table values"
            },
            "reduce": {
                "__fn": "reduce",
                "__desc": "Reduce table to single value"
            },
            "insert": {
                "__fn": "insert",
                "__desc": "Insert element into table"
            },
            "remove": {
                "__fn": "remove",
                "__desc": "Remove element from table"
            },
            "contains": {
                "__fn": "contains",
                "__desc": "Check if table contains value"
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_module_name() {
        let module = TableModule::new();
        assert_eq!(module.name(), "table");
    }

    #[test]
    fn test_table_module_exports() {
        let module = TableModule::new();
        let exports = module.exports().unwrap();
        assert!(exports.is_object());
        assert!(exports.get("merge").is_some());
        assert!(exports.get("clone").is_some());
        assert!(exports.get("keys").is_some());
        assert!(exports.get("values").is_some());
        assert!(exports.get("filter").is_some());
        assert!(exports.get("map").is_some());
        assert!(exports.get("reduce").is_some());
        assert!(exports.get("insert").is_some());
        assert!(exports.get("remove").is_some());
        assert!(exports.get("contains").is_some());
    }

    #[test]
    fn test_table_module_default() {
        let module = TableModule::default();
        assert_eq!(module.name(), "table");
    }

    #[test]
    fn test_table_module_exports_structure() {
        let module = TableModule::new();
        let exports = module.exports().unwrap();

        let merge = exports.get("merge").unwrap();
        assert!(merge.get("__fn").is_some());
        assert!(merge.get("__desc").is_some());
    }

    #[test]
    fn test_table_module_all_functions() {
        let module = TableModule::new();
        let exports = module.exports().unwrap();

        let functions = vec![
            "merge", "clone", "keys", "values", "filter", "map", "reduce", "insert", "remove",
            "contains",
        ];

        for func in functions {
            assert!(exports.get(func).is_some(), "Missing function: {}", func);
        }
    }

    #[test]
    fn test_table_module_init() {
        let mut module = TableModule::new();
        assert!(module.init().is_ok());
    }

    #[test]
    fn test_table_module_count_functions() {
        let module = TableModule::new();
        let exports = module.exports().unwrap();
        assert_eq!(exports.as_object().unwrap().len(), 10);
    }
}
