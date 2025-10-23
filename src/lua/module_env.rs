use mlua::{Lua, Table};
use std::path::Path;

use crate::error::Result;

/// Module environment setup for isolated Lua module execution
pub struct ModuleEnvironment;

impl ModuleEnvironment {
    /// Creates a new module environment with isolated globals and module context
    ///
    /// # Arguments
    /// * `lua` - The Lua runtime instance
    /// * `module_path` - Path to the module file being loaded
    ///
    /// # Returns
    /// A Lua table containing the isolated environment for the module
    ///
    /// # Example
    /// ```ignore
    /// let env = create_module_env(&lua, Path::new("/path/to/module.lua"))?;
    /// lua.load("-- module code --").set_environment(env)?;
    /// ```
    pub fn new() -> Self {
        Self
    }
}

impl Default for ModuleEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates an isolated Lua environment for a module with standard globals and module context
///
/// # Arguments
/// * `lua` - The Lua runtime instance
/// * `module_path` - Absolute or relative path to the module file
///
/// # Returns
/// A Lua table containing the isolated environment with __dirname, __filename, and module table
///
/// # Example
/// ```ignore
/// let env = create_module_env(&lua, Path::new("./modules/utils.lua"))?;
/// lua.load(module_code).set_environment(env)?;
/// ```
pub fn create_module_env<'lua>(lua: &'lua Lua, module_path: &Path) -> Result<Table<'lua>> {
    let env: Table = lua.create_table()?;

    copy_standard_globals(lua, &env)?;
    setup_module_table(lua, &env, module_path)?;

    Ok(env)
}

/// Copies standard Lua globals to the module environment
///
/// Copies essential Lua standard library functions to the isolated environment:
/// print, type, pairs, ipairs, tostring, tonumber, rawget, rawset
fn copy_standard_globals<'lua>(lua: &'lua Lua, env: &Table<'lua>) -> Result<()> {
    let globals = lua.globals();

    let standard_functions = vec![
        "print",
        "type",
        "pairs",
        "ipairs",
        "tostring",
        "tonumber",
        "rawget",
        "rawset",
        "assert",
        "error",
        "next",
        "select",
        "unpack",
        "pcall",
        "xpcall",
        "setmetatable",
        "getmetatable",
        "rawlen",
        "rawequal",
    ];

    for func_name in standard_functions {
        if let Ok(func) = globals.get::<_, mlua::Value>(func_name) {
            env.set(func_name, func)?;
        }
    }

    Ok(())
}

/// Sets up the module context within the environment
///
/// Creates __dirname, __filename variables and initializes the module.exports table
fn setup_module_table<'lua>(lua: &'lua Lua, env: &Table<'lua>, module_path: &Path) -> Result<()> {
    let absolute_path = if module_path.is_absolute() {
        module_path.to_path_buf()
    } else {
        std::env::current_dir()?.join(module_path)
    };

    let dirname: String = absolute_path
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "/".to_string());

    let filename: String = absolute_path.to_string_lossy().to_string();

    env.set("__dirname", dirname)?;
    env.set("__filename", filename)?;

    let module_table: Table = lua.create_table()?;
    let exports: Table = lua.create_table()?;
    module_table.set("exports", exports)?;

    env.set("module", module_table)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_environment_creation() {
        let lua = Lua::new();
        let result = create_module_env(&lua, Path::new("test.lua"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_dirname_is_parent_directory() {
        let lua = Lua::new();
        let env = create_module_env(&lua, Path::new("modules/utils.lua")).unwrap();
        let dirname: String = env.get("__dirname").unwrap();

        assert!(!dirname.is_empty());
        assert!(!dirname.ends_with("modules/utils.lua"));
        assert!(dirname.ends_with("modules") || dirname.contains("modules"));
    }

    #[test]
    fn test_filename_is_full_path() {
        let lua = Lua::new();
        let env = create_module_env(&lua, Path::new("test.lua")).unwrap();
        let filename: String = env.get("__filename").unwrap();

        assert!(filename.contains("test.lua"));
    }

    #[test]
    fn test_standard_globals_present() {
        let lua = Lua::new();
        let env = create_module_env(&lua, Path::new("test.lua")).unwrap();

        assert!(env.get::<_, mlua::Function>("print").is_ok());
        assert!(env.get::<_, mlua::Function>("type").is_ok());
        assert!(env.get::<_, mlua::Function>("pairs").is_ok());
        assert!(env.get::<_, mlua::Function>("ipairs").is_ok());
        assert!(env.get::<_, mlua::Function>("tostring").is_ok());
        assert!(env.get::<_, mlua::Function>("tonumber").is_ok());
        assert!(env.get::<_, mlua::Function>("rawget").is_ok());
        assert!(env.get::<_, mlua::Function>("rawset").is_ok());
    }

    #[test]
    fn test_module_table_with_exports() {
        let lua = Lua::new();
        let env = create_module_env(&lua, Path::new("test.lua")).unwrap();

        let module_table: Table = env.get("module").unwrap();
        let exports: Table = module_table.get("exports").unwrap();

        assert_eq!(exports.len().unwrap(), 0);
    }

    #[test]
    fn test_different_paths_create_different_environments() {
        let lua = Lua::new();
        let env1 = create_module_env(&lua, Path::new("path1/module.lua")).unwrap();
        let env2 = create_module_env(&lua, Path::new("path2/module.lua")).unwrap();

        let dirname1: String = env1.get("__dirname").unwrap();
        let dirname2: String = env2.get("__dirname").unwrap();

        assert_ne!(dirname1, dirname2);
    }

    #[test]
    fn test_absolute_path_handling() {
        let lua = Lua::new();
        let abs_path = Path::new("/absolute/path/to/module.lua");
        let env = create_module_env(&lua, abs_path).unwrap();

        let dirname: String = env.get("__dirname").unwrap();
        let filename: String = env.get("__filename").unwrap();

        assert_eq!(dirname, "/absolute/path/to");
        assert_eq!(filename, "/absolute/path/to/module.lua");
    }

    #[test]
    fn test_module_environment_isolation() {
        let lua = Lua::new();
        let globals = lua.globals();
        globals.set("global_var", "global_value").unwrap();

        let env = create_module_env(&lua, Path::new("test.lua")).unwrap();

        let result: mlua::Result<String> = env.get("global_var");
        assert!(result.is_err());
    }

    #[test]
    fn test_environment_can_access_module_context() {
        let lua = Lua::new();
        let env = create_module_env(&lua, Path::new("test.lua")).unwrap();

        assert!(env.get::<_, String>("__dirname").is_ok());
        assert!(env.get::<_, String>("__filename").is_ok());
        assert!(env.get::<_, Table>("module").is_ok());
    }

    #[test]
    fn test_module_environment_new() {
        let _env = ModuleEnvironment::new();
    }

    #[test]
    fn test_module_environment_default() {
        let _env = ModuleEnvironment::default();
    }
}
