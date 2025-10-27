use super::error::ProcessError;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, ProcessError>;

pub fn get_cwd() -> Result<String> {
    env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(Into::into)
}

pub fn set_cwd(path: &str) -> Result<()> {
    let path_buf = PathBuf::from(path);
    env::set_current_dir(&path_buf).map_err(Into::into)
}

pub fn get_env() -> HashMap<String, String> {
    env::vars().collect()
}

pub fn get_env_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

pub fn set_env_var(key: &str, value: &str) {
    env::set_var(key, value);
}

pub fn remove_env_var(key: &str) {
    env::remove_var(key);
}

pub fn get_pid() -> u32 {
    std::process::id()
}

pub fn get_platform() -> String {
    env::consts::OS.to_string()
}

pub fn get_arch() -> String {
    env::consts::ARCH.to_string()
}

pub fn exit(code: i32) -> ! {
    std::process::exit(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cwd() {
        let cwd = get_cwd().unwrap();
        assert!(!cwd.is_empty());
    }

    #[test]
    fn test_get_env() {
        let env = get_env();
        assert!(!env.is_empty());
    }

    #[test]
    fn test_env_var_operations() {
        let key = "HYPE_TEST_VAR";
        let value = "test_value";

        set_env_var(key, value);
        assert_eq!(get_env_var(key), Some(value.to_string()));

        remove_env_var(key);
        assert_eq!(get_env_var(key), None);
    }

    #[test]
    fn test_get_pid() {
        let pid = get_pid();
        assert!(pid > 0);
    }

    #[test]
    fn test_get_platform() {
        let platform = get_platform();
        assert!(matches!(
            platform.as_str(),
            "linux" | "macos" | "windows" | "freebsd" | "netbsd" | "openbsd" | "dragonfly"
        ));
    }

    #[test]
    fn test_get_arch() {
        let arch = get_arch();
        assert!(matches!(
            arch.as_str(),
            "x86_64" | "x86" | "aarch64" | "arm" | "powerpc64" | "riscv64"
        ));
    }

    #[test]
    fn test_set_cwd() {
        let original = get_cwd().unwrap();
        let temp = std::env::temp_dir();
        let temp_str = temp.to_string_lossy().to_string();

        set_cwd(&temp_str).unwrap();
        let new_cwd = get_cwd().unwrap();
        assert_eq!(
            PathBuf::from(new_cwd).canonicalize().unwrap(),
            temp.canonicalize().unwrap()
        );

        set_cwd(&original).unwrap();
    }
}
