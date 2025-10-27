use super::operations::*;
use mlua::{Lua, Table, Value as LuaValue};

pub fn create_process_module(lua: &Lua) -> mlua::Result<Table> {
    let process_table = lua.create_table()?;

    register_cwd(lua, &process_table)?;
    register_chdir(lua, &process_table)?;
    register_env(lua, &process_table)?;
    register_getenv(lua, &process_table)?;
    register_setenv(lua, &process_table)?;
    register_pid(lua, &process_table)?;
    register_platform(lua, &process_table)?;
    register_arch(lua, &process_table)?;
    register_exit(lua, &process_table)?;
    register_argv(lua, &process_table)?;

    Ok(process_table)
}

fn register_cwd(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let cwd_fn = lua.create_function(|_, ()| get_cwd().map_err(mlua::Error::external))?;
    table.set("cwd", cwd_fn)?;
    Ok(())
}

fn register_chdir(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let chdir_fn =
        lua.create_function(|_, path: String| set_cwd(&path).map_err(mlua::Error::external))?;
    table.set("chdir", chdir_fn)?;
    Ok(())
}

fn register_env(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let env_table = lua.create_table()?;
    for (key, value) in get_env() {
        env_table.set(key, value)?;
    }

    let env_metatable = lua.create_table()?;
    env_metatable.set(
        "__index",
        lua.create_function(|_, (_, key): (LuaValue, String)| Ok(get_env_var(&key)))?,
    )?;
    env_metatable.set(
        "__newindex",
        lua.create_function(|_, (_, key, value): (LuaValue, String, Option<String>)| {
            if let Some(val) = value {
                set_env_var(&key, &val);
            } else {
                remove_env_var(&key);
            }
            Ok(())
        })?,
    )?;

    env_table.set_metatable(Some(env_metatable));
    table.set("env", env_table)?;
    Ok(())
}

fn register_getenv(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let getenv_fn = lua.create_function(|_, key: String| Ok(get_env_var(&key)))?;
    table.set("getenv", getenv_fn)?;
    Ok(())
}

fn register_setenv(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let setenv_fn = lua.create_function(|_, (key, value): (String, String)| {
        set_env_var(&key, &value);
        Ok(())
    })?;
    table.set("setenv", setenv_fn)?;
    Ok(())
}

fn register_pid(_lua: &Lua, table: &Table) -> mlua::Result<()> {
    table.set("pid", get_pid())?;
    Ok(())
}

fn register_platform(_lua: &Lua, table: &Table) -> mlua::Result<()> {
    table.set("platform", get_platform())?;
    Ok(())
}

fn register_arch(_lua: &Lua, table: &Table) -> mlua::Result<()> {
    table.set("arch", get_arch())?;
    Ok(())
}

fn register_exit(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let exit_fn = lua.create_function(|_, code: Option<i32>| -> mlua::Result<()> {
        let exit_code = code.unwrap_or(0);
        if !(0..=255).contains(&exit_code) {
            return Err(mlua::Error::external("Exit code must be between 0 and 255"));
        }
        exit(exit_code);
    })?;
    table.set("exit", exit_fn)?;
    Ok(())
}

fn register_argv(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let argv_table = lua.create_table()?;
    for (i, arg) in args.iter().enumerate() {
        argv_table.set(i + 1, arg.as_str())?;
    }
    table.set("argv", argv_table)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_process_module() {
        let lua = Lua::new();
        let result = create_process_module(&lua);
        assert!(result.is_ok());

        let process_table = result.unwrap();
        assert!(process_table.contains_key("cwd").unwrap());
        assert!(process_table.contains_key("chdir").unwrap());
        assert!(process_table.contains_key("env").unwrap());
        assert!(process_table.contains_key("getenv").unwrap());
        assert!(process_table.contains_key("setenv").unwrap());
        assert!(process_table.contains_key("pid").unwrap());
        assert!(process_table.contains_key("platform").unwrap());
        assert!(process_table.contains_key("arch").unwrap());
        assert!(process_table.contains_key("argv").unwrap());
    }

    #[test]
    fn test_lua_cwd() {
        let lua = Lua::new();
        let process_table = create_process_module(&lua).unwrap();
        lua.globals().set("process", process_table).unwrap();

        let code = r#"
            local cwd = process.cwd()
            return type(cwd) == "string" and #cwd > 0
        "#;
        let result: bool = lua.load(code).eval().unwrap();
        assert!(result);
    }

    #[test]
    fn test_lua_env() {
        let lua = Lua::new();
        let process_table = create_process_module(&lua).unwrap();
        lua.globals().set("process", process_table).unwrap();

        let code = r#"
            process.env.HYPE_LUA_TEST = "test_value"
            return process.env.HYPE_LUA_TEST
        "#;
        let result: String = lua.load(code).eval().unwrap();
        assert_eq!(result, "test_value");
    }

    #[test]
    fn test_lua_getenv_setenv() {
        let lua = Lua::new();
        let process_table = create_process_module(&lua).unwrap();
        lua.globals().set("process", process_table).unwrap();

        let code = r#"
            process.setenv("HYPE_TEST_KEY", "test_val")
            return process.getenv("HYPE_TEST_KEY")
        "#;
        let result: String = lua.load(code).eval().unwrap();
        assert_eq!(result, "test_val");
    }

    #[test]
    fn test_lua_platform_arch() {
        let lua = Lua::new();
        let process_table = create_process_module(&lua).unwrap();
        lua.globals().set("process", process_table).unwrap();

        let code = r#"
            return type(process.platform) == "string" and type(process.arch) == "string"
        "#;
        let result: bool = lua.load(code).eval().unwrap();
        assert!(result);
    }

    #[test]
    fn test_lua_pid() {
        let lua = Lua::new();
        let process_table = create_process_module(&lua).unwrap();
        lua.globals().set("process", process_table).unwrap();

        let code = r#"
            return type(process.pid) == "number" and process.pid > 0
        "#;
        let result: bool = lua.load(code).eval().unwrap();
        assert!(result);
    }
}
