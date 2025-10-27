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

#[test]
fn test_empty_object() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local result = json.decode("{}")
        return type(result) == "table"
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_empty_array() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local result = json.decode("[]")
        return #result
    "#;
    let result: i64 = lua.load(code).eval().unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_simple_string() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('"hello"')
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_simple_number() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('42')
    "#;
    let result: i64 = lua.load(code).eval().unwrap();
    assert_eq!(result, 42);
}

#[test]
fn test_negative_number() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('-123')
    "#;
    let result: i64 = lua.load(code).eval().unwrap();
    assert_eq!(result, -123);
}

#[test]
fn test_float_number() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('3.14159')
    "#;
    let result: f64 = lua.load(code).eval().unwrap();
    assert!((result - 3.14159).abs() < 0.00001);
}

#[test]
fn test_exponential_number() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('1e10')
    "#;
    let result: f64 = lua.load(code).eval().unwrap();
    assert_eq!(result, 1e10);
}

#[test]
fn test_true_boolean() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('true')
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_false_boolean() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('false')
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(!result);
}

#[test]
fn test_null_value() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local result = json.decode('null')
        return result == nil
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_escaped_quotes() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local json_str = '"hello \\"world\\""'
        return json.decode(json_str)
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, r#"hello "world""#);
}

#[test]
fn test_escaped_backslash() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('"back\\\\slash"')
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "back\\slash");
}

#[test]
fn test_escaped_newline() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('"line1\\nline2"')
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "line1\nline2");
}

#[test]
fn test_escaped_tab() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('"tab\\there"')
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "tab\there");
}

#[test]
fn test_unicode_escape() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('"\\u0041\\u0042\\u0043"')
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "ABC");
}

#[test]
fn test_whitespace_handling() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('  {  "key"  :  "value"  }  ')
    "#;
    let result = lua.load(code).eval::<mlua::Value>().unwrap();
    assert!(result.is_table());
}

#[test]
fn test_nested_arrays() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local result = json.decode('[[1,2],[3,4]]')
        return result[1][2], result[2][1]
    "#;
    let (a, b): (i64, i64) = lua.load(code).eval().unwrap();
    assert_eq!(a, 2);
    assert_eq!(b, 3);
}

#[test]
fn test_nested_objects() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local result = json.decode('{"a":{"b":{"c":"deep"}}}')
        return result.a.b.c
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "deep");
}

#[test]
fn test_mixed_array() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local result = json.decode('[1, "two", true, null, {"five": 5}]')
        return result[1], result[2], result[3], result[5].five
    "#;
    let (n, s, b, five): (i64, String, bool, i64) = lua.load(code).eval().unwrap();
    assert_eq!(n, 1);
    assert_eq!(s, "two");
    assert!(b);
    assert_eq!(five, 5);
}

#[test]
fn test_object_with_array_values() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local result = json.decode('{"numbers":[1,2,3],"strings":["a","b"]}')
        return #result.numbers, #result.strings
    "#;
    let (num_len, str_len): (i64, i64) = lua.load(code).eval().unwrap();
    assert_eq!(num_len, 3);
    assert_eq!(str_len, 2);
}

#[test]
fn test_encode_special_characters() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local data = {text = "line1\nline2\ttab"}
        local encoded = json.encode(data)
        return encoded:match("\\n") ~= nil and encoded:match("\\t") ~= nil
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_encode_empty_table_as_object() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local empty = {}
        local encoded = json.encode(empty)
        return encoded == "{}" or encoded == "[]"
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_large_integer() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('9223372036854775807')
    "#;
    let result: i64 = lua.load(code).eval().unwrap();
    assert_eq!(result, 9223372036854775807i64);
}

#[test]
fn test_very_small_float() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('0.0000001')
    "#;
    let result: f64 = lua.load(code).eval().unwrap();
    assert!((result - 0.0000001).abs() < 0.00000001);
}

#[test]
fn test_zero() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('0')
    "#;
    let result: i64 = lua.load(code).eval().unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_negative_zero() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('-0')
    "#;
    let result: i64 = lua.load(code).eval().unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_array_with_trailing_comma_fails() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('[1,2,3,]')
    "#;
    let result = lua.load(code).exec();
    assert!(result.is_err());
}

#[test]
fn test_object_with_trailing_comma_fails() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('{"a":1,}')
    "#;
    let result = lua.load(code).exec();
    assert!(result.is_err());
}

#[test]
fn test_unclosed_array_fails() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('[1,2,3')
    "#;
    let result = lua.load(code).exec();
    assert!(result.is_err());
}

#[test]
fn test_unclosed_object_fails() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('{"key":"value"')
    "#;
    let result = lua.load(code).exec();
    assert!(result.is_err());
}

#[test]
fn test_missing_colon_fails() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('{"key" "value"}')
    "#;
    let result = lua.load(code).exec();
    assert!(result.is_err());
}

#[test]
fn test_missing_comma_fails() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('{"a":1 "b":2}')
    "#;
    let result = lua.load(code).exec();
    assert!(result.is_err());
}

#[test]
fn test_unquoted_key_fails() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode('{key:"value"}')
    "#;
    let result = lua.load(code).exec();
    assert!(result.is_err());
}

#[test]
fn test_single_quotes_fail() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        json.decode("{'key':'value'}")
    "#;
    let result = lua.load(code).exec();
    assert!(result.is_err());
}

#[test]
fn test_empty_string() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('""')
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_string_with_spaces() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        return json.decode('"hello   world"')
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "hello   world");
}

#[test]
fn test_deeply_nested_structure() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local deep = '{"a":{"b":{"c":{"d":{"e":{"f":"deep"}}}}}}'
        local result = json.decode(deep)
        return result.a.b.c.d.e.f
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "deep");
}

#[test]
fn test_array_of_objects() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local data = '[{"id":1},{"id":2},{"id":3}]'
        local result = json.decode(data)
        return result[1].id, result[2].id, result[3].id
    "#;
    let (a, b, c): (i64, i64, i64) = lua.load(code).eval().unwrap();
    assert_eq!((a, b, c), (1, 2, 3));
}

#[test]
fn test_encode_decode_preserves_types() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local original = {
            str = "text",
            num = 42,
            flt = 3.14,
            bool_t = true,
            bool_f = false,
            arr = {1, 2, 3}
        }
        local encoded = json.encode(original)
        local decoded = json.decode(encoded)
        return type(decoded.str) == "string",
               type(decoded.num) == "number",
               type(decoded.flt) == "number",
               type(decoded.bool_t) == "boolean",
               type(decoded.bool_f) == "boolean",
               type(decoded.arr) == "table"
    "#;
    let (str_ok, num_ok, flt_ok, bool_t_ok, bool_f_ok, arr_ok): (bool, bool, bool, bool, bool, bool) =
        lua.load(code).eval().unwrap();
    assert!(str_ok && num_ok && flt_ok && bool_t_ok && bool_f_ok && arr_ok);
}

#[test]
fn test_emoji_in_string() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local data = {emoji = "🚀 🌟 ⭐ 💎"}
        local encoded = json.encode(data)
        local decoded = json.decode(encoded)
        return decoded.emoji
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "🚀 🌟 ⭐ 💎");
}

#[test]
fn test_chinese_characters() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local data = {text = "你好世界"}
        local encoded = json.encode(data)
        local decoded = json.decode(encoded)
        return decoded.text
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "你好世界");
}

#[test]
fn test_mixed_unicode() {
    let lua = setup_lua();
    let code = r#"
        local json = require("json")
        local data = {text = "Hello 世界 🚀 مرحبا"}
        local encoded = json.encode(data)
        local decoded = json.decode(encoded)
        return decoded.text
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "Hello 世界 🚀 مرحبا");
}
