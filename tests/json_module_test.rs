use hype_rs::lua::require::setup_require_fn;
use hype_rs::modules::loader::ModuleLoader;
use mlua::Lua;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[test]
fn test_json_encode_object() {
    let lua = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua, loader).unwrap();

    let code = r#"
        local json = require("json")
        local data = {name = "Alice", age = 30, active = true}
        return json.encode(data)
    "#;

    let result: String = lua.load(code).eval().unwrap();
    assert!(result.contains("Alice"));
    assert!(result.contains("30"));
    assert!(result.contains("true"));
}

#[test]
fn test_json_encode_array() {
    let lua = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua, loader).unwrap();

    let code = r#"
        local json = require("json")
        local data = {1, 2, 3, 4, 5}
        return json.encode(data)
    "#;

    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "[1,2,3,4,5]");
}

#[test]
fn test_json_encode_pretty() {
    let lua = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua, loader).unwrap();

    let code = r#"
        local json = require("json")
        local data = {name = "Bob"}
        return json.encode(data, true)
    "#;

    let result: String = lua.load(code).eval().unwrap();
    assert!(result.contains('\n'));
    assert!(result.contains("Bob"));
}

#[test]
fn test_json_decode_object() {
    let lua = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua, loader).unwrap();

    let code = r#"
        local json = require("json")
        local data = json.decode('{"name":"Carol","age":25}')
        return data.name, data.age
    "#;

    let (name, age): (String, i64) = lua.load(code).eval().unwrap();
    assert_eq!(name, "Carol");
    assert_eq!(age, 25);
}

#[test]
fn test_json_decode_array() {
    let lua = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua, loader).unwrap();

    let code = r#"
        local json = require("json")
        local data = json.decode('[10,20,30]')
        return data[1], data[2], data[3]
    "#;

    let (a, b, c): (i64, i64, i64) = lua.load(code).eval().unwrap();
    assert_eq!(a, 10);
    assert_eq!(b, 20);
    assert_eq!(c, 30);
}

#[test]
fn test_json_roundtrip() {
    let lua = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua, loader).unwrap();

    let code = r#"
        local json = require("json")
        local original = {
            string = "test",
            number = 42,
            bool = true,
            array = {1, 2, 3},
            nested = {key = "value"}
        }
        local encoded = json.encode(original)
        local decoded = json.decode(encoded)
        return decoded.string, decoded.number, decoded.bool, decoded.array[2], decoded.nested.key
    "#;

    let (s, n, b, arr_val, nested_val): (String, i64, bool, i64, String) =
        lua.load(code).eval().unwrap();
    assert_eq!(s, "test");
    assert_eq!(n, 42);
    assert!(b);
    assert_eq!(arr_val, 2);
    assert_eq!(nested_val, "value");
}

#[test]
fn test_json_stringify_alias() {
    let lua = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua, loader).unwrap();

    let code = r#"
        local json = require("json")
        local data = {x = 1}
        return json.stringify(data)
    "#;

    let result: String = lua.load(code).eval().unwrap();
    assert!(result.contains("1"));
}

#[test]
fn test_json_parse_alias() {
    let lua = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua, loader).unwrap();

    let code = r#"
        local json = require("json")
        local data = json.parse('{"y":2}')
        return data.y
    "#;

    let result: i64 = lua.load(code).eval().unwrap();
    assert_eq!(result, 2);
}

#[test]
fn test_json_unicode() {
    let lua = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua, loader).unwrap();

    let code = r#"
        local json = require("json")
        local data = {text = "Hello 世界 🚀"}
        local encoded = json.encode(data)
        local decoded = json.decode(encoded)
        return decoded.text
    "#;

    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "Hello 世界 🚀");
}

#[test]
fn test_json_nested_structures() {
    let lua = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua, loader).unwrap();

    let code = r#"
        local json = require("json")
        local data = {
            users = {
                {name = "Alice", age = 30},
                {name = "Bob", age = 25}
            },
            metadata = {
                version = 1,
                timestamp = 1234567890
            }
        }
        local encoded = json.encode(data)
        local decoded = json.decode(encoded)
        return decoded.users[1].name, decoded.users[2].age, decoded.metadata.version
    "#;

    let (name, age, version): (String, i64, i64) = lua.load(code).eval().unwrap();
    assert_eq!(name, "Alice");
    assert_eq!(age, 25);
    assert_eq!(version, 1);
}

#[test]
fn test_json_null_handling() {
    let lua = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua, loader).unwrap();

    let code = r#"
        local json = require("json")
        local encoded = json.encode(nil)
        return encoded
    "#;

    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "null");
}

#[test]
fn test_json_error_invalid_syntax() {
    let lua = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua, loader).unwrap();

    let code = r#"
        local json = require("json")
        json.decode('{"invalid": }')
    "#;

    let result = lua.load(code).exec();
    assert!(result.is_err());
}
