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
fn test_string_split() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
local result = string.split("a,b,c", ",")
assert(#result == 3)
assert(result[1] == "a")
assert(result[2] == "b")
assert(result[3] == "c")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_split_empty_delimiter() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
local result = string.split("abc", "")
assert(#result == 3)
assert(result[1] == "a")
assert(result[2] == "b")
assert(result[3] == "c")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_trim() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
assert(string.trim("  hello  ") == "hello")
assert(string.trim("hello") == "hello")
assert(string.trim("  ") == "")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_trim_start() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
assert(string.trimStart("  hello  ") == "hello  ")
assert(string.trimStart("hello") == "hello")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_trim_end() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
assert(string.trimEnd("  hello  ") == "  hello")
assert(string.trimEnd("hello") == "hello")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_starts_with() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
assert(string.startsWith("hello", "hel") == true)
assert(string.startsWith("hello", "ell") == false)
assert(string.startsWith("", "") == true)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_ends_with() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
assert(string.endsWith("hello", "llo") == true)
assert(string.endsWith("hello", "ell") == false)
assert(string.endsWith("", "") == true)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_contains() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
assert(string.contains("hello world", "wo") == true)
assert(string.contains("hello", "xyz") == false)
assert(string.contains("", "") == true)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_pad_start() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
assert(string.padStart("5", 3) == "  5")
assert(string.padStart("5", 3, "0") == "005")
assert(string.padStart("hello", 3) == "hello")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_pad_end() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
assert(string.padEnd("5", 3) == "5  ")
assert(string.padEnd("5", 3, "0") == "500")
assert(string.padEnd("hello", 3) == "hello")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_repeat_fn() {
    let lua = setup_lua();
    lua.load(
        r#"
local strmod = require("string")
local repeat_fn = strmod["repeat"]
assert(repeat_fn("ab", 3) == "ababab")
assert(repeat_fn("x", 0) == "")
assert(repeat_fn("", 5) == "")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_replace() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
assert(string.replace("hello hello", "l", "L", 2) == "heLLo hello")
assert(string.replace("abc", "", "x", 1) == "abc")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_replace_all() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
assert(string.replaceAll("hello hello", "l", "L") == "heLLo heLLo")
assert(string.replaceAll("abc", "x", "y") == "abc")
assert(string.replaceAll("", "a", "b") == "")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_to_upper_case() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
assert(string.toUpperCase("hello") == "HELLO")
assert(string.toUpperCase("Hello") == "HELLO")
assert(string.toUpperCase("") == "")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_to_lower_case() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
assert(string.toLowerCase("HELLO") == "hello")
assert(string.toLowerCase("Hello") == "hello")
assert(string.toLowerCase("") == "")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_capitalize() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
assert(string.capitalize("hello") == "Hello")
assert(string.capitalize("HELLO") == "HELLO")
assert(string.capitalize("") == "")
assert(string.capitalize("h") == "H")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_lines() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
local result = string.lines("a\nb\nc")
assert(#result == 3)
assert(result[1] == "a")
assert(result[2] == "b")
assert(result[3] == "c")

local single = string.lines("one")
assert(#single == 1)
assert(single[1] == "one")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_chars() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
local result = string.chars("abc")
assert(#result == 3)
assert(result[1] == "a")
assert(result[2] == "b")
assert(result[3] == "c")

local single = string.chars("x")
assert(#single == 1)
assert(single[1] == "x")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_string_combined_operations() {
    let lua = setup_lua();
    lua.load(
        r#"
local string = require("string")
local text = "  hello world  "
text = string.trim(text)
assert(text == "hello world")
text = string.toUpperCase(text)
assert(text == "HELLO WORLD")
text = string.replaceAll(text, "HELLO", "HI")
assert(text == "HI WORLD")
"#,
    )
    .exec()
    .unwrap();
}
