use anyhow::{anyhow, Result};
use std::env;
use std::path::{Path, PathBuf};

pub struct PathResolver;

impl PathResolver {
    pub fn resolve_script_path(script_path: &str) -> Result<PathBuf> {
        let path = PathBuf::from(script_path);

        if path.is_absolute() {
            return Ok(path);
        }

        let current_dir = env::current_dir()?;
        let resolved = current_dir.join(&path);

        if !resolved.exists() {
            return Err(anyhow!("Script not found: {}", script_path));
        }

        Ok(resolved)
    }

    pub fn get_script_name(path: &Path) -> String {
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("script")
            .to_string()
    }

    pub fn get_script_dir(path: &Path) -> PathBuf {
        path.parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf()
    }

    pub fn get_working_dir() -> Result<PathBuf> {
        env::current_dir().map_err(|e| anyhow!(e))
    }

    pub fn set_working_dir(path: &Path) -> Result<()> {
        env::set_current_dir(path).map_err(|e| anyhow!(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_resolve_script_path() {
        let test_file = "test_script.lua";
        fs::write(test_file, "print 'hello'").unwrap();

        let resolved = PathResolver::resolve_script_path(test_file).unwrap();
        assert!(resolved.exists());

        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_get_script_name() {
        let path = Path::new("/path/to/script.lua");
        assert_eq!(PathResolver::get_script_name(path), "script.lua");
    }

    #[test]
    fn test_get_script_dir() {
        let path = Path::new("/path/to/script.lua");
        let dir = PathResolver::get_script_dir(path);
        assert_eq!(dir, PathBuf::from("/path/to"));
    }
}
