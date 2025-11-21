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
/// 2. Relative paths (./ or ../)
/// 3. Absolute paths (if allowed)
/// 4. hype_modules directories (walk up from current directory)
/// 5. Home directory modules (~/.hype/modules)
/// 6. Returns error if not found
///
/// Handles cross-platform paths, tilde expansion, and relative vs absolute paths.
pub struct ModuleResolver {
    root_dir: PathBuf,
    search_paths: Vec<PathBuf>,
    allow_absolute_paths: bool,
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
            allow_absolute_paths: false,
        }
    }

    /// Resolve a module identifier to its full filesystem path.
    ///
    /// Uses the Hype-RS module resolution algorithm:
    /// 1. Check if module is a built-in
    /// 2. Handle relative paths (./ or ../)
    /// 3. Handle absolute paths (if allowed)
    /// 4. Walk up directory tree looking for hype_modules/module_id
    /// 5. Check ~/.hype/modules/module_id
    /// 6. Try direct file/directory lookup from root (fallback)
    /// 7. Return ModuleNotFound error with attempted paths
    ///
    /// # Arguments
    /// * `module_id` - The module identifier to resolve (e.g., "fs", "my-module", "./lib/utils")
    ///
    /// # Errors
    /// Returns `ModuleError::ModuleNotFoundWithPaths` if the module cannot be resolved
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

        if module_id.starts_with("./") || module_id.starts_with("../") {
            return self.resolve_relative(module_id);
        }

        if Path::new(module_id).is_absolute() {
            return self.resolve_absolute(module_id);
        }

        // Try to resolve in hype_modules first
        match self.resolve_module_paths(module_id) {
            Ok(path) => Ok(path),
            Err(_) => {
                // Fallback: try direct file/directory lookup from root
                if let Some(resolved) = self.try_direct_file_fallback(module_id) {
                    Ok(resolved)
                } else {
                    self.resolve_module_paths(module_id)
                }
            }
        }
    }

    /// Resolve module paths by searching in hype_modules with various extensions.
    ///
    /// Search order for each search path:
    /// 1. {module_id}.lua
    /// 2. {module_id}/index.lua
    /// 3. {module_id}/init.lua
    /// 4. {module_id} (directory)
    ///
    /// # Arguments
    /// * `module_id` - The module identifier to resolve
    ///
    /// # Errors
    /// Returns `ModuleError::ModuleNotFoundWithPaths` with all attempted paths
    fn resolve_module_paths(&self, module_id: &str) -> Result<PathBuf, HypeError> {
        let mut attempted_paths = Vec::new();

        for search_path in &self.search_paths {
            let base = search_path.join("hype_modules");
            if let Some(resolved) =
                self.resolve_with_extensions(&base, module_id, &mut attempted_paths)
            {
                return Ok(resolved);
            }
        }

        let home_modules = self.expand_tilde("~/.hype/modules")?;
        if let Some(resolved) =
            self.resolve_with_extensions(&home_modules, module_id, &mut attempted_paths)
        {
            return Ok(resolved);
        }

        Err(HypeError::Execution(
            ModuleError::ModuleNotFoundWithPaths {
                module_id: module_id.to_string(),
                attempted_paths,
            }
            .to_string(),
        ))
    }

    /// Try to resolve a module with various extensions.
    ///
    /// Checks for:
    /// 1. {base}/{module_id}.lua
    /// 2. {base}/{module_id}/index.lua
    /// 3. {base}/{module_id}/init.lua
    /// 4. {base}/{module_id} (directory)
    ///
    /// # Arguments
    /// * `base` - The base directory to search in
    /// * `module_id` - The module identifier
    /// * `attempted_paths` - Vector to collect attempted paths for error reporting
    ///
    /// # Returns
    /// The first matching path, or None if no matches found
    fn resolve_with_extensions(
        &self,
        base: &Path,
        module_id: &str,
        attempted_paths: &mut Vec<PathBuf>,
    ) -> Option<PathBuf> {
        let candidates = vec![
            base.join(format!("{}.lua", module_id)),
            base.join(module_id).join("index.lua"),
            base.join(module_id).join("init.lua"),
            base.join(module_id),
        ];

        for candidate in candidates {
            attempted_paths.push(candidate.clone());
            if candidate.exists() && (candidate.is_file() || candidate.is_dir()) {
                return Some(candidate);
            }
        }

        None
    }

    /// Try to resolve a module as a direct file or directory in root_dir.
    ///
    /// This is a fallback mechanism that allows requiring files/directories
    /// directly without needing them to be in hype_modules/.
    ///
    /// Checks for:
    /// 1. {root_dir}/{module_id}.lua
    /// 2. {root_dir}/{module_id}/index.lua
    /// 3. {root_dir}/{module_id}/init.lua
    /// 4. {root_dir}/{module_id} (directory)
    ///
    /// # Arguments
    /// * `module_id` - The module identifier to resolve
    ///
    /// # Returns
    /// The first matching path, or None if no matches found
    fn try_direct_file_fallback(&self, module_id: &str) -> Option<PathBuf> {
        let candidates = vec![
            self.root_dir.join(format!("{}.lua", module_id)),
            self.root_dir.join(module_id).join("index.lua"),
            self.root_dir.join(module_id).join("init.lua"),
            self.root_dir.join(module_id),
        ];

        for candidate in candidates {
            if candidate.exists() && (candidate.is_file() || candidate.is_dir()) {
                return Some(candidate);
            }
        }

        None
    }

    /// Resolve a relative path starting with ./ or ../
    ///
    /// Resolves from root_dir context and validates paths don't escape
    /// allowed directories.
    ///
    /// # Arguments
    /// * `module_id` - The relative module path (e.g., "./lib/utils" or "../shared/helpers")
    ///
    /// # Errors
    /// Returns `ModuleError::PathTraversal` if path escapes allowed directories
    /// Returns `ModuleError::ModuleNotFoundWithPaths` if the file doesn't exist
    fn resolve_relative(&self, module_id: &str) -> Result<PathBuf, HypeError> {
        let base_path = &self.root_dir;
        let requested_path = base_path.join(module_id);

        let canonical_base = std::fs::canonicalize(base_path).map_err(|e| {
            HypeError::Execution(format!("Failed to canonicalize base path: {}", e))
        })?;

        let mut attempted_paths = Vec::new();

        if let Some(resolved) = self.try_relative_extensions(&requested_path, &mut attempted_paths)
        {
            let canonical = std::fs::canonicalize(&resolved).map_err(|e| {
                HypeError::Execution(format!("Failed to canonicalize resolved path: {}", e))
            })?;

            if !canonical.starts_with(&canonical_base) {
                return Err(HypeError::Execution(
                    ModuleError::PathTraversal {
                        path: canonical,
                        reason: "Path escapes allowed directories".to_string(),
                    }
                    .to_string(),
                ));
            }

            return Ok(canonical);
        }

        Err(HypeError::Execution(
            ModuleError::ModuleNotFoundWithPaths {
                module_id: module_id.to_string(),
                attempted_paths,
            }
            .to_string(),
        ))
    }

    /// Try various extensions for a relative path
    fn try_relative_extensions(
        &self,
        base: &Path,
        attempted_paths: &mut Vec<PathBuf>,
    ) -> Option<PathBuf> {
        let candidates = vec![
            base.with_extension("lua"),
            base.to_path_buf(),
            base.join("index.lua"),
            base.join("init.lua"),
        ];

        for candidate in candidates {
            attempted_paths.push(candidate.clone());
            if candidate.exists() {
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }

        None
    }

    /// Collect attempted paths for relative path resolution errors
    fn collect_relative_attempts(
        &self,
        base: &Path,
        module_id: &str,
        attempted_paths: &mut Vec<PathBuf>,
    ) {
        let requested = base.join(module_id);
        attempted_paths.push(requested.clone());
        attempted_paths.push(requested.with_extension("lua"));
        attempted_paths.push(requested.join("index.lua"));
        attempted_paths.push(requested.join("init.lua"));
    }

    /// Resolve an absolute path (only if explicitly allowed)
    ///
    /// # Arguments
    /// * `module_id` - The absolute path to resolve
    ///
    /// # Errors
    /// Returns `ModuleError::AbsolutePathNotAllowed` if absolute paths are not enabled
    /// Returns `ModuleError::PathTraversal` if path validation fails
    fn resolve_absolute(&self, module_id: &str) -> Result<PathBuf, HypeError> {
        if !self.allow_absolute_paths {
            return Err(HypeError::Execution(
                ModuleError::AbsolutePathNotAllowed(PathBuf::from(module_id)).to_string(),
            ));
        }

        let path = PathBuf::from(module_id);

        if path.exists() && path.is_file() {
            let canonical = std::fs::canonicalize(&path)
                .map_err(|e| HypeError::Execution(format!("Failed to canonicalize path: {}", e)))?;
            return Ok(canonical);
        }

        Err(HypeError::Execution(
            ModuleError::ModuleNotFoundWithPaths {
                module_id: module_id.to_string(),
                attempted_paths: vec![path],
            }
            .to_string(),
        ))
    }

    /// Enable or disable absolute path resolution.
    ///
    /// By default, absolute paths are not allowed for security reasons.
    ///
    /// # Arguments
    /// * `allow` - Whether to allow absolute paths
    pub fn set_allow_absolute_paths(&mut self, allow: bool) {
        self.allow_absolute_paths = allow;
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

    #[test]
    fn test_resolve_with_lua_extension() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let hype_modules = temp_path.join("hype_modules");
        fs::create_dir_all(&hype_modules).unwrap();

        let test_file = hype_modules.join("utils.lua");
        fs::write(&test_file, "-- test module").unwrap();

        let resolver = ModuleResolver::new(temp_path.to_path_buf());
        let result = resolver.resolve("utils");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("utils.lua"));
    }

    #[test]
    fn test_resolve_with_index_lua() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let hype_modules = temp_path.join("hype_modules");
        let test_module = hype_modules.join("my-module");
        fs::create_dir_all(&test_module).unwrap();

        let index_file = test_module.join("index.lua");
        fs::write(&index_file, "-- test index").unwrap();

        let resolver = ModuleResolver::new(temp_path.to_path_buf());
        let result = resolver.resolve("my-module");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("index.lua"));
    }

    #[test]
    fn test_resolve_with_init_lua() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let hype_modules = temp_path.join("hype_modules");
        let test_module = hype_modules.join("lua-module");
        fs::create_dir_all(&test_module).unwrap();

        let init_file = test_module.join("init.lua");
        fs::write(&init_file, "-- test init").unwrap();

        let resolver = ModuleResolver::new(temp_path.to_path_buf());
        let result = resolver.resolve("lua-module");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("init.lua"));
    }

    #[test]
    fn test_resolve_precedence_lua_over_directory() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let hype_modules = temp_path.join("hype_modules");
        fs::create_dir_all(&hype_modules).unwrap();

        let foo_file = hype_modules.join("foo.lua");
        fs::write(&foo_file, "-- foo.lua").unwrap();

        let foo_dir = hype_modules.join("foo");
        fs::create_dir_all(&foo_dir).unwrap();
        let index_file = foo_dir.join("index.lua");
        fs::write(&index_file, "-- foo/index.lua").unwrap();

        let resolver = ModuleResolver::new(temp_path.to_path_buf());
        let result = resolver.resolve("foo");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("foo.lua"));
        assert!(!path.ends_with("index.lua"));
    }

    #[test]
    fn test_resolve_relative_path() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let lib_dir = temp_path.join("lib");
        fs::create_dir_all(&lib_dir).unwrap();

        let utils_file = lib_dir.join("utils.lua");
        fs::write(&utils_file, "-- utils").unwrap();

        let resolver = ModuleResolver::new(temp_path.to_path_buf());
        let result = resolver.resolve("./lib/utils");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("utils.lua"));
    }

    #[test]
    fn test_resolve_parent_relative_path() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let lib_dir = temp_path.join("lib");
        fs::create_dir_all(&lib_dir).unwrap();

        let shared_file = temp_path.join("shared.lua");
        fs::write(&shared_file, "-- shared").unwrap();

        let resolver = ModuleResolver::new(temp_path.to_path_buf());
        let result = resolver.resolve("./lib/../shared");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("shared.lua"));
    }

    #[test]
    fn test_resolve_absolute_path_not_allowed_by_default() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let test_file = temp_path.join("test.lua");
        fs::write(&test_file, "-- test").unwrap();

        let resolver = ModuleResolver::new(temp_path.to_path_buf());
        let abs_path = test_file.to_string_lossy().to_string();
        let result = resolver.resolve(&abs_path);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Absolute paths not allowed"));
    }

    #[test]
    fn test_resolve_absolute_path_when_allowed() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let test_file = temp_path.join("test.lua");
        fs::write(&test_file, "-- test").unwrap();

        let mut resolver = ModuleResolver::new(temp_path.to_path_buf());
        resolver.set_allow_absolute_paths(true);

        let abs_path = test_file.to_string_lossy().to_string();
        let result = resolver.resolve(&abs_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_module_not_found_with_attempted_paths() {
        let temp_dir = TempDir::new().unwrap();
        let resolver = ModuleResolver::new(temp_dir.path().to_path_buf());
        let result = resolver.resolve("nonexistent");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Module not found"));
        assert!(err.contains("Searched in:"));
        assert!(err.contains("nonexistent.lua"));
        assert!(err.contains("index.lua"));
        assert!(err.contains("init.lua"));
    }

    #[test]
    fn test_resolve_relative_path_escaping_prevented() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let outside_file = temp_dir.path().parent().unwrap().join("outside.lua");
        fs::write(&outside_file, "-- outside").unwrap();

        let nested_dir = temp_path.join("nested");
        fs::create_dir_all(&nested_dir).unwrap();

        let resolver = ModuleResolver::new(nested_dir.clone());
        let result = resolver.resolve("../../outside");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Path escapes allowed directories") || err.contains("not found"));
    }

    #[test]
    fn test_direct_file_fallback_lua_file() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let foo_file = temp_path.join("foo.lua");
        fs::write(&foo_file, "return { message = 'hello' }").unwrap();

        let resolver = ModuleResolver::new(temp_path.to_path_buf());
        let result = resolver.resolve("foo");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("foo.lua"));
    }

    #[test]
    fn test_direct_file_fallback_directory_with_index() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let module_dir = temp_path.join("mymod");
        fs::create_dir_all(&module_dir).unwrap();
        let index_file = module_dir.join("index.lua");
        fs::write(&index_file, "return { name = 'mymod' }").unwrap();

        let resolver = ModuleResolver::new(temp_path.to_path_buf());
        let result = resolver.resolve("mymod");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("index.lua"));
    }

    #[test]
    fn test_direct_file_fallback_directory_with_init() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let module_dir = temp_path.join("mylib");
        fs::create_dir_all(&module_dir).unwrap();
        let init_file = module_dir.join("init.lua");
        fs::write(&init_file, "return { version = '1.0' }").unwrap();

        let resolver = ModuleResolver::new(temp_path.to_path_buf());
        let result = resolver.resolve("mylib");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("init.lua"));
    }

    #[test]
    fn test_direct_file_fallback_directory_only() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let module_dir = temp_path.join("emptymod");
        fs::create_dir_all(&module_dir).unwrap();

        let resolver = ModuleResolver::new(temp_path.to_path_buf());
        let result = resolver.resolve("emptymod");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("emptymod"));
    }

    #[test]
    fn test_direct_file_fallback_hype_modules_priority() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create a module in hype_modules
        let hype_modules = temp_path.join("hype_modules");
        let hype_mod = hype_modules.join("utils");
        fs::create_dir_all(&hype_mod).unwrap();
        fs::write(hype_mod.join("index.lua"), "return { from = 'hype_modules' }").unwrap();

        // Create a direct file with the same name
        fs::write(temp_path.join("utils.lua"), "return { from = 'direct' }").unwrap();

        let resolver = ModuleResolver::new(temp_path.to_path_buf());
        let result = resolver.resolve("utils");
        assert!(result.is_ok());
        let path = result.unwrap();
        // hype_modules should take priority
        assert!(path.to_string_lossy().contains("hype_modules"));
    }

    #[test]
    fn test_direct_file_fallback_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let resolver = ModuleResolver::new(temp_dir.path().to_path_buf());
        let result = resolver.resolve("nonexistent");
        assert!(result.is_err());
    }
}
