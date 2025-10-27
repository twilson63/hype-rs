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
fn test_querystring_parse() {
    let lua = setup_lua();
    lua.load(
        r#"
local qs = require("querystring")
local parsed = qs.parse("foo=bar&baz=qux")
assert(parsed.foo == "bar")
assert(parsed.baz == "qux")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_querystring_parse_with_encoding() {
    let lua = setup_lua();
    lua.load(
        r#"
local qs = require("querystring")
local parsed = qs.parse("name=John+Doe&email=test%40example.com")
assert(parsed.name == "John Doe")
assert(parsed.email == "test@example.com")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_querystring_stringify() {
    let lua = setup_lua();
    lua.load(
        r#"
local qs = require("querystring")
local result = qs.stringify({foo = "bar", baz = "qux"})
assert(string.find(result, "foo=bar") ~= nil)
assert(string.find(result, "baz=qux") ~= nil)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_querystring_stringify_with_spaces() {
    let lua = setup_lua();
    lua.load(
        r#"
local qs = require("querystring")
local result = qs.stringify({name = "John Doe"})
assert(string.find(result, "name=John+Doe", 1, true) ~= nil)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_querystring_escape() {
    let lua = setup_lua();
    lua.load(
        r#"
local qs = require("querystring")
assert(qs.escape("hello world") == "hello+world")
assert(qs.escape("foo@bar.com") == "foo%40bar.com")
assert(qs.escape("a&b=c") == "a%26b%3Dc")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_querystring_unescape() {
    let lua = setup_lua();
    lua.load(
        r#"
local qs = require("querystring")
assert(qs.unescape("hello+world") == "hello world")
assert(qs.unescape("foo%40bar.com") == "foo@bar.com")
assert(qs.unescape("a%26b%3Dc") == "a&b=c")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_querystring_escape_unescape_roundtrip() {
    let lua = setup_lua();
    lua.load(
        r#"
local qs = require("querystring")
local original = "hello world & foo=bar @#$"
local escaped = qs.escape(original)
local unescaped = qs.unescape(escaped)
assert(unescaped == original)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_querystring_parse_empty() {
    let lua = setup_lua();
    lua.load(
        r#"
local qs = require("querystring")
local parsed = qs.parse("")
local count = 0
for _ in pairs(parsed) do count = count + 1 end
assert(count == 0)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_querystring_stringify_empty() {
    let lua = setup_lua();
    lua.load(
        r#"
local qs = require("querystring")
local result = qs.stringify({})
assert(result == "")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_querystring_parse_single_param() {
    let lua = setup_lua();
    lua.load(
        r#"
local qs = require("querystring")
local parsed = qs.parse("key=value")
assert(parsed.key == "value")
local count = 0
for _ in pairs(parsed) do count = count + 1 end
assert(count == 1)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_querystring_parse_multiple_params() {
    let lua = setup_lua();
    lua.load(
        r#"
local qs = require("querystring")
local parsed = qs.parse("a=1&b=2&c=3")
assert(parsed.a == "1")
assert(parsed.b == "2")
assert(parsed.c == "3")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_querystring_parse_special_chars() {
    let lua = setup_lua();
    lua.load(
        r#"
local qs = require("querystring")
local parsed = qs.parse("msg=hello%20world%21&symbol=%40%23%24")
assert(parsed.msg == "hello world!")
assert(parsed.symbol == "@#$")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_querystring_stringify_special_chars() {
    let lua = setup_lua();
    lua.load(
        r#"
local qs = require("querystring")
local result = qs.stringify({msg = "hello world!", symbol = "@#$"})
assert(string.find(result, "msg=hello") ~= nil)
assert(string.find(result, "symbol=") ~= nil)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_querystring_parse_stringify_roundtrip() {
    let lua = setup_lua();
    lua.load(
        r#"
local qs = require("querystring")
local original = {name = "John Doe", email = "test@example.com", age = "30"}
local stringified = qs.stringify(original)
local parsed = qs.parse(stringified)
assert(parsed.name == original.name)
assert(parsed.email == original.email)
assert(parsed.age == original.age)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_querystring_require_loads_module() {
    let lua = setup_lua();
    lua.load(
        r#"
local qs = require("querystring")
assert(type(qs) == "table")
assert(type(qs.parse) == "function")
assert(type(qs.stringify) == "function")
assert(type(qs.escape) == "function")
assert(type(qs.unescape) == "function")
"#,
    )
    .exec()
    .unwrap();
}
