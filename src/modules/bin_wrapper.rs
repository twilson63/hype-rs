use crate::error::Result;
use std::path::Path;

pub struct BinWrapper;

impl BinWrapper {
    const UNIX_TEMPLATE: &'static str = r#"#!/usr/bin/env bash
HYPE_BIN="$(command -v hype)"
if [ -z "$HYPE_BIN" ]; then
    echo "Error: hype not found in PATH" >&2
    exit 1
fi
PACKAGE_DIR="{PACKAGE_DIR}"
SCRIPT_PATH="$PACKAGE_DIR/{SCRIPT_RELATIVE}"
exec "$HYPE_BIN" "$SCRIPT_PATH" "$@"
"#;

    const WINDOWS_TEMPLATE: &'static str = r#"@echo off
where hype >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Error: hype not found in PATH >&2
    exit /b 1
)
set PACKAGE_DIR={PACKAGE_DIR}
set SCRIPT_PATH=%PACKAGE_DIR%\{SCRIPT_RELATIVE}
hype "%SCRIPT_PATH%" %*
"#;

    pub fn create_unix_wrapper(
        bin_path: &Path,
        package_dir: &Path,
        script_relative: &str,
    ) -> Result<()> {
        let package_dir_str = package_dir.to_string_lossy();
        let content = Self::UNIX_TEMPLATE
            .replace("{PACKAGE_DIR}", &package_dir_str)
            .replace("{SCRIPT_RELATIVE}", script_relative);

        std::fs::write(bin_path, content)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = std::fs::Permissions::from_mode(0o755);
            std::fs::set_permissions(bin_path, permissions)?;
        }

        Ok(())
    }

    pub fn create_windows_wrapper(
        bin_path: &Path,
        package_dir: &Path,
        script_relative: &str,
    ) -> Result<()> {
        let package_dir_str = package_dir.to_string_lossy().replace('/', "\\");
        let script_relative_windows = script_relative.replace('/', "\\");
        let content = Self::WINDOWS_TEMPLATE
            .replace("{PACKAGE_DIR}", &package_dir_str)
            .replace("{SCRIPT_RELATIVE}", &script_relative_windows);

        std::fs::write(bin_path, content)?;

        Ok(())
    }

    #[cfg(unix)]
    pub fn create_wrapper(
        bin_path: &Path,
        package_dir: &Path,
        script_relative: &str,
    ) -> Result<()> {
        Self::create_unix_wrapper(bin_path, package_dir, script_relative)
    }

    #[cfg(windows)]
    pub fn create_wrapper(
        bin_path: &Path,
        package_dir: &Path,
        script_relative: &str,
    ) -> Result<()> {
        Self::create_windows_wrapper(bin_path, package_dir, script_relative)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_create_unix_wrapper() {
        let temp_dir = TempDir::new().unwrap();
        let bin_path = temp_dir.path().join("test-bin");
        let package_dir = PathBuf::from("/home/user/.hype/packages/mypackage");
        let script_relative = "bin/script.lua";

        BinWrapper::create_unix_wrapper(&bin_path, &package_dir, script_relative).unwrap();

        let content = std::fs::read_to_string(&bin_path).unwrap();
        assert!(content.contains("#!/usr/bin/env bash"));
        assert!(content.contains("PACKAGE_DIR=\"/home/user/.hype/packages/mypackage\""));
        assert!(content.contains("SCRIPT_PATH=\"$PACKAGE_DIR/bin/script.lua\""));
        assert!(content.contains("exec \"$HYPE_BIN\" \"$SCRIPT_PATH\" \"$@\""));
    }

    #[test]
    fn test_create_windows_wrapper() {
        let temp_dir = TempDir::new().unwrap();
        let bin_path = temp_dir.path().join("test-bin.bat");
        let package_dir = PathBuf::from("C:\\Users\\user\\.hype\\packages\\mypackage");
        let script_relative = "bin/script.lua";

        BinWrapper::create_windows_wrapper(&bin_path, &package_dir, script_relative).unwrap();

        let content = std::fs::read_to_string(&bin_path).unwrap();
        assert!(content.contains("@echo off"));
        assert!(content.contains("set PACKAGE_DIR=C:\\Users\\user\\.hype\\packages\\mypackage"));
        assert!(content.contains("set SCRIPT_PATH=%PACKAGE_DIR%\\bin\\script.lua"));
        assert!(content.contains("hype \"%SCRIPT_PATH%\" %*"));
    }

    #[test]
    #[cfg(unix)]
    fn test_unix_wrapper_executable_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let temp_dir = TempDir::new().unwrap();
        let bin_path = temp_dir.path().join("test-bin");
        let package_dir = PathBuf::from("/home/user/.hype/packages/mypackage");
        let script_relative = "bin/script.lua";

        BinWrapper::create_unix_wrapper(&bin_path, &package_dir, script_relative).unwrap();

        let metadata = std::fs::metadata(&bin_path).unwrap();
        let permissions = metadata.permissions();
        assert_eq!(permissions.mode() & 0o777, 0o755);
    }

    #[test]
    fn test_wrapper_with_spaces_in_paths() {
        let temp_dir = TempDir::new().unwrap();
        let bin_path = temp_dir.path().join("test-bin");
        let package_dir = PathBuf::from("/home/user/My Documents/.hype/packages/my package");
        let script_relative = "bin/my script.lua";

        BinWrapper::create_unix_wrapper(&bin_path, &package_dir, script_relative).unwrap();

        let content = std::fs::read_to_string(&bin_path).unwrap();
        assert!(
            content.contains("PACKAGE_DIR=\"/home/user/My Documents/.hype/packages/my package\"")
        );
        assert!(content.contains("SCRIPT_PATH=\"$PACKAGE_DIR/bin/my script.lua\""));
    }

    #[test]
    fn test_windows_wrapper_path_separators() {
        let temp_dir = TempDir::new().unwrap();
        let bin_path = temp_dir.path().join("test-bin.bat");
        let package_dir = PathBuf::from("C:/Users/user/.hype/packages/mypackage");
        let script_relative = "bin/script.lua";

        BinWrapper::create_windows_wrapper(&bin_path, &package_dir, script_relative).unwrap();

        let content = std::fs::read_to_string(&bin_path).unwrap();
        assert!(content.contains("C:\\Users\\user\\.hype\\packages\\mypackage"));
        assert!(content.contains("\\bin\\script.lua"));
    }

    #[test]
    fn test_wrapper_content_is_utf8() {
        let temp_dir = TempDir::new().unwrap();
        let bin_path = temp_dir.path().join("test-bin");
        let package_dir = PathBuf::from("/home/user/.hype/packages/mypackage");
        let script_relative = "bin/script.lua";

        BinWrapper::create_unix_wrapper(&bin_path, &package_dir, script_relative).unwrap();

        let result = std::fs::read_to_string(&bin_path);
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(unix)]
    fn test_create_wrapper_delegates_to_unix() {
        let temp_dir = TempDir::new().unwrap();
        let bin_path = temp_dir.path().join("test-bin");
        let package_dir = PathBuf::from("/home/user/.hype/packages/mypackage");
        let script_relative = "bin/script.lua";

        BinWrapper::create_wrapper(&bin_path, &package_dir, script_relative).unwrap();

        let content = std::fs::read_to_string(&bin_path).unwrap();
        assert!(content.contains("#!/usr/bin/env bash"));
    }

    #[test]
    #[cfg(windows)]
    fn test_create_wrapper_delegates_to_windows() {
        let temp_dir = TempDir::new().unwrap();
        let bin_path = temp_dir.path().join("test-bin.bat");
        let package_dir = PathBuf::from("C:\\Users\\user\\.hype\\packages\\mypackage");
        let script_relative = "bin\\script.lua";

        BinWrapper::create_wrapper(&bin_path, &package_dir, script_relative).unwrap();

        let content = std::fs::read_to_string(&bin_path).unwrap();
        assert!(content.contains("@echo off"));
    }
}
