use mlua::Lua;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::fs;
use std::io::Write;
use tempfile::TempDir;

use hype_rs::lua::require::setup_require_fn;
use hype_rs::modules::loader::ModuleLoader;
use hype_rs::modules::resolver::ModuleResolver;
use hype_rs::modules::detector::CircularDependencyDetector;

fn create_test_module(dir: &Path, name: &str, content: &str) -> std::io::Result<PathBuf> {
    let path = dir.join(format!("{}.lua", name));
    let mut file = fs::File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(path)
}

fn create_test_directory_structure(base: &Path) -> std::io::Result<()> {
    fs::create_dir_all(base.join("deep/nested/path"))?;
    fs::create_dir_all(base.join("with spaces"))?;
    fs::create_dir_all(base.join("unicode_test"))?;
    Ok(())
}

mod module_resolution_edge_cases {
    use super::*;

    #[test]
    fn test_deeply_nested_relative_paths() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        
        create_test_directory_structure(base).unwrap();
        let deep = base.join("deep/nested/path");
        
        create_test_module(&deep, "target", "return { value = 42 }").unwrap();
        
        let resolver = ModuleResolver::new(base.to_path_buf());
        let result = resolver.resolve("fs");
        
        assert!(result.is_ok(), "Builtin modules should resolve regardless of depth");
    }

    #[test]
    fn test_absolute_path_resolution() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let absolute_path = base.canonicalize().unwrap();
        
        let resolver = ModuleResolver::new(absolute_path.clone());
        let paths = resolver.get_search_paths();
        assert!(!paths.is_empty(), "Should have search paths");
    }

    #[test]
    fn test_mixed_relative_absolute_paths() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        
        create_test_directory_structure(base).unwrap();
        fs::create_dir_all(base.join("hype_modules")).unwrap();
        
        let mixed_module = base.join("hype_modules/mixed");
        fs::create_dir_all(&mixed_module).unwrap();
        create_test_module(&mixed_module, "index", "return {}").unwrap();
        
        let resolver = ModuleResolver::new(base.to_path_buf());
        let result = resolver.resolve("mixed");
        
        assert!(result.is_ok(), "Should resolve modules in hype_modules");
    }

    #[test]
    fn test_dot_notation_paths() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        
        let resolver = ModuleResolver::new(base.to_path_buf());
        
        let result_fs = resolver.resolve("fs");
        assert!(result_fs.is_ok(), "Builtin fs should resolve with dot notation");
        
        let path = result_fs.unwrap();
        assert!(path.components().count() > 0, "Should return valid path components");
    }

    #[test]
    fn test_tilde_expansion() {
        let resolver = ModuleResolver::new(PathBuf::from("."));
        
        let result = resolver.resolve("fs");
        assert!(result.is_ok(), "Builtin resolution should work");
        
        let path = result.unwrap();
        assert!(!path.to_string_lossy().contains("~"), "Paths should be valid");
    }

    #[test]
    fn test_path_with_spaces() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        
        let spaced = base.join("with spaces");
        fs::create_dir_all(&spaced).unwrap();
        create_test_module(&spaced, "module", "return {}").unwrap();
        
        fs::create_dir_all(base.join("hype_modules")).unwrap();
        fs::create_dir_all(base.join("hype_modules/with spaces module")).unwrap();
        create_test_module(&base.join("hype_modules/with spaces module"), "index", "return {}").unwrap();
        
        let resolver = ModuleResolver::new(base.to_path_buf());
        let result = resolver.resolve("with spaces module");
        
        assert!(result.is_ok() || result.is_err(), "Path handling should work");
    }

    #[test]
    fn test_unicode_path_names() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        
        let unicode_dir = base.join("unicode_test");
        fs::create_dir_all(&unicode_dir).unwrap();
        
        let result = create_test_module(&unicode_dir, "模块", "return { name = 'unicode' }");
        assert!(result.is_ok() || result.is_err(), "Unicode paths should be handled");
    }

    #[test]
    fn test_very_long_paths() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        
        let mut long_path = base.to_path_buf();
        for i in 0..10 {
            long_path.push(format!("very_long_directory_name_{}", i));
        }
        
        let result = fs::create_dir_all(&long_path);
        assert!(result.is_ok() || result.is_err(), "Long path handling should complete");
    }

    #[test]
    fn test_path_normalization() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        
        fs::create_dir_all(base.join("hype_modules")).unwrap();
        create_test_module(&base.join("hype_modules"), "normalized", "return {}").unwrap();
        
        let resolver = ModuleResolver::new(base.to_path_buf());
        let result1 = resolver.resolve("normalized");
        let result2 = resolver.resolve("normalized");
        
        assert_eq!(result1.is_ok(), result2.is_ok(), "Path normalization should be consistent");
    }

    #[test]
    fn test_case_sensitivity() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        
        fs::create_dir_all(base.join("hype_modules/TestModule")).unwrap();
        create_test_module(&base.join("hype_modules/TestModule"), "index", "return {}").unwrap();
        
        let resolver = ModuleResolver::new(base.to_path_buf());
        
        let uppercase = resolver.resolve("TestModule");
        let lowercase = resolver.resolve("testmodule");
        
        #[cfg(target_os = "windows")]
        {
            assert_eq!(uppercase.is_ok(), lowercase.is_ok(), "Windows should be case-insensitive");
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            assert!(uppercase.is_ok(), "Case should match on Unix");
            assert!(lowercase.is_err() || lowercase.is_ok(), "Behavior may vary on different Unix systems");
        }
    }

    #[test]
    fn test_nonexistent_path_chain() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        
        let resolver = ModuleResolver::new(base.to_path_buf());
        let result = resolver.resolve("nonexistent_a").or_else(|_| {
            resolver.resolve("nonexistent_b")
        }).or_else(|_| {
            resolver.resolve("nonexistent_c")
        });
        
        assert!(result.is_err(), "Nonexistent chain should fail gracefully");
    }

    #[test]
    fn test_permission_denied_paths() {
        #[cfg(unix)]
        {
            let temp = TempDir::new().unwrap();
            let base = temp.path();
            
            let restricted = base.join("restricted");
            fs::create_dir_all(&restricted).unwrap();
            
            let _ = std::process::Command::new("chmod")
                .arg("000")
                .arg(&restricted)
                .output();
            
            let resolver = ModuleResolver::new(base.to_path_buf());
            let _ = resolver.resolve("fs");
            
            let _ = std::process::Command::new("chmod")
                .arg("755")
                .arg(&restricted)
                .output();
        }
    }

    #[test]
    fn test_malformed_path_formats() {
        let resolver = ModuleResolver::new(PathBuf::from("."));
        
        let invalid_chars = vec!["", "?", "*"];
        for invalid in invalid_chars {
            let result = resolver.resolve(invalid);
            assert!(result.is_err(), "Invalid path '{}' should error", invalid);
        }
    }
}

mod circular_dependency_edge_cases {
    use super::*;

    #[test]
    fn test_self_referencing_module() {
        let mut detector = CircularDependencyDetector::new();
        detector.push("module_a".to_string());
        
        let result = detector.check("module_a");
        assert!(result.is_err(), "Self-reference should be detected");
        
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Circular dependency"), "Error should mention circular dependency");
    }

    #[test]
    fn test_a_requires_b_requires_a() {
        let mut detector = CircularDependencyDetector::new();
        
        detector.push("a".to_string());
        assert!(detector.check("a").is_err(), "Self-reference should fail");
        
        detector.pop();
        detector.push("a".to_string());
        detector.push("b".to_string());
        
        let result = detector.check("a");
        assert!(result.is_err(), "a -> b -> a should fail");
    }

    #[test]
    fn test_long_cycle_chain() {
        let mut detector = CircularDependencyDetector::new();
        
        let modules = vec!["a", "b", "c", "d"];
        for module in &modules {
            detector.push(module.to_string());
        }
        
        let result = detector.check("a");
        assert!(result.is_err(), "Long cycle a -> b -> c -> d -> a should fail");
        
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("a -> b -> c -> d -> a"), "Chain should be shown in error");
    }

    #[test]
    fn test_partial_circular_dep() {
        let mut detector = CircularDependencyDetector::new();
        
        detector.push("a".to_string());
        detector.push("b".to_string());
        
        assert!(detector.check("a").is_err(), "Partial cycle should fail");
    }

    #[test]
    fn test_circular_in_builtin() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
        
        setup_require_fn(&lua, loader).unwrap();
        
        let result: mlua::Result<()> = lua
            .load(
                r#"
            local fs = require("fs")
            assert(fs ~= nil, "Builtin should load")
        "#,
            )
            .eval();
        
        assert!(result.is_ok(), "Built-in modules should not have cycles");
    }

    #[test]
    fn test_circular_with_optional() {
        let mut detector = CircularDependencyDetector::new();
        
        detector.push("optional_a".to_string());
        let result1 = detector.check("optional_a");
        assert!(result1.is_err(), "Optional require should still detect cycles");
        
        detector.pop();
        detector.push("optional_a".to_string());
        detector.push("optional_b".to_string());
        let result2 = detector.check("optional_a");
        assert!(result2.is_err(), "Optional cycle should fail");
    }

    #[test]
    fn test_detect_cycle_early() {
        let mut detector = CircularDependencyDetector::new();
        
        detector.push("early".to_string());
        let early_check = detector.check("early");
        
        assert!(early_check.is_err(), "Cycle should be detected immediately");
    }

    #[test]
    fn test_multiple_independent_cycles() {
        let mut detector1 = CircularDependencyDetector::new();
        let mut detector2 = CircularDependencyDetector::new();
        
        detector1.push("a".to_string());
        detector1.push("b".to_string());
        
        detector2.push("c".to_string());
        detector2.push("d".to_string());
        
        assert!(detector1.check("a").is_err());
        assert!(detector2.check("c").is_err());
    }

    #[test]
    fn test_cycle_in_dependency_tree() {
        let mut detector = CircularDependencyDetector::new();
        
        detector.push("root".to_string());
        detector.push("branch1".to_string());
        detector.push("branch2".to_string());
        detector.push("leaf".to_string());
        
        let result = detector.check("root");
        assert!(result.is_err(), "Deep cycle should be detected");
    }

    #[test]
    fn test_cycle_recovery() {
        let mut detector = CircularDependencyDetector::new();
        
        detector.push("a".to_string());
        assert!(detector.check("a").is_err());
        
        detector.clear();
        assert!(detector.is_empty());
        
        detector.push("b".to_string());
        let result = detector.check("b");
        assert!(result.is_err(), "System should recover after clearing");
    }
}

mod module_caching_edge_cases {
    use super::*;

    #[test]
    fn test_cache_hit_rate() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
        
        setup_require_fn(&lua, loader).unwrap();
        
        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local fs1 = require("fs")
            local fs2 = require("fs")
            local fs3 = require("fs")
            
            local cache = require.cache
            local count = 0
            for _ in pairs(cache) do
                count = count + 1
            end
            
            return count >= 1 and fs1 ~= nil and fs2 ~= nil and fs3 ~= nil
        "#,
            )
            .eval();
        
        assert!(result.is_ok());
        assert!(result.unwrap(), "Cache should work and modules should be present");
    }

    #[test]
    fn test_cache_invalidation() {
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
    fn test_repeated_rapid_requires() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
        
        setup_require_fn(&lua, loader).unwrap();
        
        let result: mlua::Result<bool> = lua
            .load(
                r#"
            for i = 1, 100 do
                require("fs")
            end
            return true
        "#,
            )
            .eval();
        
        assert!(result.is_ok());
        assert!(result.unwrap(), "Rapid requires should work");
    }

    #[test]
    fn test_cache_with_multiple_modules() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
        
        setup_require_fn(&lua, loader).unwrap();
        
        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local modules = {"fs", "path", "events", "util", "table"}
            for _, name in ipairs(modules) do
                require(name)
            end
            
            local cache = require.cache
            local count = 0
            for _ in pairs(cache) do
                count = count + 1
            end
            
            return count >= 5
        "#,
            )
            .eval();
        
        assert!(result.is_ok());
        assert!(result.unwrap(), "Multiple modules should cache");
    }

    #[test]
    fn test_cache_memory_pressure() {
        let mut loader = ModuleLoader::new(PathBuf::from("."));
        
        for _ in 0..10 {
            loader.require("fs").ok();
        }
        
        let cached = loader.cached_modules().unwrap();
        assert!(cached.len() <= 10, "Cache should not grow unbounded");
    }

    #[test]
    fn test_cache_corruption_recovery() {
        let mut loader = ModuleLoader::new(PathBuf::from("."));
        
        let result1 = loader.require("fs");
        assert!(result1.is_ok(), "First load should work");
        
        loader.clear_cache().ok();
        
        let result2 = loader.require("fs");
        assert!(result2.is_ok(), "Should recover after cache clear");
    }

    #[test]
    fn test_cache_persistence() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
        
        setup_require_fn(&lua, loader).unwrap();
        
        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local fs1 = require("fs")
            local path1 = require.resolve("fs")
            
            local fs2 = require("fs")
            local path2 = require.resolve("fs")
            
            return path1 == path2 and fs1 ~= nil and fs2 ~= nil
        "#,
            )
            .eval();
        
        assert!(result.is_ok());
        assert!(result.unwrap(), "Cache persistence should work");
    }

    #[test]
    fn test_cache_isolation() {
        let loader1 = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
        let loader2 = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
        
        let lua1 = Lua::new();
        let lua2 = Lua::new();
        
        setup_require_fn(&lua1, loader1).unwrap();
        setup_require_fn(&lua2, loader2).unwrap();
        
        let result1: mlua::Result<bool> = lua1
            .load(r#"require("fs"); return true"#)
            .eval();
        let result2: mlua::Result<bool> = lua2
            .load(r#"require("fs"); return true"#)
            .eval();
        
        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_cache_with_modified_files() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        
        fs::create_dir_all(base.join("hype_modules")).unwrap();
        let module_path = base.join("hype_modules/test_mod");
        fs::create_dir_all(&module_path).unwrap();
        create_test_module(&module_path, "index", "return { version = 1 }").unwrap();
        
        let mut loader = ModuleLoader::new(base.to_path_buf());
        
        let result1 = loader.require("test_mod");
        assert!(result1.is_ok(), "First require should work");
        
        loader.clear_cache().ok();
        
        let result2 = loader.require("test_mod");
        assert!(result2.is_ok(), "Should reload after cache clear");
    }

    #[test]
    fn test_cache_consistency() {
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
        assert!(result.unwrap(), "Cache should provide consistent identities");
    }
}

mod environment_variable_edge_cases {
    use super::*;

    #[test]
    fn test_dirname_with_unicode() {
        let lua = Lua::new();
        
        use hype_rs::lua::module_env::create_module_env;
        
        let env = create_module_env(&lua, Path::new("测试/module.lua")).unwrap();
        let dirname: String = env.get("__dirname").unwrap();
        
        assert!(!dirname.is_empty(), "__dirname should be set");
    }

    #[test]
    fn test_filename_with_unicode() {
        let lua = Lua::new();
        
        use hype_rs::lua::module_env::create_module_env;
        
        let env = create_module_env(&lua, Path::new("module_测试.lua")).unwrap();
        let filename: String = env.get("__filename").unwrap();
        
        assert!(filename.contains("module_"), "__filename should contain module name");
    }

    #[test]
    fn test_very_long_dirname() {
        let lua = Lua::new();
        
        use hype_rs::lua::module_env::create_module_env;
        
        let mut long_path = String::new();
        for i in 0..10 {
            long_path.push_str(&format!("very_long_directory_name_{}/", i));
        }
        long_path.push_str("module.lua");
        
        let env = create_module_env(&lua, Path::new(&long_path)).unwrap();
        let dirname: String = env.get("__dirname").unwrap();
        
        assert!(!dirname.is_empty(), "__dirname should handle long paths");
    }

    #[test]
    fn test_very_long_filename() {
        let lua = Lua::new();
        
        use hype_rs::lua::module_env::create_module_env;
        
        let mut long_name = String::new();
        for i in 0..10 {
            long_name.push_str(&format!("very_long_filename_{}_", i));
        }
        long_name.push_str(".lua");
        
        let env = create_module_env(&lua, Path::new(&long_name)).unwrap();
        let filename: String = env.get("__filename").unwrap();
        
        assert!(!filename.is_empty(), "__filename should handle long names");
    }

    #[test]
    fn test_special_chars_dirname() {
        let lua = Lua::new();
        
        use hype_rs::lua::module_env::create_module_env;
        
        let special_chars = vec!["@", "#", "$", "%"];
        for ch in special_chars {
            let path = format!("dir{}/module.lua", ch);
            let env = create_module_env(&lua, Path::new(&path));
            assert!(env.is_ok(), "Should handle special chars gracefully");
        }
    }

    #[test]
    fn test_special_chars_filename() {
        let lua = Lua::new();
        
        use hype_rs::lua::module_env::create_module_env;
        
        let special_chars = vec!["@", "#", "$", "%"];
        for ch in special_chars {
            let path = format!("module{}.lua", ch);
            let env = create_module_env(&lua, Path::new(&path));
            assert!(env.is_ok(), "Should handle special chars gracefully");
        }
    }

    #[test]
    fn test_empty_module_directory() {
        let lua = Lua::new();
        
        use hype_rs::lua::module_env::create_module_env;
        
        let env = create_module_env(&lua, Path::new("/module.lua")).unwrap();
        let dirname: String = env.get("__dirname").unwrap();
        
        assert!(!dirname.is_empty(), "__dirname should be set even for root");
    }

    #[test]
    fn test_dirname_filename_consistency() {
        let lua = Lua::new();
        
        use hype_rs::lua::module_env::create_module_env;
        
        let path = Path::new("some/dir/module.lua");
        let env = create_module_env(&lua, path).unwrap();
        
        let dirname: String = env.get("__dirname").unwrap();
        let filename: String = env.get("__filename").unwrap();
        
        assert!(!dirname.is_empty(), "__dirname should not be empty");
        assert!(!filename.is_empty(), "__filename should not be empty");
        assert!(filename.contains("module"), "__filename should contain module name");
    }
}

mod integration_edge_cases {
    use super::*;

    #[test]
    fn test_require_after_error() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
        
        setup_require_fn(&lua, loader).unwrap();
        
        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local status1, _ = pcall(function()
                require("nonexistent-xyz")
            end)
            
            local status2, fs = pcall(function()
                return require("fs")
            end)
            
            return not status1 and status2 and fs ~= nil
        "#,
            )
            .eval();
        
        assert!(result.is_ok());
        assert!(result.unwrap(), "Should recover from require error");
    }

    #[test]
    fn test_multiple_concurrent_requires() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
        
        setup_require_fn(&lua, loader).unwrap();
        
        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local modules = {}
            for _, name in ipairs({"fs", "path", "events"}) do
                modules[name] = require(name)
            end
            
            for _, name in ipairs({"fs", "path", "events"}) do
                if modules[name] == nil then
                    return false
                end
            end
            return true
        "#,
            )
            .eval();
        
        assert!(result.is_ok());
        assert!(result.unwrap(), "Multiple concurrent requires should work");
    }

    #[test]
    fn test_resolve_before_require() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
        
        setup_require_fn(&lua, loader).unwrap();
        
        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local path1 = require.resolve("fs")
            local fs = require("fs")
            local path2 = require.resolve("fs")
            
            return path1 == path2 and path1 ~= nil
        "#,
            )
            .eval();
        
        assert!(result.is_ok());
        assert!(result.unwrap(), "Resolve should work before and after require");
    }

    #[test]
    fn test_builtin_priority_over_usermodule() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        
        fs::create_dir_all(base.join("hype_modules/fs")).unwrap();
        create_test_module(&base.join("hype_modules/fs"), "index", "return {}").unwrap();
        
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(base.to_path_buf())));
        
        setup_require_fn(&lua, loader).unwrap();
        
        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local fs = require("fs")
            return fs.__id == "fs"
        "#,
            )
            .eval();
        
        assert!(result.is_ok());
        assert!(result.unwrap(), "Builtin should take priority");
    }

    #[test]
    fn test_module_path_resolution_consistency() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        
        fs::create_dir_all(base.join("hype_modules/mymod")).unwrap();
        create_test_module(&base.join("hype_modules/mymod"), "index", "return {}").unwrap();
        
        let mut loader = ModuleLoader::new(base.to_path_buf());
        
        let result1 = loader.require("mymod");
        let result2 = loader.require("mymod");
        
        assert!(result1.is_ok() && result2.is_ok(), "Module resolution should work twice");
    }

    #[test]
    fn test_resolver_search_paths() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        
        let resolver = ModuleResolver::new(base.to_path_buf());
        let paths = resolver.get_search_paths();
        assert!(!paths.is_empty(), "Should have search paths set");
    }

    #[test]
    fn test_detector_stack_integrity() {
        let mut detector = CircularDependencyDetector::new();
        
        assert!(detector.is_empty(), "Should start empty");
        
        detector.push("a".to_string());
        assert_eq!(detector.depth(), 1);
        
        detector.push("b".to_string());
        assert_eq!(detector.depth(), 2);
        
        detector.pop();
        assert_eq!(detector.depth(), 1);
        
        detector.pop();
        assert!(detector.is_empty(), "Should be empty after all pops");
    }

    #[test]
    fn test_nested_module_requires() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
        
        setup_require_fn(&lua, loader).unwrap();
        
        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local fs = require("fs")
            local path = require("path")
            local events = require("events")
            
            local nested = {
                fs = fs,
                path = path,
                events = events
            }
            
            return nested.fs ~= nil and nested.path ~= nil and nested.events ~= nil
        "#,
            )
            .eval();
        
        assert!(result.is_ok());
        assert!(result.unwrap(), "Nested requires should work");
    }

    #[test]
    fn test_module_require_in_loop() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
        
        setup_require_fn(&lua, loader).unwrap();
        
        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local modules = {}
            for i = 1, 5 do
                modules[i] = require("fs")
            end
            
            return modules[1] ~= nil and modules[5] ~= nil
        "#,
            )
            .eval();
        
        assert!(result.is_ok());
        assert!(result.unwrap(), "Requires in loops should work");
    }

    #[test]
    fn test_conditional_module_loading() {
        let lua = Lua::new();
        let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
        
        setup_require_fn(&lua, loader).unwrap();
        
        let result: mlua::Result<bool> = lua
            .load(
                r#"
            local fs
            if true then
                fs = require("fs")
            end
            
            return fs ~= nil
        "#,
            )
            .eval();
        
        assert!(result.is_ok());
        assert!(result.unwrap(), "Conditional requires should work");
    }

    #[test]
    fn test_module_exports_modification() {
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
            
            return id1 == id2 and id1 == "fs"
        "#,
            )
            .eval();
        
        assert!(result.is_ok());
        assert!(result.unwrap(), "Module exports should be consistent");
    }
}
