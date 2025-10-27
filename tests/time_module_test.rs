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
fn test_time_now() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local t = time.now()
assert(type(t) == "number")
assert(t > 0)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_now_seconds() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local t = time.nowSeconds()
assert(type(t) == "number")
assert(t > 0)
assert(t < time.now())
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_now_nanos() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local t = time.nowNanos()
assert(type(t) == "number")
assert(t > 0)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_to_iso_and_from_iso() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local timestamp = 1609459200000
local iso = time.toISO(timestamp)
assert(type(iso) == "string")
assert(string.find(iso, "2021"))
local parsed = time.fromISO(iso)
assert(parsed == timestamp)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_format() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local timestamp = 1609459200000
local formatted = time.format(timestamp, "%Y-%m-%d")
assert(formatted == "2021-01-01")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_date() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local timestamp = 1609459200000
local d = time.date(timestamp)
assert(type(d) == "table")
assert(d.year == 2021)
assert(d.month == 1)
assert(d.day == 1)
assert(type(d.hour) == "number")
assert(type(d.minute) == "number")
assert(type(d.second) == "number")
assert(type(d.weekday) == "number")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_date_current() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local d = time.date()
assert(type(d) == "table")
assert(d.year >= 2021)
assert(d.month >= 1 and d.month <= 12)
assert(d.day >= 1 and d.day <= 31)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_year() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local timestamp = 1609459200000
assert(time.year(timestamp) == 2021)
local current_year = time.year()
assert(current_year >= 2021)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_month() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local timestamp = 1609459200000
assert(time.month(timestamp) == 1)
local current_month = time.month()
assert(current_month >= 1 and current_month <= 12)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_day() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local timestamp = 1609459200000
assert(time.day(timestamp) == 1)
local current_day = time.day()
assert(current_day >= 1 and current_day <= 31)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_hour() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local timestamp = 1609459200000
local h = time.hour(timestamp)
assert(type(h) == "number")
assert(h >= 0 and h < 24)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_minute() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local timestamp = 1609459200000
local m = time.minute(timestamp)
assert(type(m) == "number")
assert(m >= 0 and m < 60)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_second() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local timestamp = 1609459200000
local s = time.second(timestamp)
assert(type(s) == "number")
assert(s >= 0 and s < 60)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_sleep() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local start = time.now()
time.sleep(50)
local elapsed = time.now() - start
assert(elapsed >= 50)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_elapsed() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local start = time.now()
time.sleep(10)
local elapsed = time.elapsed(start)
assert(elapsed >= 10)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_duration() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
assert(time.duration(500) == "500ms")
assert(time.duration(1500) == "1.500s")
assert(time.duration(65000) == "1m 5s")
assert(time.duration(3665000) == "1h 1m 5s")
assert(time.duration(90065000) == "1d 1h 1m")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_time_combined_operations() {
    let lua = setup_lua();
    lua.load(
        r#"
local time = require("time")
local now = time.now()
local iso = time.toISO(now)
local parsed = time.fromISO(iso)
assert(parsed == now)

local components = time.date(now)
assert(components.year > 2020)
assert(components.month >= 1 and components.month <= 12)
"#,
    )
    .exec()
    .unwrap();
}
