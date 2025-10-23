use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_cli_module_flag_long() {
    let temp_dir = TempDir::new().unwrap();
    let module_path = temp_dir.path().join("test_module.lua");

    fs::write(&module_path, "print('Module loaded successfully')").unwrap();

    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "--module", module_path.to_str().unwrap()])
        .current_dir("/Users/rakis/hype-projects/hype-rs")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Module execution should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Module loaded successfully"),
        "Output should contain module output"
    );
}

#[test]
fn test_cli_module_flag_short() {
    let temp_dir = TempDir::new().unwrap();
    let module_path = temp_dir.path().join("test_module.lua");

    fs::write(&module_path, "print('Short form works')").unwrap();

    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "-m", module_path.to_str().unwrap()])
        .current_dir("/Users/rakis/hype-projects/hype-rs")
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Module execution with -m should succeed"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Short form works"),
        "Output should contain module output"
    );
}

#[test]
fn test_cli_module_file_not_found() {
    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "--module", "/nonexistent/path/module.lua"])
        .current_dir("/Users/rakis/hype-projects/hype-rs")
        .output()
        .expect("Failed to execute command");

    assert!(
        !output.status.success(),
        "Module execution should fail for nonexistent file"
    );
}

#[test]
fn test_cli_module_with_require() {
    let temp_dir = TempDir::new().unwrap();
    let module_path = temp_dir.path().join("main_module.lua");

    fs::write(
        &module_path,
        r#"local fs = require('fs')
print('Require succeeded')
assert(fs ~= nil, 'fs module should not be nil')
print('fs module is available')"#,
    )
    .unwrap();

    let output = std::process::Command::new("cargo")
        .args(&[
            "run",
            "--quiet",
            "--",
            "--module",
            module_path.to_str().unwrap(),
        ])
        .current_dir("/Users/rakis/hype-projects/hype-rs")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);

    assert!(
        output.status.success(),
        "Module with require should succeed. stderr: {}",
        stderr
    );
    assert!(
        combined.contains("Require succeeded"),
        "Output should contain require success message. stdout: {}, stderr: {}",
        stdout,
        stderr
    );
}

#[test]
fn test_cli_module_with_verbose() {
    let temp_dir = TempDir::new().unwrap();
    let module_path = temp_dir.path().join("verbose_test.lua");

    fs::write(&module_path, "print('Verbose test')").unwrap();

    let output = std::process::Command::new("cargo")
        .args(&[
            "run",
            "--",
            "--verbose",
            "--module",
            module_path.to_str().unwrap(),
        ])
        .current_dir("/Users/rakis/hype-projects/hype-rs")
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Module execution with --verbose should succeed"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Module mode enabled") || stderr.contains("module"),
        "Verbose output should mention module execution"
    );
}

#[test]
fn test_cli_module_with_env_variables() {
    let temp_dir = TempDir::new().unwrap();
    let module_path = temp_dir.path().join("env_test.lua");

    fs::write(
        &module_path,
        r#"
print('Module environment:')
print(__dirname)
print(__filename)
"#,
    )
    .unwrap();

    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "--module", module_path.to_str().unwrap()])
        .current_dir("/Users/rakis/hype-projects/hype-rs")
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Module should have environment variables"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Module environment"),
        "Output should show environment section"
    );
}
