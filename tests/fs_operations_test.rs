use hype_rs::lua::require::setup_require_fn;
use hype_rs::modules::loader::ModuleLoader;
use mlua::Lua;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tempfile::TempDir;

#[test]
fn test_fs_module_read_write() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");

    let lua_ctx = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua_ctx, loader).unwrap();

    let code = format!(
        r#"
        local fs = require("fs")
        fs.writeFileSync("{}", "test content")
        return fs.readFileSync("{}")
        "#,
        test_file.display(),
        test_file.display()
    );

    let result: String = lua_ctx.load(&code).eval().unwrap();
    assert_eq!(result, "test content");
}

#[test]
fn test_fs_module_exists() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "content").unwrap();

    let lua_ctx = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua_ctx, loader).unwrap();

    let code = format!(
        r#"
        local fs = require("fs")
        return fs.existsSync("{}")
        "#,
        test_file.display()
    );

    let result: bool = lua_ctx.load(&code).eval().unwrap();
    assert!(result);

    let code_nonexistent = format!(
        r#"
        local fs = require("fs")
        return fs.existsSync("{}")
        "#,
        temp_dir.path().join("nonexistent.txt").display()
    );

    let result_nonexistent: bool = lua_ctx.load(&code_nonexistent).eval().unwrap();
    assert!(!result_nonexistent);
}

#[test]
fn test_fs_module_stat() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "hello").unwrap();

    let lua_ctx = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua_ctx, loader).unwrap();

    let code = format!(
        r#"
        local fs = require("fs")
        local stat = fs.statSync("{}")
        return stat.size, stat.isFile, stat.isDirectory
        "#,
        test_file.display()
    );

    let (size, is_file, is_dir): (u64, bool, bool) = lua_ctx.load(&code).eval().unwrap();
    assert_eq!(size, 5);
    assert!(is_file);
    assert!(!is_dir);
}

#[test]
fn test_fs_module_readdir() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("file1.txt"), "content").unwrap();
    fs::write(temp_dir.path().join("file2.txt"), "content").unwrap();
    fs::create_dir(temp_dir.path().join("subdir")).unwrap();

    let lua_ctx = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua_ctx, loader).unwrap();

    let code = format!(
        r#"
        local fs = require("fs")
        local files = fs.readdirSync("{}")
        return #files, files[1], files[2], files[3]
        "#,
        temp_dir.path().display()
    );

    let (count, file1, file2, file3): (usize, String, String, String) =
        lua_ctx.load(&code).eval().unwrap();

    assert_eq!(count, 3);
    assert_eq!(file1, "file1.txt");
    assert_eq!(file2, "file2.txt");
    assert_eq!(file3, "subdir");
}

#[test]
fn test_fs_module_mkdir_rmdir() {
    let temp_dir = TempDir::new().unwrap();
    let new_dir = temp_dir.path().join("newdir");

    let lua_ctx = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua_ctx, loader).unwrap();

    let code = format!(
        r#"
        local fs = require("fs")
        fs.mkdirSync("{}")
        local exists = fs.existsSync("{}")
        fs.rmdirSync("{}")
        local exists_after = fs.existsSync("{}")
        return exists, exists_after
        "#,
        new_dir.display(),
        new_dir.display(),
        new_dir.display(),
        new_dir.display()
    );

    let (exists_before, exists_after): (bool, bool) = lua_ctx.load(&code).eval().unwrap();
    assert!(exists_before);
    assert!(!exists_after);
}

#[test]
fn test_fs_module_mkdir_recursive() {
    let temp_dir = TempDir::new().unwrap();
    let nested_dir = temp_dir.path().join("a/b/c");

    let lua_ctx = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua_ctx, loader).unwrap();

    let code = format!(
        r#"
        local fs = require("fs")
        fs.mkdirSync("{}")
        return fs.existsSync("{}")
        "#,
        nested_dir.display(),
        nested_dir.display()
    );

    let exists: bool = lua_ctx.load(&code).eval().unwrap();
    assert!(exists);
}

#[test]
fn test_fs_module_unlink() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("delete_me.txt");
    fs::write(&test_file, "content").unwrap();

    let lua_ctx = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua_ctx, loader).unwrap();

    let code = format!(
        r#"
        local fs = require("fs")
        local before = fs.existsSync("{}")
        fs.unlinkSync("{}")
        local after = fs.existsSync("{}")
        return before, after
        "#,
        test_file.display(),
        test_file.display(),
        test_file.display()
    );

    let (exists_before, exists_after): (bool, bool) = lua_ctx.load(&code).eval().unwrap();
    assert!(exists_before);
    assert!(!exists_after);
}

#[test]
fn test_fs_module_utf8_content() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("utf8.txt");

    let lua_ctx = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua_ctx, loader).unwrap();

    let utf8_content = "Hello 世界 🚀";
    let code = format!(
        r#"
        local fs = require("fs")
        fs.writeFileSync("{}", "{}")
        return fs.readFileSync("{}")
        "#,
        test_file.display(),
        utf8_content,
        test_file.display()
    );

    let result: String = lua_ctx.load(&code).eval().unwrap();
    assert_eq!(result, utf8_content);
}

#[test]
fn test_fs_module_error_handling_nonexistent() {
    let lua_ctx = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua_ctx, loader).unwrap();

    let code = r#"
        local fs = require("fs")
        fs.readFileSync("/nonexistent/file/path.txt")
    "#;

    let result = lua_ctx.load(code).exec();
    assert!(result.is_err());
}

#[test]
fn test_fs_module_error_handling_rmdir_nonempty() {
    let temp_dir = TempDir::new().unwrap();
    let test_dir = temp_dir.path().join("nonempty");
    fs::create_dir(&test_dir).unwrap();
    fs::write(test_dir.join("file.txt"), "content").unwrap();

    let lua_ctx = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua_ctx, loader).unwrap();

    let code = format!(
        r#"
        local fs = require("fs")
        fs.rmdirSync("{}")
        "#,
        test_dir.display()
    );

    let result = lua_ctx.load(&code).exec();
    assert!(result.is_err());
}
