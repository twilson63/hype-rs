use mlua::{Lua, Table, Value};
use serde_json::Value as JsonValue;
use std::sync::{Arc, Mutex};

use crate::error::Result;
use crate::modules::loader::ModuleLoader;

pub struct RequireSetup;

impl RequireSetup {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RequireSetup {
    fn default() -> Self {
        Self::new()
    }
}

pub fn setup_require_fn(lua: &Lua, loader: Arc<Mutex<ModuleLoader>>) -> Result<()> {
    let globals = lua.globals();

    let require_table = lua.create_table()?;
    let cache_table = lua.create_table()?;
    require_table.set("cache", cache_table)?;

    let loader_clone = Arc::clone(&loader);
    let require_fn =
        lua.create_function(move |lua_ctx: &Lua, (_table, module_id): (Table, String)| {
            let mut loader_lock = loader_clone.lock().map_err(|_| {
                mlua::Error::RuntimeError("Failed to acquire module loader lock".to_string())
            })?;

            let exports = loader_lock.require(&module_id).map_err(|err| {
                mlua::Error::RuntimeError(format!("Failed to load module '{}': {}", module_id, err))
            })?;

            let lua_exports = json_to_lua(lua_ctx, &exports).map_err(|err| {
                mlua::Error::RuntimeError(format!(
                    "Failed to convert module exports to Lua: {}",
                    err
                ))
            })?;

            update_require_cache(lua_ctx, &loader_lock)?;

            Ok(lua_exports)
        })?;

    create_resolve_fn(lua, &require_table, Arc::clone(&loader))?;

    let metatable = lua.create_table()?;
    metatable.set("__call", require_fn)?;
    require_table.set_metatable(Some(metatable));

    globals.set("require", require_table)?;

    let loader_lock = loader.lock().map_err(|_| {
        mlua::Error::ExternalError(Arc::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to acquire module loader lock",
        )))
    })?;
    update_require_cache(lua, &loader_lock)?;

    Ok(())
}

fn update_require_cache(lua: &Lua, loader: &ModuleLoader) -> mlua::Result<()> {
    let require_table: Table = lua.globals().get("require")?;
    let cache_table: Table = require_table.get("cache")?;

    let cached_modules = loader.cached_modules().map_err(|err| {
        mlua::Error::RuntimeError(format!("Failed to get cached modules: {}", err))
    })?;

    let registry = loader.registry();

    for module_key in cached_modules {
        let json_exports = registry.get(&module_key).map_err(|err| {
            mlua::Error::RuntimeError(format!("Failed to get module from registry: {}", err))
        })?;

        if let Some(exports_json) = json_exports {
            let lua_table = json_to_lua(lua, &exports_json).map_err(|err| {
                mlua::Error::RuntimeError(format!("Failed to convert module to Lua: {}", err))
            })?;
            cache_table.set(module_key, lua_table)?;
        }
    }

    Ok(())
}

fn create_resolve_fn(
    lua: &Lua,
    require_table: &Table,
    loader: Arc<Mutex<ModuleLoader>>,
) -> mlua::Result<()> {
    let resolve_fn = lua.create_function(move |_lua_ctx: &Lua, module_id: String| {
        let loader_lock = loader.lock().map_err(|_| {
            mlua::Error::RuntimeError("Failed to acquire module loader lock".to_string())
        })?;

        let resolver = loader_lock.resolver();
        let path = resolver.resolve(&module_id).map_err(|err| {
            mlua::Error::RuntimeError(format!("Failed to resolve module '{}': {}", module_id, err))
        })?;

        Ok(path.to_string_lossy().to_string())
    })?;

    require_table.set("resolve", resolve_fn)?;
    Ok(())
}

fn json_to_lua<'a>(lua: &'a Lua, value: &JsonValue) -> Result<Value<'a>> {
    match value {
        JsonValue::Null => Ok(Value::Nil),
        JsonValue::Bool(b) => Ok(Value::Boolean(*b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::Number(f))
            } else {
                Ok(Value::Number(0.0))
            }
        }
        JsonValue::String(s) => Ok(Value::String(lua.create_string(s)?)),
        JsonValue::Array(arr) => {
            let table = lua.create_table()?;
            for (i, item) in arr.iter().enumerate() {
                let lua_val = json_to_lua(lua, item)?;
                table.set(i + 1, lua_val)?;
            }
            Ok(Value::Table(table))
        }
        JsonValue::Object(obj) => {
            let table = lua.create_table()?;
            for (key, val) in obj.iter() {
                let lua_val = json_to_lua(lua, val)?;
                table.set(key.as_str(), lua_val)?;
            }
            Ok(Value::Table(table))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_require_setup_new() {
        let setup = RequireSetup::new();
        let _ = setup;
    }

    #[test]
    fn test_require_setup_default() {
        let setup = RequireSetup::default();
        let _ = setup;
    }

    #[test]
    fn test_json_to_lua_null() {
        let lua = Lua::new();
        let result = json_to_lua(&lua, &JsonValue::Null).unwrap();
        assert!(matches!(result, Value::Nil));
    }

    #[test]
    fn test_json_to_lua_bool_true() {
        let lua = Lua::new();
        let result = json_to_lua(&lua, &JsonValue::Bool(true)).unwrap();
        assert!(matches!(result, Value::Boolean(true)));
    }

    #[test]
    fn test_json_to_lua_bool_false() {
        let lua = Lua::new();
        let result = json_to_lua(&lua, &JsonValue::Bool(false)).unwrap();
        assert!(matches!(result, Value::Boolean(false)));
    }

    #[test]
    fn test_json_to_lua_integer() {
        let lua = Lua::new();
        let result = json_to_lua(&lua, &serde_json::json!(42)).unwrap();
        match result {
            Value::Integer(i) => assert_eq!(i, 42),
            _ => panic!("Expected integer"),
        }
    }

    #[test]
    fn test_json_to_lua_float() {
        let lua = Lua::new();
        let result = json_to_lua(&lua, &serde_json::json!(3.14)).unwrap();
        match result {
            Value::Number(n) => assert!((n - 3.14).abs() < 0.001),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_json_to_lua_string() {
        let lua = Lua::new();
        let result = json_to_lua(&lua, &JsonValue::String("hello".to_string())).unwrap();
        match result {
            Value::String(s) => assert_eq!(s.to_str().unwrap(), "hello"),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_json_to_lua_empty_array() {
        let lua = Lua::new();
        let result = json_to_lua(&lua, &serde_json::json!([])).unwrap();
        match result {
            Value::Table(t) => {
                assert_eq!(t.len().unwrap(), 0);
            }
            _ => panic!("Expected table"),
        }
    }

    #[test]
    fn test_json_to_lua_array_with_values() {
        let lua = Lua::new();
        let result = json_to_lua(&lua, &serde_json::json!([1, 2, 3])).unwrap();
        match result {
            Value::Table(t) => {
                assert_eq!(t.get::<_, i64>(1).unwrap(), 1);
                assert_eq!(t.get::<_, i64>(2).unwrap(), 2);
                assert_eq!(t.get::<_, i64>(3).unwrap(), 3);
            }
            _ => panic!("Expected table"),
        }
    }

    #[test]
    fn test_json_to_lua_empty_object() {
        let lua = Lua::new();
        let result = json_to_lua(&lua, &serde_json::json!({})).unwrap();
        match result {
            Value::Table(t) => {
                assert_eq!(t.len().unwrap(), 0);
            }
            _ => panic!("Expected table"),
        }
    }

    #[test]
    fn test_json_to_lua_object_with_values() {
        let lua = Lua::new();
        let result = json_to_lua(&lua, &serde_json::json!({"a": 1, "b": "hello"})).unwrap();
        match result {
            Value::Table(t) => {
                assert_eq!(t.get::<_, i64>("a").unwrap(), 1);
                assert_eq!(t.get::<_, String>("b").unwrap(), "hello");
            }
            _ => panic!("Expected table"),
        }
    }

    #[test]
    fn test_json_to_lua_nested_object() {
        let lua = Lua::new();
        let result = json_to_lua(&lua, &serde_json::json!({"outer": {"inner": 42}})).unwrap();
        match result {
            Value::Table(t) => {
                let outer: Table = t.get("outer").unwrap();
                assert_eq!(outer.get::<_, i64>("inner").unwrap(), 42);
            }
            _ => panic!("Expected table"),
        }
    }

    #[test]
    fn test_json_to_lua_nested_array() {
        let lua = Lua::new();
        let result = json_to_lua(&lua, &serde_json::json!([[1, 2], [3, 4]])).unwrap();
        match result {
            Value::Table(t) => {
                let inner1: Table = t.get(1).unwrap();
                assert_eq!(inner1.get::<_, i64>(1).unwrap(), 1);
                assert_eq!(inner1.get::<_, i64>(2).unwrap(), 2);

                let inner2: Table = t.get(2).unwrap();
                assert_eq!(inner2.get::<_, i64>(1).unwrap(), 3);
                assert_eq!(inner2.get::<_, i64>(2).unwrap(), 4);
            }
            _ => panic!("Expected table"),
        }
    }

    #[test]
    fn test_setup_require_fn_fs_module() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        let result = setup_require_fn(&lua, loader);
        assert!(result.is_ok());

        let globals = lua.globals();
        let _require_table: Table = globals.get("require").unwrap();
    }

    #[test]
    fn test_setup_require_fn_call_builtin() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<()> = lua
            .load("local fs = require('fs'); assert(fs ~= nil)")
            .eval();
        assert!(result.is_ok());
    }

    #[test]
    fn test_setup_require_fn_returns_table() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<String> =
            lua.load("local fs = require('fs'); return type(fs)").eval();
        assert_eq!(result.unwrap(), "table");
    }

    #[test]
    fn test_setup_require_fn_call_path_module() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<()> = lua
            .load("local path = require('path'); assert(path ~= nil)")
            .eval();
        assert!(result.is_ok());
    }

    #[test]
    fn test_setup_require_fn_caching() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<()> = lua
            .load(
                "
            local fs1 = require('fs')
            local fs2 = require('fs')
            assert(type(fs1) == 'table', 'First module should be a table')
            assert(type(fs2) == 'table', 'Second module should be a table')
        ",
            )
            .eval();
        assert!(result.is_ok());
    }

    #[test]
    fn test_setup_require_fn_module_not_found() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<()> = lua
            .load("local nonexistent = require('nonexistent-module-xyz')")
            .eval();
        assert!(result.is_err());
    }

    #[test]
    fn test_json_to_lua_mixed_types() {
        let lua = Lua::new();
        let json_val = serde_json::json!({
            "string_val": "hello",
            "int_val": 42,
            "float_val": 3.14,
            "bool_val": true,
            "null_val": null,
            "array_val": [1, 2, 3],
            "object_val": {"nested": "value"}
        });

        let result = json_to_lua(&lua, &json_val).unwrap();
        match result {
            Value::Table(t) => {
                assert_eq!(t.get::<_, String>("string_val").unwrap(), "hello");
                assert_eq!(t.get::<_, i64>("int_val").unwrap(), 42);
                let float_val: f64 = t.get("float_val").unwrap();
                assert!((float_val - 3.14).abs() < 0.001);
                assert_eq!(t.get::<_, bool>("bool_val").unwrap(), true);
                assert!(matches!(
                    t.get::<_, mlua::Value>("null_val").unwrap(),
                    mlua::Value::Nil
                ));

                let array: Table = t.get("array_val").unwrap();
                assert_eq!(array.get::<_, i64>(1).unwrap(), 1);

                let obj: Table = t.get("object_val").unwrap();
                assert_eq!(obj.get::<_, String>("nested").unwrap(), "value");
            }
            _ => panic!("Expected table"),
        }
    }

    #[test]
    fn test_setup_require_fn_multiple_modules() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<()> = lua
            .load(
                "
            local fs = require('fs')
            local path = require('path')
            assert(fs ~= nil)
            assert(path ~= nil)
            assert(fs ~= path)
        ",
            )
            .eval();
        assert!(result.is_ok());
    }

    #[test]
    fn test_require_cache_exists() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load("return require.cache ~= nil and type(require.cache) == 'table'")
            .eval();
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_require_cache_contains_loaded_modules() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                "
            local fs = require('fs')
            local cache = require.cache
            local has_fs = false
            for key, value in pairs(cache) do
                if string.find(key, 'fs') then
                    has_fs = true
                    break
                end
            end
            return has_fs
        ",
            )
            .eval();
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_require_resolve_function_exists() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load("return require.resolve ~= nil and type(require.resolve) == 'function'")
            .eval();
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_require_resolve_returns_path() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<String> = lua.load("return require.resolve('fs')").eval();
        let path = result.unwrap();
        assert!(path.contains("fs") || path.contains("builtin"));
    }

    #[test]
    fn test_require_resolve_builtin_module() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<String> = lua.load("return require.resolve('path')").eval();
        let path = result.unwrap();
        assert!(path.contains("path") || path.contains("builtin"));
    }

    #[test]
    fn test_require_resolve_nonexistent_module_error() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<String> = lua
            .load("return require.resolve('nonexistent-module-xyz')")
            .eval();
        assert!(result.is_err());
    }

    #[test]
    fn test_require_still_callable() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load("local fs = require('fs'); return fs ~= nil and type(fs) == 'table'")
            .eval();
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_require_cache_persists_across_requires() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                "
            require('fs')
            local cache1 = require.cache
            local cache1_size = 0
            for _ in pairs(cache1) do
                cache1_size = cache1_size + 1
            end

            require('path')
            local cache2 = require.cache
            local cache2_size = 0
            for _ in pairs(cache2) do
                cache2_size = cache2_size + 1
            end

            return cache2_size > cache1_size
        ",
            )
            .eval();
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_require_cache_has_module_exports() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                "
            local fs = require('fs')
            local cache = require.cache
            local found = false
            for key, cached_module in pairs(cache) do
                if cached_module and type(cached_module) == 'table' and cached_module.__id then
                    found = true
                    break
                end
            end
            return found
        ",
            )
            .eval();
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_require_resolve_all_builtins() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                "
            local modules = {'fs', 'path', 'events', 'util', 'table'}
            for _, module_id in ipairs(modules) do
                local path = require.resolve(module_id)
                if not path or string.len(path) == 0 then
                    return false
                end
            end
            return true
        ",
            )
            .eval();
        assert_eq!(result.unwrap(), true);
    }
}
