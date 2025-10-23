use mlua::prelude::*;
use std::collections::HashMap;
use std::env;

pub struct EnvironmentModule;

impl EnvironmentModule {
    pub fn register(lua: &Lua) -> LuaResult<()> {
        let globals = lua.globals();

        let env_table = lua.create_table()?;

        for (key, value) in env::vars() {
            env_table.set(key, value)?;
        }

        globals.set("env", env_table)?;
        Ok(())
    }

    pub fn get_var(name: &str) -> Option<String> {
        env::var(name).ok()
    }

    pub fn set_var(name: &str, value: &str) {
        env::set_var(name, value);
    }

    pub fn current_dir() -> String {
        env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| String::from("."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_access() {
        let lua = Lua::new();
        env::set_var("TEST_VAR", "test_value");

        EnvironmentModule::register(&lua).unwrap();

        let result: String = lua.load("return env.TEST_VAR").eval().unwrap();
        assert_eq!(result, "test_value");
    }

    #[test]
    fn test_current_dir() {
        let current = EnvironmentModule::current_dir();
        assert!(!current.is_empty());
    }
}
