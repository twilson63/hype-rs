use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tempfile::TempDir;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

fn get_lock() -> std::sync::MutexGuard<'static, ()> {
    match TEST_MUTEX.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn setup_test_env() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let hype_home = temp_dir.path().join(".hype");
    fs::create_dir_all(&hype_home).unwrap();
    env::set_var("HYPE_HOME", &hype_home);
    (temp_dir, hype_home)
}

fn create_test_package(
    dir: &Path,
    name: &str,
    version: &str,
    bins: HashMap<String, String>,
) -> PathBuf {
    let pkg_dir = dir.join(name);
    fs::create_dir_all(&pkg_dir).unwrap();

    if !bins.is_empty() {
        let bin_dir = pkg_dir.join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        for script in bins.values() {
            let script_path = pkg_dir.join(script);
            if let Some(parent) = script_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(&script_path, "print('Hello from script')").unwrap();
        }
    }

    let manifest = serde_json::json!({
        "name": name,
        "version": version,
        "bin": bins,
    });

    let manifest_path = pkg_dir.join("hype.json");
    fs::write(
        manifest_path,
        serde_json::to_string_pretty(&manifest).unwrap(),
    )
    .unwrap();

    pkg_dir
}

#[test]
fn test_install_package_end_to_end() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let mut bins = HashMap::new();
    bins.insert("testcli".to_string(), "bin/cli.lua".to_string());
    let pkg_dir = create_test_package(hype_home.as_path(), "test-package", "1.0.0", bins);

    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg_dir.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute install command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Install should succeed. stdout: {}, stderr: {}",
        stdout,
        stderr
    );
    assert!(
        stdout.contains("Successfully installed test-package@1.0.0"),
        "Output should confirm installation. stdout: {}",
        stdout
    );

    let bin_wrapper = hype_home.join("bin").join("testcli");
    assert!(bin_wrapper.exists(), "Binary wrapper should be created");

    let packages_dir = hype_home.join("packages").join("test-package@1.0.0");
    assert!(packages_dir.exists(), "Package should be copied");
    assert!(
        packages_dir.join("hype.json").exists(),
        "Manifest should be copied"
    );
    assert!(
        packages_dir.join("bin/cli.lua").exists(),
        "Script should be copied"
    );

    let registry_file = hype_home.join("registry.json");
    assert!(registry_file.exists(), "Registry should be created");

    let registry_content = fs::read_to_string(&registry_file).unwrap();
    assert!(
        registry_content.contains("test-package"),
        "Registry should contain package"
    );
}

#[test]
fn test_install_package_from_specified_path() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();
    let pkg_temp = TempDir::new().unwrap();

    let mut bins = HashMap::new();
    bins.insert("mycmd".to_string(), "bin/script.lua".to_string());
    let pkg_dir = create_test_package(pkg_temp.path(), "path-pkg", "2.0.0", bins);

    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg_dir.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute install command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "Install from path should succeed. stdout: {}",
        stdout
    );
    assert!(stdout.contains("Successfully installed path-pkg@2.0.0"));

    let bin_wrapper = hype_home.join("bin").join("mycmd");
    assert!(
        bin_wrapper.exists(),
        "Binary wrapper should be created from specified path"
    );
}

#[test]
fn test_uninstall_package_end_to_end() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let mut bins = HashMap::new();
    bins.insert("uninstallme".to_string(), "bin/cli.lua".to_string());
    let pkg_dir = create_test_package(hype_home.as_path(), "uninstall-pkg", "1.0.0", bins);

    let install_output = std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg_dir.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute install command");

    assert!(
        install_output.status.success(),
        "Install should succeed first"
    );

    let uninstall_output = std::process::Command::new("cargo")
        .args(&["run", "--", "uninstall", "uninstall-pkg"])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute uninstall command");

    let stdout = String::from_utf8_lossy(&uninstall_output.stdout);
    assert!(
        uninstall_output.status.success(),
        "Uninstall should succeed. stdout: {}",
        stdout
    );
    assert!(stdout.contains("Successfully uninstalled uninstall-pkg@1.0.0"));

    let bin_wrapper = hype_home.join("bin").join("uninstallme");
    assert!(!bin_wrapper.exists(), "Binary wrapper should be removed");

    let packages_dir = hype_home.join("packages").join("uninstall-pkg@1.0.0");
    assert!(
        !packages_dir.exists(),
        "Package directory should be removed"
    );
}

#[test]
fn test_list_packages() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let mut bins1 = HashMap::new();
    bins1.insert("cmd1".to_string(), "bin/cli.lua".to_string());
    let pkg1 = create_test_package(hype_home.as_path(), "pkg1", "1.0.0", bins1);

    let mut bins2 = HashMap::new();
    bins2.insert("cmd2".to_string(), "bin/cli.lua".to_string());
    let pkg2 = create_test_package(hype_home.as_path(), "pkg2", "2.0.0", bins2);

    std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg1.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to install pkg1");

    std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg2.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to install pkg2");

    let list_output = std::process::Command::new("cargo")
        .args(&["run", "--", "list"])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute list command");

    let stdout = String::from_utf8_lossy(&list_output.stdout);
    assert!(list_output.status.success(), "List should succeed");
    assert!(stdout.contains("pkg1@1.0.0"), "List should show pkg1");
    assert!(stdout.contains("pkg2@2.0.0"), "List should show pkg2");
    assert!(
        stdout.contains("Total: 2 packages"),
        "List should show count"
    );
}

#[test]
fn test_list_packages_json() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let mut bins = HashMap::new();
    bins.insert("jsoncmd".to_string(), "bin/cli.lua".to_string());
    let pkg = create_test_package(hype_home.as_path(), "json-pkg", "1.0.0", bins);

    std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to install package");

    let list_output = std::process::Command::new("cargo")
        .args(&["run", "--", "list", "--json"])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute list command");

    let stdout = String::from_utf8_lossy(&list_output.stdout);
    assert!(list_output.status.success(), "List --json should succeed");

    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("Output should be valid JSON");
    assert!(parsed.is_array(), "JSON output should be an array");
}

#[test]
fn test_which_command() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let mut bins = HashMap::new();
    bins.insert("whichtest".to_string(), "bin/cli.lua".to_string());
    let pkg = create_test_package(hype_home.as_path(), "which-pkg", "1.0.0", bins);

    std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to install package");

    let which_output = std::process::Command::new("cargo")
        .args(&["run", "--", "which", "whichtest"])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute which command");

    let stdout = String::from_utf8_lossy(&which_output.stdout);
    assert!(which_output.status.success(), "Which should succeed");
    assert!(stdout.contains("whichtest is provided by which-pkg@1.0.0"));
}

#[test]
fn test_which_command_not_found() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let which_output = std::process::Command::new("cargo")
        .args(&["run", "--", "which", "nonexistent"])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute which command");

    let stdout = String::from_utf8_lossy(&which_output.stdout);
    assert!(
        !which_output.status.success(),
        "Which should fail for nonexistent command"
    );
    assert!(stdout.contains("not found"));
}

#[test]
fn test_conflict_detection() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let mut bins1 = HashMap::new();
    bins1.insert("conflict".to_string(), "bin/cli.lua".to_string());
    let pkg1 = create_test_package(hype_home.as_path(), "pkg1", "1.0.0", bins1.clone());

    let pkg2 = create_test_package(hype_home.as_path(), "pkg2", "1.0.0", bins1);

    std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg1.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to install pkg1");

    let conflict_output = std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg2.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute install command");

    let stderr = String::from_utf8_lossy(&conflict_output.stderr);
    assert!(
        !conflict_output.status.success(),
        "Second install should fail due to conflict"
    );
    assert!(
        stderr.contains("conflict") || stderr.contains("Conflict"),
        "Error should mention conflict. stderr: {}",
        stderr
    );
}

#[test]
fn test_force_reinstall() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let mut bins = HashMap::new();
    bins.insert("forcecmd".to_string(), "bin/cli.lua".to_string());
    let pkg = create_test_package(hype_home.as_path(), "force-pkg", "1.0.0", bins);

    std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to install package");

    let force_output = std::process::Command::new("cargo")
        .args(&["run", "--", "install", "--force", pkg.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute force install");

    let stdout = String::from_utf8_lossy(&force_output.stdout);
    assert!(
        force_output.status.success(),
        "Force install should succeed. stdout: {}",
        stdout
    );
    assert!(stdout.contains("Successfully installed force-pkg@1.0.0"));
}

#[test]
fn test_multiple_packages() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let mut bins1 = HashMap::new();
    bins1.insert("multi1".to_string(), "bin/cli.lua".to_string());
    let pkg1 = create_test_package(hype_home.as_path(), "multi-pkg1", "1.0.0", bins1);

    let mut bins2 = HashMap::new();
    bins2.insert("multi2".to_string(), "bin/cli.lua".to_string());
    let pkg2 = create_test_package(hype_home.as_path(), "multi-pkg2", "2.0.0", bins2);

    let mut bins3 = HashMap::new();
    bins3.insert("multi3".to_string(), "bin/cli.lua".to_string());
    let pkg3 = create_test_package(hype_home.as_path(), "multi-pkg3", "3.0.0", bins3);

    std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg1.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to install pkg1");

    std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg2.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to install pkg2");

    std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg3.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to install pkg3");

    let list_output = std::process::Command::new("cargo")
        .args(&["run", "--", "list"])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute list command");

    let stdout = String::from_utf8_lossy(&list_output.stdout);
    assert!(
        stdout.contains("Total: 3 packages"),
        "Should show 3 packages"
    );
    assert!(stdout.contains("multi-pkg1@1.0.0"));
    assert!(stdout.contains("multi-pkg2@2.0.0"));
    assert!(stdout.contains("multi-pkg3@3.0.0"));
}

#[test]
fn test_package_without_bin_field() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let pkg_dir = hype_home.join("no-bin-pkg");
    fs::create_dir_all(&pkg_dir).unwrap();

    let manifest = serde_json::json!({
        "name": "no-bin-pkg",
        "version": "1.0.0",
    });

    let manifest_path = pkg_dir.join("hype.json");
    fs::write(
        manifest_path,
        serde_json::to_string_pretty(&manifest).unwrap(),
    )
    .unwrap();

    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg_dir.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute install command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "Install should fail for package without bin field"
    );
    assert!(
        stderr.contains("no 'bin' field") || stderr.contains("nothing to install"),
        "Error should mention missing bin field. stderr: {}",
        stderr
    );
}

#[test]
fn test_invalid_manifest_missing_file() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let pkg_dir = hype_home.join("invalid-pkg");
    fs::create_dir_all(&pkg_dir).unwrap();

    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg_dir.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute install command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "Install should fail for missing manifest"
    );
    assert!(
        stderr.contains("No hype.json found") || stderr.contains("hype.json"),
        "Error should mention missing hype.json. stderr: {}",
        stderr
    );
}

#[test]
fn test_invalid_manifest_malformed_json() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let pkg_dir = hype_home.join("malformed-pkg");
    fs::create_dir_all(&pkg_dir).unwrap();

    let manifest_path = pkg_dir.join("hype.json");
    fs::write(manifest_path, "{ invalid json }").unwrap();

    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg_dir.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute install command");

    assert!(
        !output.status.success(),
        "Install should fail for malformed JSON"
    );
}

#[test]
fn test_invalid_bin_path_nonexistent() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let pkg_dir = hype_home.join("nonexistent-bin-pkg");
    fs::create_dir_all(&pkg_dir).unwrap();

    let mut bins = HashMap::new();
    bins.insert("badcmd".to_string(), "bin/nonexistent.lua".to_string());

    let manifest = serde_json::json!({
        "name": "nonexistent-bin-pkg",
        "version": "1.0.0",
        "bin": bins,
    });

    let manifest_path = pkg_dir.join("hype.json");
    fs::write(
        manifest_path,
        serde_json::to_string_pretty(&manifest).unwrap(),
    )
    .unwrap();

    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg_dir.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute install command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "Install should fail for nonexistent bin script"
    );
    assert!(
        stderr.contains("does not exist") || stderr.contains("not found"),
        "Error should mention nonexistent script. stderr: {}",
        stderr
    );
}

#[test]
fn test_install_verbose_mode() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let mut bins = HashMap::new();
    bins.insert("verbosecmd".to_string(), "bin/cli.lua".to_string());
    let pkg = create_test_package(hype_home.as_path(), "verbose-pkg", "1.0.0", bins);

    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "install", "--verbose", pkg.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute install command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);

    assert!(output.status.success(), "Verbose install should succeed");
    assert!(
        combined.contains("Loading manifest") || combined.contains("Creating"),
        "Verbose output should contain details. combined: {}",
        combined
    );
}

#[test]
fn test_uninstall_verbose_mode() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let mut bins = HashMap::new();
    bins.insert("uninstallverbose".to_string(), "bin/cli.lua".to_string());
    let pkg = create_test_package(hype_home.as_path(), "uninstall-verbose-pkg", "1.0.0", bins);

    std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to install package");

    let output = std::process::Command::new("cargo")
        .args(&[
            "run",
            "--",
            "uninstall",
            "--verbose",
            "uninstall-verbose-pkg",
        ])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute uninstall command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);

    assert!(output.status.success(), "Verbose uninstall should succeed");
    assert!(
        combined.contains("Removing") || combined.contains("wrapper"),
        "Verbose output should contain details. combined: {}",
        combined
    );
}

#[test]
fn test_list_empty_packages() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let list_output = std::process::Command::new("cargo")
        .args(&["run", "--", "list"])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute list command");

    let stdout = String::from_utf8_lossy(&list_output.stdout);
    assert!(
        list_output.status.success(),
        "List should succeed even with no packages"
    );
    assert!(stdout.contains("No globally installed packages") || stdout.contains("Total: 0"));
}

#[test]
fn test_uninstall_nonexistent_package() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "uninstall", "nonexistent-pkg"])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute uninstall command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "Uninstall should fail for nonexistent package"
    );
    assert!(
        stderr.contains("is not installed") || stderr.contains("not found"),
        "Error should mention package not installed. stderr: {}",
        stderr
    );
}

#[test]
#[cfg(unix)]
fn test_wrapper_executable_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let mut bins = HashMap::new();
    bins.insert("exectest".to_string(), "bin/cli.lua".to_string());
    let pkg = create_test_package(hype_home.as_path(), "exec-pkg", "1.0.0", bins);

    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute install command");

    assert!(output.status.success(), "Install should succeed");

    let bin_wrapper = hype_home.join("bin").join("exectest");
    let metadata = fs::metadata(&bin_wrapper).expect("Wrapper should exist");
    let permissions = metadata.permissions();

    assert_eq!(
        permissions.mode() & 0o777,
        0o755,
        "Wrapper should have executable permissions (0755)"
    );
}

#[test]
fn test_multiple_bin_commands_in_one_package() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let mut bins = HashMap::new();
    bins.insert("cmd-a".to_string(), "bin/a.lua".to_string());
    bins.insert("cmd-b".to_string(), "bin/b.lua".to_string());
    bins.insert("cmd-c".to_string(), "bin/c.lua".to_string());

    let pkg = create_test_package(hype_home.as_path(), "multi-bin-pkg", "1.0.0", bins);

    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute install command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "Install should succeed");

    assert!(hype_home.join("bin").join("cmd-a").exists());
    assert!(hype_home.join("bin").join("cmd-b").exists());
    assert!(hype_home.join("bin").join("cmd-c").exists());

    assert!(stdout.contains("cmd-a") && stdout.contains("cmd-b") && stdout.contains("cmd-c"));
}

#[test]
fn test_registry_persistence_across_operations() {
    let _lock = get_lock();
    let (_temp, hype_home) = setup_test_env();

    let mut bins1 = HashMap::new();
    bins1.insert("persist1".to_string(), "bin/cli.lua".to_string());
    let pkg1 = create_test_package(hype_home.as_path(), "persist-pkg1", "1.0.0", bins1);

    std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg1.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to install pkg1");

    let mut bins2 = HashMap::new();
    bins2.insert("persist2".to_string(), "bin/cli.lua".to_string());
    let pkg2 = create_test_package(hype_home.as_path(), "persist-pkg2", "1.0.0", bins2);

    std::process::Command::new("cargo")
        .args(&["run", "--", "install", pkg2.to_str().unwrap()])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to install pkg2");

    std::process::Command::new("cargo")
        .args(&["run", "--", "uninstall", "persist-pkg1"])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to uninstall pkg1");

    let list_output = std::process::Command::new("cargo")
        .args(&["run", "--", "list"])
        .env("HYPE_HOME", &hype_home)
        .output()
        .expect("Failed to execute list command");

    let stdout = String::from_utf8_lossy(&list_output.stdout);
    assert!(
        !stdout.contains("persist-pkg1"),
        "Uninstalled package should not appear"
    );
    assert!(
        stdout.contains("persist-pkg2"),
        "Remaining package should still appear"
    );
}
