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
fn test_url_parse() {
    let lua = setup_lua();
    lua.load(
        r#"
local url = require("url")
local parsed = url.parse("https://example.com:8080/path?query=1#hash")
assert(parsed.protocol == "https")
assert(parsed.hostname == "example.com")
assert(parsed.port == 8080)
assert(parsed.path == "/path")
assert(parsed.query == "query=1")
assert(parsed.hash == "hash")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_url_parse_with_auth() {
    let lua = setup_lua();
    lua.load(
        r#"
local url = require("url")
local parsed = url.parse("https://user:pass@example.com/path")
assert(parsed.username == "user")
assert(parsed.password == "pass")
assert(parsed.hostname == "example.com")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_url_format() {
    let lua = setup_lua();
    lua.load(
        r#"
local url = require("url")
local formatted = url.format({
    protocol = "https",
    hostname = "example.com",
    port = 8080,
    path = "/path",
    query = "key=value",
    hash = "section"
})
assert(formatted == "https://example.com:8080/path?key=value#section")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_url_resolve() {
    let lua = setup_lua();
    lua.load(
        r#"
local url = require("url")
local resolved = url.resolve("https://example.com/foo/bar", "../baz")
assert(resolved == "https://example.com/baz")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_url_resolve_absolute() {
    let lua = setup_lua();
    lua.load(
        r#"
local url = require("url")
local resolved = url.resolve("https://example.com/foo", "/bar")
assert(resolved == "https://example.com/bar")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_url_encode_component() {
    let lua = setup_lua();
    lua.load(
        r#"
local url = require("url")
assert(url.encodeComponent("hello world") == "hello+world")
assert(url.encodeComponent("foo@bar.com") == "foo%40bar.com")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_url_decode_component() {
    let lua = setup_lua();
    lua.load(
        r#"
local url = require("url")
assert(url.decodeComponent("hello%20world") == "hello world")
assert(url.decodeComponent("foo%40bar.com") == "foo@bar.com")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_url_encode_decode_roundtrip() {
    let lua = setup_lua();
    lua.load(
        r#"
local url = require("url")
local original = "hello world & foo=bar"
local encoded = url.encodeComponent(original)
local decoded = url.decodeComponent(encoded)
assert(decoded == original)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_url_parse_query() {
    let lua = setup_lua();
    lua.load(
        r#"
local url = require("url")
local params = url.parseQuery("foo=bar&baz=qux&name=value")
assert(params.foo == "bar")
assert(params.baz == "qux")
assert(params.name == "value")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_url_format_query() {
    let lua = setup_lua();
    lua.load(
        r#"
local url = require("url")
local query = url.formatQuery({foo = "bar", baz = "qux"})
assert(type(query) == "string")
assert(string.find(query, "foo=bar"))
assert(string.find(query, "baz=qux"))
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_url_parse_query_with_encoding() {
    let lua = setup_lua();
    lua.load(
        r#"
local url = require("url")
local params = url.parseQuery("message=hello+world&email=foo%40bar.com")
assert(params.message == "hello world")
assert(params.email == "foo@bar.com")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_url_combined_operations() {
    let lua = setup_lua();
    lua.load(
        r#"
local url = require("url")
local original = "https://api.example.com:443/users?active=true#top"
local parsed = url.parse(original)
assert(parsed.protocol == "https")
assert(parsed.hostname == "api.example.com")

local components = {
    protocol = parsed.protocol,
    hostname = parsed.hostname,
    path = "/updated"
}
local new_url = url.format(components)
assert(string.find(new_url, "https://api.example.com/updated"))
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_url_query_params_workflow() {
    let lua = setup_lua();
    lua.load(
        r#"
local url = require("url")
local full_url = "https://api.example.com/search?q=test&limit=10"
local parsed = url.parse(full_url)
local params = url.parseQuery(parsed.query)
assert(params.q == "test")
assert(params.limit == "10")

params.page = "2"
local new_query = url.formatQuery(params)
local updated_url = url.format({
    protocol = parsed.protocol,
    hostname = parsed.hostname,
    path = parsed.path,
    query = new_query
})
assert(string.find(updated_url, "page=2"))
"#,
    )
    .exec()
    .unwrap();
}
