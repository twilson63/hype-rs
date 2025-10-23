use mlua::prelude::*;
use std::env;
use crate::error::Result;

pub fn setup_environment(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    
    let env_table = lua.create_table()?;
    
    for (key, value) in env::vars() {
        env_table.set(key, value)?;
    }
    
    globals.set("env", env_table)?;
    
    Ok(())
}
