// Extreme edge cases based on JSONTestSuite patterns
// Testing behavior that should fail or handle gracefully

use hype_rs::lua::require::setup_require_fn;
use hype_rs::modules::loader::ModuleLoader;
use mlua::Lua;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

fn setup_lua() -> Lua {
    let lua = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua, loader).unwrap();
    lua
}

// MUST REJECT: Invalid JSON syntax
#[test]
fn test_reject_bare_word() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('hello')
    "#;
    assert!(lua.load(code).exec().is_err());
}

#[test]
fn test_reject_undefined() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('undefined')
    "#;
    assert!(lua.load(code).exec().is_err());
}

#[test]
fn test_reject_nan() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('NaN')
    "#;
    assert!(lua.load(code).exec().is_err());
}

#[test]
fn test_reject_infinity() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('Infinity')
    "#;
    assert!(lua.load(code).exec().is_err());
}

#[test]
fn test_reject_hex_number() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('0x1234')
    "#;
    assert!(lua.load(code).exec().is_err());
}

#[test]
fn test_reject_leading_plus() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('+123')
    "#;
    assert!(lua.load(code).exec().is_err());
}

#[test]
fn test_reject_leading_zero() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('0123')
    "#;
    assert!(lua.load(code).exec().is_err());
}

#[test]
fn test_reject_comments() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('{"key":"value"} // comment')
    "#;
    assert!(lua.load(code).exec().is_err());
}

#[test]
fn test_reject_multiline_comment() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('/* comment */ {"key":"value"}')
    "#;
    assert!(lua.load(code).exec().is_err());
}

#[test]
fn test_reject_duplicate_keys_allowed() {
    // Note: serde_json allows duplicate keys, last one wins
    // This is implementation-defined behavior
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local result = json.decode('{"key":"value1","key":"value2"}')
        return result.key
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "value2");
}

// MUST ACCEPT: Valid but unusual JSON
#[test]
fn test_accept_deep_nesting() {
    let lua = setup_lua();
    let mut json = String::from("[");
    for _ in 0..100 {
        json.push_str("[");
    }
    for _ in 0..100 {
        json.push_str("]");
    }
    json.push_str("]");

    let code = format!(
        r#"
        local json = require("json")
        local result = json.decode('{}')
        return type(result) == "table"
    "#,
        json
    );
    let result: bool = lua.load(&code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_accept_long_string() {
    let lua = setup_lua();
    let long_str = "a".repeat(10000);
    let json = format!(r#"{{"text":"{}"}}"#, long_str);

    let code = format!(
        r#"
        local json = require("json")
        local result = json.decode('{}')
        return #result.text
    "#,
        json
    );
    let result: i64 = lua.load(&code).eval().unwrap();
    assert_eq!(result, 10000);
}

#[test]
fn test_accept_large_array() {
    let lua = setup_lua();
    let mut json = String::from("[");
    for i in 0..1000 {
        if i > 0 {
            json.push(',');
        }
        json.push_str(&i.to_string());
    }
    json.push(']');

    let code = format!(
        r#"
        local json = require("json")
        local result = json.decode('{}')
        return #result
    "#,
        json
    );
    let result: i64 = lua.load(&code).eval().unwrap();
    assert_eq!(result, 1000);
}

#[test]
fn test_accept_many_object_keys() {
    let lua = setup_lua();
    let mut parts = vec![];
    for i in 0..100 {
        parts.push(format!(r#""key{}":"{}""#, i, i));
    }
    let json = format!("{{{}}}", parts.join(","));

    let code = format!(
        r#"
        local json = require("json")
        local result = json.decode([[{}]])
        return result.key50
    "#,
        json
    );
    let result: String = lua.load(&code).eval().unwrap();
    assert_eq!(result, "50");
}

#[test]
fn test_accept_string_with_all_escapes() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local json_str = '"\\\"\\/\\b\\f\\n\\r\\t"'
        local result = json.decode(json_str)
        return #result > 0
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_accept_number_zero_point_zero() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('0.0')
    "#;
    let result: f64 = lua.load(code).eval().unwrap();
    assert_eq!(result, 0.0);
}

#[test]
fn test_accept_negative_exponent() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('1e-10')
    "#;
    let result: f64 = lua.load(code).eval().unwrap();
    assert_eq!(result, 1e-10);
}

#[test]
fn test_accept_positive_exponent() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('1e+10')
    "#;
    let result: f64 = lua.load(code).eval().unwrap();
    assert_eq!(result, 1e10);
}

#[test]
fn test_accept_capital_e_exponent() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('1E10')
    "#;
    let result: f64 = lua.load(code).eval().unwrap();
    assert_eq!(result, 1e10);
}

// Encoding edge cases
#[test]
fn test_encode_large_number() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local data = {num = 9007199254740991}
        local encoded = json.encode(data)
        local decoded = json.decode(encoded)
        return decoded.num
    "#;
    let result: i64 = lua.load(code).eval().unwrap();
    assert_eq!(result, 9007199254740991i64);
}

#[test]
fn test_encode_very_small_number() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local data = {num = -9007199254740991}
        local encoded = json.encode(data)
        local decoded = json.decode(encoded)
        return decoded.num
    "#;
    let result: i64 = lua.load(code).eval().unwrap();
    assert_eq!(result, -9007199254740991i64);
}

#[test]
fn test_encode_boolean_values() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local data = {t = true, f = false}
        local encoded = json.encode(data)
        return encoded:match('"t":true') ~= nil and encoded:match('"f":false') ~= nil
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_roundtrip_complex() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local original = {
            users = {
                {id = 1, name = "Alice", tags = {"admin", "user"}},
                {id = 2, name = "Bob", tags = {"user"}}
            },
            metadata = {
                version = 1,
                timestamp = 1234567890,
                enabled = true,
                config = {
                    timeout = 30,
                    retries = 3,
                    endpoints = {"api.example.com", "backup.example.com"}
                }
            }
        }
        
        local encoded = json.encode(original)
        local decoded = json.decode(encoded)
        
        return decoded.users[1].name == "Alice" and
               decoded.users[2].tags[1] == "user" and
               decoded.metadata.version == 1 and
               decoded.metadata.enabled == true and
               decoded.metadata.config.timeout == 30 and
               decoded.metadata.config.endpoints[1] == "api.example.com"
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_empty_string_key() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local result = json.decode('{"":"empty key"}')
        return result[""]
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "empty key");
}

#[test]
fn test_special_chars_in_key() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local result = json.decode('{"key with spaces":"value"}')
        return result["key with spaces"]
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "value");
}

#[test]
fn test_number_string_in_object() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local result = json.decode('{"123":"numeric key"}')
        return result["123"]
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "numeric key");
}
