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
fn test_process_cwd() {
    let lua = setup_lua();
    let code = r#"
        local process = require("process")
        local cwd = process.cwd()
        return type(cwd) == "string" and #cwd > 0
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_process_chdir() {
    let lua = setup_lua();
    let code = r#"
        local process = require("process")
        local original = process.cwd()
        local temp = "/tmp"
        process.chdir(temp)
        local new_cwd = process.cwd()
        process.chdir(original)
        return new_cwd:find("/tmp") ~= nil or new_cwd:find("/var") ~= nil
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_process_env_read() {
    let lua = setup_lua();
    std::env::set_var("HYPE_TEST_VAR", "test_value");
    let code = r#"
        local process = require("process")
        return process.env.HYPE_TEST_VAR
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "test_value");
    std::env::remove_var("HYPE_TEST_VAR");
}

#[test]
fn test_process_env_write() {
    let lua = setup_lua();
    let code = r#"
        local process = require("process")
        process.env.HYPE_WRITE_TEST = "written_value"
        return process.env.HYPE_WRITE_TEST
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "written_value");
    assert_eq!(std::env::var("HYPE_WRITE_TEST").unwrap(), "written_value");
    std::env::remove_var("HYPE_WRITE_TEST");
}

#[test]
fn test_process_getenv() {
    let lua = setup_lua();
    std::env::set_var("HYPE_GETENV_TEST", "getenv_value");
    let code = r#"
        local process = require("process")
        return process.getenv("HYPE_GETENV_TEST")
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "getenv_value");
    std::env::remove_var("HYPE_GETENV_TEST");
}

#[test]
fn test_process_setenv() {
    let lua = setup_lua();
    let code = r#"
        local process = require("process")
        process.setenv("HYPE_SETENV_TEST", "setenv_value")
        return process.getenv("HYPE_SETENV_TEST")
    "#;
    let result: String = lua.load(code).eval().unwrap();
    assert_eq!(result, "setenv_value");
    assert_eq!(std::env::var("HYPE_SETENV_TEST").unwrap(), "setenv_value");
    std::env::remove_var("HYPE_SETENV_TEST");
}

#[test]
fn test_process_pid() {
    let lua = setup_lua();
    let code = r#"
        local process = require("process")
        return type(process.pid) == "number" and process.pid > 0
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_process_platform() {
    let lua = setup_lua();
    let code = r#"
        local process = require("process")
        local platform = process.platform
        return type(platform) == "string" and #platform > 0
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_process_arch() {
    let lua = setup_lua();
    let code = r#"
        local process = require("process")
        local arch = process.arch
        return type(arch) == "string" and #arch > 0
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_process_argv() {
    let lua = setup_lua();
    let code = r#"
        local process = require("process")
        return type(process.argv) == "table" and #process.argv > 0
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_process_env_metatable() {
    let lua = setup_lua();
    let code = r#"
        local process = require("process")
        process.env.META_TEST = "meta_value"
        local value = process.env.META_TEST
        process.env.META_TEST = nil
        local after = process.env.META_TEST
        return value == "meta_value" and after == nil
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}

#[test]
fn test_process_integration_with_fs() {
    let lua = setup_lua();
    let code = r#"
        local process = require("process")
        local fs = require("fs")
        
        local cwd = process.cwd()
        local exists = fs.existsSync(cwd)
        return exists == true
    "#;
    let result: bool = lua.load(code).eval().unwrap();
    assert!(result);
}
