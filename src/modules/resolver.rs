use std::env;
use std::path::{Path, PathBuf};

use super::error::ModuleError;
use crate::error::HypeError;

#[cfg(feature = "http")]
const BUILTIN_MODULES: &[&str] = &["fs", "path", "events", "util", "table", "http"];

#[cfg(not(feature = "http"))]
const BUILTIN_MODULES: &[&str] = &["fs", "path", "events", "util", "table"];

/// Module resolver implementing Hype-RS module resolution algorithm.
///
/// Resolves module identifiers to their full filesystem paths using the
/// following priority order:
/// 1. Built-in modules (fs, path, events, util, table)
/// 2. hype_modules directories (walk up from current directory)
/// 3. Home directory modules (~/.hype/modules)
/// 4. Returns error if not found
///
/// Handles cross-platform paths, tilde expansion, and relative vs absolute paths.
pub struct ModuleResolver {
    root_dir: PathBuf,
    search_paths: Vec<PathBuf>,
}

impl ModuleResolver {
    /// Create a new ModuleResolver with the specified root directory.
    ///
    /// # Arguments
    /// * `root_dir` - The root directory from which to start module resolution
    ///
    /// # Examples
    /// ```ignore
    /// let resolver = ModuleResolver::new(PathBuf::from("."));
    /// ```
    pub fn new(root_dir: PathBuf) -> Self {
        let mut search_paths = vec![root_dir.clone()];

        let mut current = root_dir.clone();
        while current.pop() {
            search_paths.push(current.clone());
        }

        Self {
            root_dir,
            search_paths,
        }
    }

    /// Resolve a module identifier to its full filesystem path.
    ///
    /// Uses the Hype-RS module resolution algorithm:
    /// 1. Check if module is a built-in
    /// 2. Walk up directory tree looking for hype_modules/module_id
    /// 3. Check ~/.hype/modules/module_id
    /// 4. Return ModuleNotFound error
    ///
    /// # Arguments
    /// * `module_id` - The module identifier to resolve (e.g., "fs", "my-module")
    ///
    /// # Errors
    /// Returns `ModuleError::ModuleNotFound` if the module cannot be resolved
    ///
    /// # Examples
    /// ```ignore
    /// let resolver = ModuleResolver::new(PathBuf::from("."));
    /// let path = resolver.resolve("fs")?;
    /// assert!(path.ends_with("fs"));
    /// ```
    pub fn resolve(&self, module_id: &str) -> Result<PathBuf, HypeError> {
        if self.is_builtin(module_id) {
            return Ok(self.get_builtin_path(module_id));
        }

        for search_path in &self.search_paths {
            let candidate = search_path.join("hype_modules").join(module_id);
            if candidate.exists() {
                return Ok(candidate);
            }
        }

        let home_modules = self.expand_tilde("~/.hype/modules")?;
        let home_candidate = home_modules.join(module_id);
        if home_candidate.exists() {
            return Ok(home_candidate);
        }

        Err(HypeError::Execution(
            ModuleError::ModuleNotFound(module_id.to_string()).to_string(),
        ))
    }

    /// Resolve a module identifier relative to a specific path.
    ///
    /// # Arguments
    /// * `from` - The path from which to start resolution (typically __dirname)
    /// * `module_id` - The module identifier to resolve
    ///
    /// # Errors
    /// Returns `ModuleError::ModuleNotFound` if the module cannot be resolved
    ///
    /// # Examples
    /// ```ignore
    /// let resolver = ModuleResolver::new(PathBuf::from("."));
    /// let path = resolver.resolve_from(Path::new("./lib"), "my-module")?;
    /// ```
    pub fn resolve_from(&self, from: &Path, module_id: &str) -> Result<PathBuf, HypeError> {
        if self.is_builtin(module_id) {
            return Ok(self.get_builtin_path(module_id));
        }

        let mut current = from.to_path_buf();

        loop {
            let candidate = current.join("hype_modules").join(module_id);
            if candidate.exists() {
                return Ok(candidate);
            }

            if !current.pop() {
                break;
            }
        }

        let home_modules = self.expand_tilde("~/.hype/modules")?;
        let home_candidate = home_modules.join(module_id);
        if home_candidate.exists() {
            return Ok(home_candidate);
        }

        Err(HypeError::Execution(
            ModuleError::ModuleNotFound(module_id.to_string()).to_string(),
        ))
    }

    /// Check if a module name is a built-in module.
    ///
    /// Built-in modules are: fs, path, events, util, table
    ///
    /// # Arguments
    /// * `name` - The module name to check
    ///
    /// # Returns
    /// true if the module is a built-in, false otherwise
    ///
    /// # Examples
    /// ```ignore
    /// let resolver = ModuleResolver::new(PathBuf::from("."));
    /// assert!(resolver.is_builtin("fs"));
    /// assert!(!resolver.is_builtin("my-module"));
    /// ```
    pub fn is_builtin(&self, name: &str) -> bool {
        BUILTIN_MODULES.contains(&name)
    }

    /// Get the virtual path for a built-in module.
    ///
    /// Built-in modules have synthetic paths that don't correspond to
    /// actual filesystem locations.
    ///
    /// # Arguments
    /// * `name` - The built-in module name (e.g., "fs")
    ///
    /// # Returns
    /// A PathBuf representing the module's virtual path
    ///
    /// # Examples
    /// ```ignore
    /// let resolver = ModuleResolver::new(PathBuf::from("."));
    /// let path = resolver.get_builtin_path("fs");
    /// assert!(path.ends_with("fs"));
    /// ```
    pub fn get_builtin_path(&self, name: &str) -> PathBuf {
        PathBuf::from(format!("builtin://{}", name))
    }

    /// Add a search path for module resolution.
    ///
    /// Search paths are checked before the standard hype_modules walk.
    /// This allows for custom module directories.
    ///
    /// # Arguments
    /// * `path` - The path to add to the search paths
    ///
    /// # Examples
    /// ```ignore
    /// let mut resolver = ModuleResolver::new(PathBuf::from("."));
    /// resolver.add_search_path(PathBuf::from("/custom/modules"));
    /// ```
    pub fn add_search_path(&mut self, path: PathBuf) {
        if !self.search_paths.contains(&path) {
            self.search_paths.push(path);
        }
    }

    /// Get all current search paths.
    ///
    /// # Returns
    /// A reference to the vector of search paths
    pub fn get_search_paths(&self) -> &[PathBuf] {
        &self.search_paths
    }

    /// Expand tilde (~) in paths to the home directory.
    ///
    /// Handles cross-platform path expansion for Unix-like systems (Linux, macOS)
    /// and Windows.
    ///
    /// # Arguments
    /// * `path_str` - The path string that may contain a tilde
    ///
    /// # Errors
    /// Returns an error if the home directory cannot be determined
    ///
    /// # Examples
    /// ```ignore
    /// let resolver = ModuleResolver::new(PathBuf::from("."));
    /// let path = resolver.expand_tilde("~/.config")?;
    /// // On Unix: /home/user/.config
    /// // On Windows: C:\Users\user\.config
    /// ```
    fn expand_tilde(&self, path_str: &str) -> Result<PathBuf, HypeError> {
        if path_str.starts_with('~') {
            let home = dirs_home()?;
            Ok(home.join(&path_str[2..]))
        } else {
            Ok(PathBuf::from(path_str))
        }
    }
}

/// Get the user's home directory in a cross-platform way.
fn dirs_home() -> Result<PathBuf, HypeError> {
    #[cfg(target_os = "windows")]
    {
        env::var("USERPROFILE").map(PathBuf::from).map_err(|_| {
            HypeError::Execution(
                "Failed to determine home directory (USERPROFILE not set)".to_string(),
            )
        })
    }

    #[cfg(not(target_os = "windows"))]
    {
        env::var("HOME").map(PathBuf::from).map_err(|_| {
            HypeError::Execution("Failed to determine home directory (HOME not set)".to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_resolve_builtin_fs() {
        let resolver = ModuleResolver::new(PathBuf::from("."));
        let result = resolver.resolve("fs");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("builtin://fs"));
    }

    #[test]
    fn test_resolve_builtin_path() {
        let resolver = ModuleResolver::new(PathBuf::from("."));
        let result = resolver.resolve("path");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("builtin://path"));
    }

    #[test]
    fn test_resolve_builtin_events() {
        let resolver = ModuleResolver::new(PathBuf::from("."));
        let result = resolver.resolve("events");
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolve_builtin_util() {
        let resolver = ModuleResolver::new(PathBuf::from("."));
        let result = resolver.resolve("util");
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolve_builtin_table() {
        let resolver = ModuleResolver::new(PathBuf::from("."));
        let result = resolver.resolve("table");
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolve_hype_modules() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let hype_modules = temp_path.join("hype_modules");
        let test_module = hype_modules.join("test-module");
        fs::create_dir_all(&test_module).unwrap();

        let resolver = ModuleResolver::new(temp_path.to_path_buf());
        let result = resolver.resolve("test-module");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("test-module"));
    }

    #[test]
    fn test_resolve_walking_up_directories() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let hype_modules = temp_path.join("hype_modules");
        let test_module = hype_modules.join("shared-lib");
        fs::create_dir_all(&test_module).unwrap();

        let nested_dir = temp_path.join("src").join("nested");
        fs::create_dir_all(&nested_dir).unwrap();

        let resolver = ModuleResolver::new(nested_dir.clone());
        let result = resolver.resolve("shared-lib");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("shared-lib"));
    }

    #[test]
    fn test_module_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let resolver = ModuleResolver::new(temp_dir.path().to_path_buf());
        let result = resolver.resolve("nonexistent-module");
        assert!(result.is_err());
    }

    #[test]
    fn test_is_builtin_true() {
        let resolver = ModuleResolver::new(PathBuf::from("."));
        assert!(resolver.is_builtin("fs"));
        assert!(resolver.is_builtin("path"));
        assert!(resolver.is_builtin("events"));
        assert!(resolver.is_builtin("util"));
        assert!(resolver.is_builtin("table"));
    }

    #[test]
    fn test_is_builtin_false() {
        let resolver = ModuleResolver::new(PathBuf::from("."));
        assert!(!resolver.is_builtin("my-module"));
        assert!(!resolver.is_builtin("custom-lib"));
        assert!(!resolver.is_builtin("local-module"));
    }

    #[test]
    fn test_get_builtin_path() {
        let resolver = ModuleResolver::new(PathBuf::from("."));
        let path = resolver.get_builtin_path("fs");
        assert!(path.to_string_lossy().contains("builtin://fs"));
    }

    #[test]
    fn test_add_search_path() {
        let mut resolver = ModuleResolver::new(PathBuf::from("."));
        let custom_path = PathBuf::from("/custom/modules");
        resolver.add_search_path(custom_path.clone());
        assert!(resolver
            .get_search_paths()
            .iter()
            .any(|p| p == &custom_path));
    }

    #[test]
    fn test_resolve_from() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let hype_modules = temp_path.join("hype_modules");
        let test_module = hype_modules.join("test-module");
        fs::create_dir_all(&test_module).unwrap();

        let resolver = ModuleResolver::new(PathBuf::from("."));
        let result = resolver.resolve_from(temp_path, "test-module");
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolve_from_builtin() {
        let temp_dir = TempDir::new().unwrap();
        let resolver = ModuleResolver::new(PathBuf::from("."));
        let result = resolver.resolve_from(temp_dir.path(), "fs");
        assert!(result.is_ok());
    }

    #[test]
    fn test_cross_platform_paths() {
        let resolver = ModuleResolver::new(PathBuf::from("."));
        let path = resolver.get_builtin_path("fs");
        let path_str = path.to_string_lossy();
        assert!(path_str.contains("fs"));
    }

    #[test]
    fn test_expand_tilde() {
        let resolver = ModuleResolver::new(PathBuf::from("."));
        let result = resolver.expand_tilde("~/.hype/modules");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(!path.to_string_lossy().contains("~"));
    }

    #[test]
    fn test_get_search_paths() {
        let temp_dir = TempDir::new().unwrap();
        let resolver = ModuleResolver::new(temp_dir.path().to_path_buf());
        let search_paths = resolver.get_search_paths();
        assert!(!search_paths.is_empty());
    }
}
