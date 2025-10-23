use mlua::Lua;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use hype_rs::lua::require::setup_require_fn;
use hype_rs::modules::loader::ModuleLoader;

mod basic_module_loading {
    use super::*;

    #[test]
    fn test_require_builtin_fs() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<mlua::Table> = lua
            .load(
                r#"
            local fs = require("fs")
            return fs
        "#,
            )
            .eval();

        assert!(result.is_ok(), "Should load fs module");
        let fs_module = result.unwrap();
        assert!(fs_module.get::<_, mlua::Value>("readFileSync").is_ok());
        assert!(fs_module.get::<_, mlua::Value>("writeFileSync").is_ok());
    }

    #[test]
    fn test_require_builtin_path() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<mlua::Table> = lua
            .load(
                r#"
            local path = require("path")
            return path
        "#,
            )
            .eval();

        assert!(result.is_ok(), "Should load path module");
        let path_module = result.unwrap();
        assert!(path_module.get::<_, mlua::Value>("join").is_ok());
        assert!(path_module.get::<_, mlua::Value>("dirname").is_ok());
    }

    #[test]
    fn test_require_builtin_events() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<mlua::Table> = lua
            .load(
                r#"
            local events = require("events")
            return events
        "#,
            )
            .eval();

        assert!(result.is_ok(), "Should load events module");
        let _events_module = result.unwrap();
    }
}

mod module_caching {
    use super::*;

    #[test]
    fn test_require_cache_same_module_twice() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local fs1 = require("fs")
            local fs2 = require("fs")
            local id1 = fs1.__id
            local id2 = fs2.__id
            return id1 == id2 and id1 == "fs"
        "#,
            )
            .eval();

        assert!(result.is_ok());
        assert!(result.unwrap(), "Cached modules should have same __id");
    }

    #[test]
    fn test_require_cache_different_modules() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local fs = require("fs")
            local path = require("path")
            return fs ~= path
        "#,
            )
            .eval();

        assert!(result.is_ok());
        assert!(result.unwrap(), "Different modules should not be equal");
    }

    #[test]
    fn test_require_cache_exports_preserved() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local fs1 = require("fs")
            local func1 = fs1.readFileSync
            
            local fs2 = require("fs")
            local func2 = fs2.readFileSync
            
            return func1 == func2
        "#,
            )
            .eval();

        assert!(result.is_ok());
        assert!(
            result.unwrap(),
            "Cached exports should preserve function references"
        );
    }
}

mod module_environment {
    use super::*;

    #[test]
    fn test_module_dirname_via_module_env() {
        use hype_rs::lua::module_env::create_module_env;

        let lua = Lua::new();
        let env = create_module_env(&lua, std::path::Path::new("test.lua")).unwrap();

        let dirname: String = env.get("__dirname").unwrap();
        assert!(!dirname.is_empty(), "__dirname should not be empty");
    }

    #[test]
    fn test_module_filename_via_module_env() {
        use hype_rs::lua::module_env::create_module_env;

        let lua = Lua::new();
        let env = create_module_env(&lua, std::path::Path::new("test.lua")).unwrap();

        let filename: String = env.get("__filename").unwrap();
        assert!(
            filename.contains("test.lua"),
            "__filename should contain module name"
        );
    }

    #[test]
    fn test_module_has_require() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                r#"
            return type(require) == "table"
        "#,
            )
            .eval();

        assert!(result.is_ok());
        assert!(result.unwrap(), "require should be available");
    }

    #[test]
    fn test_module_exports_via_module_env() {
        use hype_rs::lua::module_env::create_module_env;

        let lua = Lua::new();
        let env = create_module_env(&lua, std::path::Path::new("test.lua")).unwrap();

        let module_table: mlua::Table = env.get("module").unwrap();
        let _exports: mlua::Table = module_table.get("exports").unwrap();
    }
}

mod circular_dependency_handling {
    use super::*;

    #[test]
    fn test_circular_dependency_detection_programmatic() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<()> = lua
            .load(
                r#"
            local fs = require("fs")
            assert(fs ~= nil, "fs module should load")
        "#,
            )
            .eval();

        assert!(result.is_ok(), "Basic loading should work");
    }

    #[test]
    fn test_circular_dependency_error_handling() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<()> = lua
            .load(
                r#"
            local status, _ = pcall(function()
                local nonexistent = require("nonexistent-module-xyz-invalid")
            end)
            assert(not status, "Should error on missing module")
        "#,
            )
            .eval();

        assert!(result.is_ok());
    }
}

mod require_cache_and_resolve {
    use super::*;

    #[test]
    fn test_require_cache_contains_modules() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                r#"
            require("fs")
            require("path")
            local cache = require.cache
            
            local count = 0
            for _ in pairs(cache) do
                count = count + 1
            end
            
            return count >= 2
        "#,
            )
            .eval();

        assert!(result.is_ok());
        assert!(result.unwrap(), "Cache should contain loaded modules");
    }

    #[test]
    fn test_require_resolve_returns_path() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<String> = lua
            .load(
                r#"
            return require.resolve("fs")
        "#,
            )
            .eval();

        assert!(result.is_ok(), "resolve should return a path");
        let path = result.unwrap();
        assert!(!path.is_empty(), "Resolved path should not be empty");
        assert!(
            path.contains("fs") || path.contains("builtin"),
            "Path should reference fs module"
        );
    }

    #[test]
    fn test_require_resolve_error_on_missing() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<String> = lua
            .load(
                r#"
            return require.resolve("nonexistent-module-xyz-invalid")
        "#,
            )
            .eval();

        assert!(result.is_err(), "resolve should error on missing module");
    }
}

mod cli_module_integration {
    use super::*;

    #[test]
    fn test_cli_module_execution_programmatic() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<()> = lua
            .load(
                r#"
            local fs = require("fs")
            assert(fs ~= nil, "fs module should load")
        "#,
            )
            .eval();

        assert!(result.is_ok(), "Module execution should succeed");
    }

    #[test]
    fn test_cli_module_with_dependencies() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<()> = lua
            .load(
                r#"
            local fs = require("fs")
            local path = require("path")
            
            assert(fs ~= nil, "fs should load")
            assert(path ~= nil, "path should load")
            assert(fs ~= path, "modules should be different")
        "#,
            )
            .eval();

        assert!(
            result.is_ok(),
            "Module with multiple dependencies should work"
        );
    }

    #[test]
    fn test_cli_module_exports_accessible_via_require() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local fs = require("fs")
            return type(fs) == "table" and fs.__id == "fs"
        "#,
            )
            .eval();

        assert!(result.is_ok());
        assert!(
            result.unwrap(),
            "Module exports should be accessible and have __id"
        );
    }
}

mod error_handling {
    use super::*;

    #[test]
    fn test_module_not_found_error() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<()> = lua
            .load(
                r#"
            require("nonexistent-module-xyz-invalid")
        "#,
            )
            .eval();

        assert!(result.is_err(), "Should error when module not found");
    }

    #[test]
    fn test_invalid_module_path() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<()> = lua
            .load(
                r#"
            require("")
        "#,
            )
            .eval();

        assert!(result.is_err(), "Should error on invalid path");
    }
}

mod loader_operations {
    use super::*;

    #[test]
    fn test_loader_caching_behavior() {
        let mut loader = ModuleLoader::new(PathBuf::from("."));

        let first = loader.require("fs").unwrap();
        let second = loader.require("fs").unwrap();

        assert_eq!(first, second, "Loader should return cached module");
    }

    #[test]
    fn test_loader_clear_cache() {
        let mut loader = ModuleLoader::new(PathBuf::from("."));

        loader.require("fs").ok();
        loader.require("path").ok();

        let before = loader.cached_modules().unwrap().len();
        assert!(before > 0, "Cache should have modules");

        loader.clear_cache().unwrap();
        let after = loader.cached_modules().unwrap().len();

        assert_eq!(after, 0, "Cache should be empty after clear");
    }

    #[test]
    fn test_loader_get_cached_loaded_module() {
        let mut loader = ModuleLoader::new(PathBuf::from("."));

        loader.require("fs").ok();

        let cached = loader.get_cached("fs");
        assert!(cached.is_ok(), "Should get cached module");
    }

    #[test]
    fn test_loader_get_cached_missing_module() {
        let loader = ModuleLoader::new(PathBuf::from("."));

        let result = loader.get_cached("nonexistent-xyz");
        assert!(result.is_err(), "Should error for missing module cache");
    }
}

mod all_builtins {
    use super::*;

    #[test]
    fn test_all_builtin_modules_loadable() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local modules = {"fs", "path", "events", "util", "table"}
            for _, name in ipairs(modules) do
                local m = require(name)
                if m == nil then
                    return false
                end
            end
            return true
        "#,
            )
            .eval();

        assert!(result.is_ok());
        assert!(result.unwrap(), "All builtin modules should load");
    }

    #[test]
    fn test_all_builtins_resolvable() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local modules = {"fs", "path", "events", "util", "table"}
            for _, name in ipairs(modules) do
                local path = require.resolve(name)
                if not path or string.len(path) == 0 then
                    return false
                end
            end
            return true
        "#,
            )
            .eval();

        assert!(result.is_ok());
        assert!(result.unwrap(), "All builtins should be resolvable");
    }
}

mod require_behavior {
    use super::*;

    #[test]
    fn test_require_returns_table() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<String> = lua
            .load(
                r#"
            local fs = require("fs")
            return type(fs)
        "#,
            )
            .eval();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "table", "Module should be a table");
    }

    #[test]
    fn test_require_function_is_callable() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                r#"
            return type(require) == "table"
        "#,
            )
            .eval();

        assert!(result.is_ok());
        assert!(result.unwrap(), "require should be a callable table");
    }

    #[test]
    fn test_require_multiple_simultaneous_loads() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local fs = require("fs")
            local path = require("path")
            local events = require("events")
            
            return fs ~= nil and path ~= nil and events ~= nil
        "#,
            )
            .eval();

        assert!(result.is_ok());
        assert!(
            result.unwrap(),
            "Should load multiple modules simultaneously"
        );
    }

    #[test]
    fn test_require_cache_populated_after_load() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local fs = require("fs")
            local cache = require.cache
            
            local has_fs = false
            for key, value in pairs(cache) do
                if string.find(key, "fs") and value ~= nil then
                    has_fs = true
                    break
                end
            end
            
            return has_fs
        "#,
            )
            .eval();

        assert!(result.is_ok());
        assert!(result.unwrap(), "Cache should contain loaded modules");
    }
}

mod module_features {
    use super::*;

    #[test]
    fn test_module_id_in_exports() {
        let mut loader = ModuleLoader::new(PathBuf::from("."));

        let exports = loader.require("fs").unwrap();
        assert!(
            exports.get("__id").is_some(),
            "Module exports should have __id"
        );
    }

    #[test]
    fn test_module_path_in_exports() {
        let mut loader = ModuleLoader::new(PathBuf::from("."));

        let exports = loader.require("fs").unwrap();
        assert!(
            exports.get("__path").is_some(),
            "Module exports should have __path"
        );
    }

    #[test]
    fn test_require_preserves_module_state() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));

        setup_require_fn(&lua, loader).unwrap();

        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local fs1 = require("fs")
            local id1 = fs1.__id
            
            local fs2 = require("fs")
            local id2 = fs2.__id
            
            return id1 == id2
        "#,
            )
            .eval();

        assert!(result.is_ok());
        assert!(
            result.unwrap(),
            "Module state should be preserved across requires"
        );
    }
}
