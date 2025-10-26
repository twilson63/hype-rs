use crate::error::{HypeError, Result};
use crate::modules::bin_wrapper::BinWrapper;
use crate::modules::manifest::HypeManifest;
use crate::modules::registry_global::{GlobalPackageRegistry, InstalledPackage};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

pub struct InstallArgs {
    pub path: Option<PathBuf>,
    pub force: bool,
    pub verbose: bool,
}

pub fn install_package(args: InstallArgs) -> Result<()> {
    let source_path = args
        .path
        .unwrap_or_else(|| PathBuf::from("."))
        .canonicalize()
        .map_err(|e| HypeError::Execution(format!("Invalid package path: {}", e)))?;

    let manifest_path = source_path.join("hype.json");
    if !manifest_path.exists() {
        return Err(HypeError::Execution(
            "No hype.json found in package directory".to_string(),
        ));
    }

    if args.verbose {
        println!("Loading manifest from {}", manifest_path.display());
    }

    let manifest = HypeManifest::load(&manifest_path)?;

    manifest.validate_with_package_dir(&source_path)?;

    let empty_map = HashMap::new();
    let bin_map = manifest.bin.as_ref().unwrap_or(&empty_map);
    if bin_map.is_empty() {
        return Err(HypeError::Execution(
            "Package has no 'bin' field - nothing to install".to_string(),
        ));
    }

    let mut registry = GlobalPackageRegistry::load()
        .map_err(|e| HypeError::Execution(format!("Failed to load package registry: {}", e)))?;

    if !args.force {
        let conflicts = registry
            .check_bin_conflict(bin_map)
            .map_err(|e| HypeError::Execution(format!("Failed to check for conflicts: {}", e)))?;

        if !conflicts.is_empty() {
            return Err(HypeError::Execution(format!(
                "Binary command conflicts detected: {}. Use --force to overwrite.",
                conflicts.join(", ")
            )));
        }
    }

    if args.force && registry.get(&manifest.name).is_some() {
        if args.verbose {
            println!("Removing existing package: {}", manifest.name);
        }
        let old_pkg = registry.remove_package(&manifest.name).map_err(|e| {
            HypeError::Execution(format!("Failed to remove existing package: {}", e))
        })?;

        for cmd in old_pkg.bin.keys() {
            let wrapper_path = registry.bin_dir().join(cmd);
            if wrapper_path.exists() {
                std::fs::remove_file(&wrapper_path)?;
            }
        }

        if old_pkg.location.exists() {
            std::fs::remove_dir_all(&old_pkg.location)?;
        }
    }

    let pkg_version_str = format!("{}@{}", manifest.name, manifest.version);
    let dest_dir = registry.packages_dir().join(&pkg_version_str);

    if args.verbose {
        println!("Copying package to {}", dest_dir.display());
    }

    copy_package(&source_path, &dest_dir)?;

    if args.verbose {
        println!("Creating binary wrappers...");
    }

    for (cmd_name, script_path) in bin_map {
        let wrapper_path = registry.bin_dir().join(cmd_name);

        if args.verbose {
            println!("  Creating wrapper: {}", cmd_name);
        }

        BinWrapper::create_wrapper(&wrapper_path, &dest_dir, script_path)?;
    }

    let install_date = chrono::Utc::now().to_rfc3339();

    let installed_pkg = InstalledPackage {
        name: manifest.name.clone(),
        version: manifest.version.clone(),
        install_date,
        location: dest_dir,
        bin: bin_map.clone(),
    };

    registry
        .add_package(installed_pkg)
        .map_err(|e| HypeError::Execution(format!("Failed to update registry: {}", e)))?;

    println!(
        "✓ Successfully installed {}@{}",
        manifest.name, manifest.version
    );
    println!(
        "  Commands: {}",
        bin_map.keys().cloned().collect::<Vec<_>>().join(", ")
    );
    println!();
    println!("To use these commands, add ~/.hype/bin to your PATH:");
    println!("  export PATH=\"$HOME/.hype/bin:$PATH\"");

    Ok(())
}

pub fn uninstall_package(name: String, verbose: bool) -> Result<()> {
    let mut registry = GlobalPackageRegistry::load()
        .map_err(|e| HypeError::Execution(format!("Failed to load package registry: {}", e)))?;

    let pkg = registry
        .get(&name)
        .ok_or_else(|| HypeError::Execution(format!("Package '{}' is not installed", name)))?;

    if verbose {
        println!("Removing binary wrappers...");
    }

    for cmd in pkg.bin.keys() {
        let wrapper_path = registry.bin_dir().join(cmd);
        if wrapper_path.exists() {
            if verbose {
                println!("  Removing: {}", cmd);
            }
            std::fs::remove_file(&wrapper_path)?;
        }
    }

    let pkg_location = pkg.location.clone();
    let pkg_version = pkg.version.clone();

    registry.remove_package(&name).map_err(|e| {
        HypeError::Execution(format!("Failed to remove package from registry: {}", e))
    })?;

    if pkg_location.exists() {
        if verbose {
            println!("Removing package directory: {}", pkg_location.display());
        }
        std::fs::remove_dir_all(&pkg_location)?;
    }

    println!("✓ Successfully uninstalled {}@{}", name, pkg_version);

    Ok(())
}

pub fn list_packages(verbose: bool, json: bool) -> Result<()> {
    let registry = GlobalPackageRegistry::load()
        .map_err(|e| HypeError::Execution(format!("Failed to load package registry: {}", e)))?;

    let packages = registry.list();

    if json {
        let json_output = serde_json::to_string_pretty(&packages)
            .map_err(|e| HypeError::Execution(format!("Failed to serialize packages: {}", e)))?;
        println!("{}", json_output);
        return Ok(());
    }

    if packages.is_empty() {
        println!("No globally installed packages.");
        return Ok(());
    }

    println!("Globally installed packages:\n");

    for pkg in &packages {
        println!("  {}@{}", pkg.name, pkg.version);

        let commands: Vec<&String> = pkg.bin.keys().collect();
        if !commands.is_empty() {
            println!(
                "    Commands: {}",
                commands
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        if verbose {
            println!("    Location: {}", pkg.location.display());
            println!("    Installed: {}", pkg.install_date);
        }

        println!();
    }

    println!(
        "Total: {} package{}",
        packages.len(),
        if packages.len() == 1 { "" } else { "s" }
    );

    Ok(())
}

pub fn which_command(cmd: String) -> Result<()> {
    let registry = GlobalPackageRegistry::load()
        .map_err(|e| HypeError::Execution(format!("Failed to load package registry: {}", e)))?;

    if let Some(pkg_version) = registry.which_command(&cmd) {
        let parts: Vec<&str> = pkg_version.split('@').collect();
        if parts.len() == 2 {
            let pkg_name = parts[0];
            if let Some(pkg) = registry.get(pkg_name) {
                if let Some(script_path) = pkg.bin.get(&cmd) {
                    println!("{} is provided by {}", cmd, pkg_version);
                    println!("Location: {}/{}", pkg.location.display(), script_path);
                    return Ok(());
                }
            }
        }
    }

    println!("Command '{}' not found in globally installed packages", cmd);
    std::process::exit(1);
}

fn copy_package(source: &Path, dest: &Path) -> Result<()> {
    if dest.exists() {
        std::fs::remove_dir_all(dest)?;
    }

    std::fs::create_dir_all(dest)?;

    copy_dir_recursive(source, dest)?;

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let file_name = entry.file_name();

        if file_name == ".git" || file_name == "node_modules" || file_name == "target" {
            continue;
        }

        let dst_path = dst.join(&file_name);

        if file_type.is_dir() {
            std::fs::create_dir_all(&dst_path)?;
            copy_dir_recursive(&src_path, &dst_path)?;
        } else if file_type.is_file() {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

pub fn get_hype_home() -> PathBuf {
    if let Ok(hype_home) = env::var("HYPE_HOME") {
        return PathBuf::from(hype_home);
    }

    if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home).join(".hype");
    }

    PathBuf::from(".hype")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;
    use tempfile::TempDir;

    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    fn setup_test_env() -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().unwrap();
        let hype_home = temp_dir.path().join(".hype");
        std::fs::create_dir_all(&hype_home).unwrap();
        env::set_var("HYPE_HOME", &hype_home);
        (temp_dir, hype_home)
    }

    fn create_test_package_dir(temp: &TempDir, name: &str) -> PathBuf {
        let pkg_dir = temp.path().join(name);
        std::fs::create_dir_all(&pkg_dir).unwrap();

        let bin_dir = pkg_dir.join("bin");
        std::fs::create_dir_all(&bin_dir).unwrap();
        std::fs::write(bin_dir.join("cli.lua"), "print('test')").unwrap();

        let mut bin_map = HashMap::new();
        bin_map.insert("testcmd".to_string(), "bin/cli.lua".to_string());

        let manifest = serde_json::json!({
            "name": name,
            "version": "1.0.0",
            "bin": bin_map,
        });

        std::fs::write(
            pkg_dir.join("hype.json"),
            serde_json::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();

        pkg_dir
    }

    #[test]
    fn test_get_hype_home_with_env_var() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let custom_path = temp_dir.path().join("custom_hype");
        env::set_var("HYPE_HOME", &custom_path);

        let result = get_hype_home();
        assert_eq!(result, custom_path);

        env::remove_var("HYPE_HOME");
    }

    #[test]
    fn test_get_hype_home_with_home_var() {
        let _lock = TEST_MUTEX.lock().unwrap();
        env::remove_var("HYPE_HOME");

        if let Ok(home) = env::var("HOME") {
            let result = get_hype_home();
            let expected = PathBuf::from(home).join(".hype");
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_get_hype_home_fallback() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let original_hype = env::var("HYPE_HOME").ok();
        let original_home = env::var("HOME").ok();

        env::remove_var("HYPE_HOME");
        env::remove_var("HOME");

        let result = get_hype_home();
        assert_eq!(result, PathBuf::from(".hype"));

        if let Some(val) = original_hype {
            env::set_var("HYPE_HOME", val);
        }
        if let Some(val) = original_home {
            env::set_var("HOME", val);
        }
    }

    #[test]
    fn test_copy_package_creates_destination() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("source");
        std::fs::create_dir_all(&source).unwrap();
        std::fs::write(source.join("test.txt"), "content").unwrap();

        let dest = temp.path().join("dest");

        copy_package(&source, &dest).unwrap();

        assert!(dest.exists());
        assert!(dest.join("test.txt").exists());
        assert_eq!(
            std::fs::read_to_string(dest.join("test.txt")).unwrap(),
            "content"
        );
    }

    #[test]
    fn test_copy_package_recursive() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("source");
        std::fs::create_dir_all(source.join("subdir")).unwrap();
        std::fs::write(source.join("file1.txt"), "content1").unwrap();
        std::fs::write(source.join("subdir/file2.txt"), "content2").unwrap();

        let dest = temp.path().join("dest");

        copy_package(&source, &dest).unwrap();

        assert!(dest.join("file1.txt").exists());
        assert!(dest.join("subdir").exists());
        assert!(dest.join("subdir/file2.txt").exists());
        assert_eq!(
            std::fs::read_to_string(dest.join("subdir/file2.txt")).unwrap(),
            "content2"
        );
    }

    #[test]
    fn test_copy_package_excludes_git() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("source");
        std::fs::create_dir_all(source.join(".git")).unwrap();
        std::fs::write(source.join(".git/config"), "gitconfig").unwrap();
        std::fs::write(source.join("file.txt"), "content").unwrap();

        let dest = temp.path().join("dest");

        copy_package(&source, &dest).unwrap();

        assert!(dest.join("file.txt").exists());
        assert!(!dest.join(".git").exists());
    }

    #[test]
    fn test_copy_package_excludes_node_modules() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("source");
        std::fs::create_dir_all(source.join("node_modules")).unwrap();
        std::fs::write(source.join("node_modules/dep.js"), "dep").unwrap();
        std::fs::write(source.join("file.txt"), "content").unwrap();

        let dest = temp.path().join("dest");

        copy_package(&source, &dest).unwrap();

        assert!(dest.join("file.txt").exists());
        assert!(!dest.join("node_modules").exists());
    }

    #[test]
    fn test_copy_package_excludes_target() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("source");
        std::fs::create_dir_all(source.join("target")).unwrap();
        std::fs::write(source.join("target/debug"), "debug").unwrap();
        std::fs::write(source.join("file.txt"), "content").unwrap();

        let dest = temp.path().join("dest");

        copy_package(&source, &dest).unwrap();

        assert!(dest.join("file.txt").exists());
        assert!(!dest.join("target").exists());
    }

    #[test]
    fn test_copy_package_replaces_existing() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("source");
        std::fs::create_dir_all(&source).unwrap();
        std::fs::write(source.join("new.txt"), "new content").unwrap();

        let dest = temp.path().join("dest");
        std::fs::create_dir_all(&dest).unwrap();
        std::fs::write(dest.join("old.txt"), "old content").unwrap();

        copy_package(&source, &dest).unwrap();

        assert!(dest.join("new.txt").exists());
        assert!(!dest.join("old.txt").exists());
    }

    #[test]
    fn test_install_package_missing_manifest() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let (_temp, _hype_home) = setup_test_env();
        let pkg_temp = TempDir::new().unwrap();
        let pkg_dir = pkg_temp.path().join("no-manifest");
        std::fs::create_dir_all(&pkg_dir).unwrap();

        let result = install_package(InstallArgs {
            path: Some(pkg_dir),
            force: false,
            verbose: false,
        });

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("No hype.json found"));
    }

    #[test]
    fn test_install_package_no_bin_field() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let (_temp, _hype_home) = setup_test_env();
        let pkg_temp = TempDir::new().unwrap();
        let pkg_dir = pkg_temp.path().join("no-bin");
        std::fs::create_dir_all(&pkg_dir).unwrap();

        let manifest = serde_json::json!({
            "name": "no-bin-pkg",
            "version": "1.0.0",
        });

        std::fs::write(
            pkg_dir.join("hype.json"),
            serde_json::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();

        let result = install_package(InstallArgs {
            path: Some(pkg_dir),
            force: false,
            verbose: false,
        });

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("no 'bin' field"));
    }

    #[test]
    fn test_install_package_success() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let (temp, _hype_home) = setup_test_env();
        let pkg_dir = create_test_package_dir(&temp, "test-pkg");

        let result = install_package(InstallArgs {
            path: Some(pkg_dir),
            force: false,
            verbose: false,
        });

        assert!(result.is_ok());
    }

    #[test]
    fn test_uninstall_package_not_found() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let (_temp, _hype_home) = setup_test_env();

        let result = uninstall_package("nonexistent-pkg".to_string(), false);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("is not installed"));
    }

    #[test]
    fn test_uninstall_package_success() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let (temp, _hype_home) = setup_test_env();
        let pkg_dir = create_test_package_dir(&temp, "uninstall-test");

        install_package(InstallArgs {
            path: Some(pkg_dir),
            force: false,
            verbose: false,
        })
        .unwrap();

        let result = uninstall_package("uninstall-test".to_string(), false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_list_packages_empty() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let (_temp, _hype_home) = setup_test_env();

        let result = list_packages(false, false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_list_packages_json_format() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let (_temp, _hype_home) = setup_test_env();

        let result = list_packages(false, true);

        assert!(result.is_ok());
    }

    #[test]
    fn test_copy_dir_recursive_empty_dir() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("empty_source");
        std::fs::create_dir_all(&source).unwrap();

        let dest = temp.path().join("empty_dest");
        std::fs::create_dir_all(&dest).unwrap();

        let result = copy_dir_recursive(&source, &dest);

        assert!(result.is_ok());
        assert!(dest.exists());
    }

    #[test]
    fn test_install_package_with_force() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let (temp, _hype_home) = setup_test_env();
        let pkg_dir = create_test_package_dir(&temp, "force-test");

        install_package(InstallArgs {
            path: Some(pkg_dir.clone()),
            force: false,
            verbose: false,
        })
        .unwrap();

        let result = install_package(InstallArgs {
            path: Some(pkg_dir),
            force: true,
            verbose: false,
        });

        assert!(result.is_ok());
    }

    #[test]
    fn test_install_package_conflict_without_force() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let (temp, _hype_home) = setup_test_env();
        let pkg1_dir = create_test_package_dir(&temp, "conflict-pkg1");

        install_package(InstallArgs {
            path: Some(pkg1_dir),
            force: false,
            verbose: false,
        })
        .unwrap();

        let pkg2_dir = temp.path().join("conflict-pkg2");
        std::fs::create_dir_all(&pkg2_dir).unwrap();
        let bin_dir = pkg2_dir.join("bin");
        std::fs::create_dir_all(&bin_dir).unwrap();
        std::fs::write(bin_dir.join("cli.lua"), "print('test2')").unwrap();

        let mut bin_map = HashMap::new();
        bin_map.insert("testcmd".to_string(), "bin/cli.lua".to_string());

        let manifest = serde_json::json!({
            "name": "conflict-pkg2",
            "version": "1.0.0",
            "bin": bin_map,
        });

        std::fs::write(
            pkg2_dir.join("hype.json"),
            serde_json::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();

        let result = install_package(InstallArgs {
            path: Some(pkg2_dir),
            force: false,
            verbose: false,
        });

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("conflict"));
    }

    #[test]
    fn test_install_package_verbose_mode() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let (temp, _hype_home) = setup_test_env();
        let pkg_dir = create_test_package_dir(&temp, "verbose-test");

        let result = install_package(InstallArgs {
            path: Some(pkg_dir),
            force: false,
            verbose: true,
        });

        assert!(result.is_ok());
    }

    #[test]
    fn test_uninstall_package_verbose_mode() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let (temp, _hype_home) = setup_test_env();
        let pkg_dir = create_test_package_dir(&temp, "uninstall-verbose-test");

        install_package(InstallArgs {
            path: Some(pkg_dir),
            force: false,
            verbose: false,
        })
        .unwrap();

        let result = uninstall_package("uninstall-verbose-test".to_string(), true);

        assert!(result.is_ok());
    }

    #[test]
    fn test_list_packages_verbose_mode() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let (_temp, _hype_home) = setup_test_env();

        let result = list_packages(true, false);

        assert!(result.is_ok());
    }
}
