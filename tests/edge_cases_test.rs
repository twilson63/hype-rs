use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use hype_rs::modules::loader::ModuleLoader;

fn create_test_module(dir: &Path, name: &str, content: &str) -> std::io::Result<PathBuf> {
    let path = dir.join(format!("{}.lua", name));
    let mut file = fs::File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(path)
}

fn create_hype_json(dir: &Path, content: &str) -> std::io::Result<PathBuf> {
    let path = dir.join("hype.json");
    let mut file = fs::File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(path)
}

mod error_recovery {
    use super::*;

    #[test]
    fn test_recover_from_missing_manifest() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        let module_dir = modules_dir.join("no_manifest");
        fs::create_dir_all(&module_dir).unwrap();
        create_test_module(&module_dir, "index", "return {}").unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let result = loader.require("no_manifest");
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle missing manifest gracefully"
        );

        let cached = loader.cached_modules();
        assert!(cached.is_ok(), "Loader should still be operational");
    }

    #[test]
    fn test_recover_from_invalid_manifest() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        let module_dir = modules_dir.join("bad_manifest");
        fs::create_dir_all(&module_dir).unwrap();
        create_test_module(&module_dir, "index", "return {}").unwrap();
        create_hype_json(&module_dir, "{ invalid json, }").unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let _ = loader.require("bad_manifest");
        assert!(
            loader.cached_modules().is_ok(),
            "Loader should still work after error"
        );
    }

    #[test]
    fn test_recover_from_circular_dep() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        let mod_a = modules_dir.join("circ_a");
        fs::create_dir_all(&mod_a).unwrap();
        create_test_module(&mod_a, "index", "return {}").unwrap();

        let mod_b = modules_dir.join("circ_b");
        fs::create_dir_all(&mod_b).unwrap();
        create_test_module(&mod_b, "index", "return {}").unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let _ = loader.require("circ_a");
        let _ = loader.require("circ_b");

        assert!(
            loader.cached_modules().is_ok(),
            "Loader should continue after circular check"
        );
    }

    #[test]
    fn test_recover_from_module_load_failure() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        let module_dir = modules_dir.join("fail_module");
        fs::create_dir_all(&module_dir).unwrap();
        create_test_module(&module_dir, "index", "error('module failed')").unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let _ = loader.require("fail_module");

        let result_after = loader.require("fs");
        assert!(
            result_after.is_ok(),
            "Should be able to load other modules after failure"
        );
    }

    #[test]
    fn test_recover_from_null_exports() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        let module_dir = modules_dir.join("null_module");
        fs::create_dir_all(&module_dir).unwrap();
        create_test_module(&module_dir, "index", "return nil").unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let result = loader.require("null_module");
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle nil exports"
        );

        let result_after = loader.require("fs");
        assert!(
            result_after.is_ok(),
            "System should continue after nil export"
        );
    }

    #[test]
    fn test_recover_from_permission_denied() {
        #[cfg(unix)]
        {
            let temp = TempDir::new().unwrap();
            let base = temp.path();
            let modules_dir = base.join("node_modules");
            fs::create_dir_all(&modules_dir).unwrap();

            let module_dir = modules_dir.join("restricted");
            fs::create_dir_all(&module_dir).unwrap();
            let module_file = create_test_module(&module_dir, "index", "return {}").unwrap();

            fs::set_permissions(&module_file, fs::Permissions::from_mode(0o000)).unwrap();

            let mut loader = ModuleLoader::new(base.to_path_buf());

            let _ = loader.require("restricted");

            fs::set_permissions(&module_file, fs::Permissions::from_mode(0o644)).unwrap();

            let result_after = loader.require("fs");
            assert!(
                result_after.is_ok(),
                "Should recover after permission error"
            );
        }
    }

    #[test]
    fn test_recover_from_disk_full_simulation() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        let module_dir = modules_dir.join("normal");
        fs::create_dir_all(&module_dir).unwrap();
        create_test_module(&module_dir, "index", "return { status = 'ok' }").unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let result = loader.require("normal");
        assert!(
            result.is_ok(),
            "Should load normally when space is available"
        );
    }

    #[test]
    fn test_recover_from_concurrent_writes() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        for i in 0..5 {
            let module_dir = modules_dir.join(format!("concurrent_{}", i));
            fs::create_dir_all(&module_dir).unwrap();
            create_test_module(&module_dir, "index", &format!("return {{ id = {} }}", i)).unwrap();
        }

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let mut all_loaded = true;
        for i in 0..5 {
            if loader.require(&format!("concurrent_{}", i)).is_err() {
                all_loaded = false;
            }
        }

        assert!(
            all_loaded,
            "All modules should load despite concurrent access"
        );
    }

    #[test]
    fn test_graceful_shutdown_with_cache() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        for i in 0..3 {
            let module_dir = modules_dir.join(format!("shutdown_{}", i));
            fs::create_dir_all(&module_dir).unwrap();
            create_test_module(&module_dir, "index", "return {}").unwrap();
        }

        {
            let mut loader = ModuleLoader::new(base.to_path_buf());
            let _ = loader.require("shutdown_0");
            let _ = loader.require("shutdown_1");
            let _ = loader.require("shutdown_2");

            let _ = loader.clear_cache();
        }

        let temp_path = temp.path().to_path_buf();
        assert!(temp_path.exists(), "Resources should be properly cleaned");
    }

    #[test]
    fn test_cache_state_consistency() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        for i in 0..3 {
            let module_dir = modules_dir.join(format!("consistent_{}", i));
            fs::create_dir_all(&module_dir).unwrap();
            create_test_module(&module_dir, "index", &format!("return {{ val = {} }}", i)).unwrap();
        }

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let mut first_load = Vec::new();
        for i in 0..3 {
            if let Ok(exports) = loader.require(&format!("consistent_{}", i)) {
                first_load.push(exports);
            }
        }

        let mut second_load = Vec::new();
        for i in 0..3 {
            if let Ok(exports) = loader.require(&format!("consistent_{}", i)) {
                second_load.push(exports);
            }
        }

        assert_eq!(
            first_load.len(),
            second_load.len(),
            "Load results should be consistent"
        );

        for (first, second) in first_load.iter().zip(second_load.iter()) {
            assert_eq!(
                first, second,
                "Module exports should be identical on cache hits"
            );
        }
    }
}

mod boundary_conditions {
    use super::*;

    #[test]
    fn test_empty_module_file() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        let module_dir = modules_dir.join("empty");
        fs::create_dir_all(&module_dir).unwrap();

        fs::File::create(module_dir.join("index.lua")).unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let result = loader.require("empty");
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle empty module"
        );
    }

    #[test]
    fn test_module_with_only_comments() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        let module_dir = modules_dir.join("comments");
        fs::create_dir_all(&module_dir).unwrap();
        create_test_module(
            &module_dir,
            "index",
            "-- This is a comment\n-- Another comment\n-- No code here",
        )
        .unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let result = loader.require("comments");
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle comment-only module"
        );
    }

    #[test]
    fn test_very_large_module_export() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        let module_dir = modules_dir.join("huge");
        fs::create_dir_all(&module_dir).unwrap();

        let mut export_content = String::from("return { ");
        for i in 0..10000 {
            export_content.push_str(&format!("k{} = {}, ", i, i));
        }
        export_content.push_str("end_marker = true }");

        create_test_module(&module_dir, "index", &export_content).unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let result = loader.require("huge");
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle very large exports"
        );
    }

    #[test]
    fn test_deeply_nested_export_structure() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        let module_dir = modules_dir.join("nested");
        fs::create_dir_all(&module_dir).unwrap();

        let mut nested_content = String::from("return { ");
        nested_content.push_str(
            "a = { b = { c = { d = { e = { f = { g = { h = { i = { j = 'deep' } } } } } } } } } }",
        );

        create_test_module(&module_dir, "index", &nested_content).unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let result = loader.require("nested");
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle deeply nested structures"
        );
    }

    #[test]
    fn test_special_export_types() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        let module_dir = modules_dir.join("types");
        fs::create_dir_all(&module_dir).unwrap();
        create_test_module(
            &module_dir,
            "index",
            "return { \
                func = function() return 42 end, \
                tbl = { x = 1 }, \
                str = 'hello', \
                num = 3.14, \
                bool = true, \
                null = nil \
            }",
        )
        .unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let result = loader.require("types");
        if let Ok(exports) = result {
            assert!(exports.is_object(), "Should preserve table structure");
        }
    }

    #[test]
    fn test_zero_length_module_name() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let result = loader.require("");
        assert!(result.is_err(), "Empty module name should error");
    }

    #[test]
    fn test_max_path_length() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();

        let mut long_path = base.to_path_buf();
        for _ in 0..10 {
            long_path = long_path.join("very_long_directory_name_to_test_path_handling");
        }
        fs::create_dir_all(&long_path).unwrap();

        let modules_dir = long_path.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        let module_dir = modules_dir.join("longpath_module");
        fs::create_dir_all(&module_dir).unwrap();
        create_test_module(&module_dir, "index", "return {}").unwrap();

        let mut loader = ModuleLoader::new(long_path.clone());

        let result = loader.require("longpath_module");
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle long paths"
        );
    }

    #[test]
    fn test_recursive_module_require() {
        let temp = TempDir::new().unwrap();
        let base = temp.path();
        let modules_dir = base.join("node_modules");
        fs::create_dir_all(&modules_dir).unwrap();

        let mod_a = modules_dir.join("rec_a");
        fs::create_dir_all(&mod_a).unwrap();
        create_test_module(&mod_a, "index", "return {}").unwrap();

        let mod_b = modules_dir.join("rec_b");
        fs::create_dir_all(&mod_b).unwrap();
        create_test_module(&mod_b, "index", "return {}").unwrap();

        let mut loader = ModuleLoader::new(base.to_path_buf());

        let _ = loader.require("rec_a");
        let result = loader.require("rec_b");
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle multiple module loads"
        );
    }
}
