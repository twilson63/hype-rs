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
fn test_os_platform() {
    let lua = setup_lua();
    let result: String = lua
        .load(
            r#"
local os = require("os")
local platform = os.platform()
assert(type(platform) == "string", "platform should return string")
assert(platform == "linux" or platform == "macos" or platform == "windows" or platform == "freebsd" or platform == "openbsd", "platform should be valid OS")
return platform
"#,
        )
        .eval()
        .unwrap();
    assert!(["linux", "macos", "windows", "freebsd", "openbsd"].contains(&result.as_str()));
}

#[test]
fn test_os_arch() {
    let lua = setup_lua();
    let result: String = lua
        .load(
            r#"
local os = require("os")
local arch = os.arch()
assert(type(arch) == "string", "arch should return string")
assert(arch == "x86_64" or arch == "aarch64" or arch == "arm" or arch == "x86", "arch should be valid architecture")
return arch
"#,
        )
        .eval()
        .unwrap();
    assert!(["x86_64", "aarch64", "arm", "x86"].contains(&result.as_str()));
}

#[test]
fn test_os_hostname() {
    let lua = setup_lua();
    lua.load(
        r#"
local os = require("os")
local hostname = os.hostname()
assert(type(hostname) == "string")
assert(#hostname > 0)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_os_homedir() {
    let lua = setup_lua();
    lua.load(
        r#"
local os = require("os")
local homedir = os.homedir()
assert(type(homedir) == "string")
assert(#homedir > 0)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_os_tmpdir() {
    let lua = setup_lua();
    lua.load(
        r#"
local os = require("os")
local tmpdir = os.tmpdir()
assert(type(tmpdir) == "string")
assert(#tmpdir > 0)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_os_cpus() {
    let lua = setup_lua();
    lua.load(
        r#"
local os = require("os")
local cpus = os.cpus()
assert(type(cpus) == "table")
assert(#cpus > 0)
assert(type(cpus[1].model) == "string")
assert(type(cpus[1].speed) == "number")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_os_totalmem() {
    let lua = setup_lua();
    lua.load(
        r#"
local os = require("os")
local totalmem = os.totalmem()
assert(type(totalmem) == "number")
assert(totalmem > 0)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_os_freemem() {
    let lua = setup_lua();
    lua.load(
        r#"
local os = require("os")
local freemem = os.freemem()
assert(type(freemem) == "number")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_os_uptime() {
    let lua = setup_lua();
    lua.load(
        r#"
local os = require("os")
local uptime = os.uptime()
assert(type(uptime) == "number")
assert(uptime > 0)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_os_loadavg() {
    let lua = setup_lua();
    lua.load(
        r#"
local os = require("os")
local loadavg = os.loadavg()
assert(type(loadavg) == "table")
assert(type(loadavg[1]) == "number")
assert(type(loadavg[2]) == "number")
assert(type(loadavg[3]) == "number")
assert(loadavg[1] >= 0)
assert(loadavg[2] >= 0)
assert(loadavg[3] >= 0)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_os_network_interfaces() {
    let lua = setup_lua();
    lua.load(
        r#"
local os = require("os")
local interfaces = os.networkInterfaces()
assert(type(interfaces) == "table")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_os_user_info() {
    let lua = setup_lua();
    lua.load(
        r#"
local os = require("os")
local info = os.userInfo()
assert(type(info) == "table")
assert(type(info.username) == "string")
assert(#info.username > 0)
assert(type(info.homedir) == "string")
assert(#info.homedir > 0)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_os_eol() {
    let lua = setup_lua();
    lua.load(
        r#"
local os = require("os")
assert(type(os.EOL) == "string")
assert(os.EOL == "\n" or os.EOL == "\r\n")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_os_memory_relationship() {
    let lua = setup_lua();
    lua.load(
        r#"
local os = require("os")
local total = os.totalmem()
local free = os.freemem()
assert(free <= total)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_os_combined_usage() {
    let lua = setup_lua();
    lua.load(
        r#"
local os = require("os")
local info = {
    platform = os.platform(),
    arch = os.arch(),
    hostname = os.hostname(),
    homedir = os.homedir(),
    tmpdir = os.tmpdir(),
    cpus = #os.cpus(),
    totalmem = os.totalmem(),
    freemem = os.freemem(),
    uptime = os.uptime(),
    eol = os.EOL,
}
assert(type(info.platform) == "string")
assert(type(info.arch) == "string")
assert(type(info.hostname) == "string")
assert(type(info.homedir) == "string")
assert(type(info.tmpdir) == "string")
assert(type(info.cpus) == "number" and info.cpus > 0)
assert(type(info.totalmem) == "number" and info.totalmem > 0)
assert(type(info.freemem) == "number")
assert(type(info.uptime) == "number" and info.uptime > 0)
assert(type(info.eol) == "string")
"#,
    )
    .exec()
    .unwrap();
}
