use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use super::error::ModuleError;
use crate::error::HypeError;

/// Semantic version pattern: major.minor.patch
const VERSION_PATTERN: &str = r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$";

/// Valid module name characters: alphanumeric, hyphen, underscore
const NAME_PATTERN: &str = r"^[a-zA-Z0-9_-]{1,255}$";

/// Module manifest structure parsed from hype.json or similar.
///
/// Defines metadata about a module including name, version, entry point,
/// and dependencies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypeManifest {
    /// Module name (required)
    pub name: String,
    /// Module version in semver format (required)
    pub version: String,
    /// Optional module description
    pub description: Option<String>,
    /// Optional main entry point (file or function)
    pub main: Option<String>,
    /// Optional list of dependencies
    pub dependencies: Option<Vec<String>>,
}

impl HypeManifest {
    /// Create a new manifest with required fields.
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            description: None,
            main: None,
            dependencies: None,
        }
    }

    /// Set the module description.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set the main entry point.
    pub fn with_main(mut self, main: String) -> Self {
        self.main = Some(main);
        self
    }

    /// Set module dependencies.
    pub fn with_dependencies(mut self, deps: Vec<String>) -> Self {
        self.dependencies = Some(deps);
        self
    }

    /// Load a manifest from a file path.
    ///
    /// Reads the file and parses it as JSON. Returns an error if the file
    /// cannot be read or if the JSON is invalid.
    pub fn load(path: &Path) -> Result<Self, HypeError> {
        if !path.exists() {
            return Err(HypeError::Execution(
                ModuleError::ManifestNotFound(path.to_path_buf()).to_string(),
            ));
        }

        let content = fs::read_to_string(path).map_err(|e| {
            HypeError::Execution(
                ModuleError::ManifestReadError(path.to_path_buf(), e.to_string()).to_string(),
            )
        })?;

        let manifest: HypeManifest = serde_json::from_str(&content).map_err(|e| {
            HypeError::Execution(
                ModuleError::ManifestParseError {
                    path: path.to_path_buf(),
                    reason: e.to_string(),
                }
                .to_string(),
            )
        })?;

        Ok(manifest)
    }

    /// Validate the manifest structure and field formats.
    ///
    /// Checks:
    /// - Name is valid (alphanumeric, hyphens, underscores, 1-255 chars)
    /// - Version follows semantic versioning
    /// - Main entry point (if present) is non-empty
    /// - Dependencies (if present) are valid
    pub fn validate(&self) -> Result<(), HypeError> {
        self.validate_name()?;
        self.validate_version()?;
        self.validate_main()?;
        self.validate_dependencies()?;

        Ok(())
    }

    /// Validate module name format.
    fn validate_name(&self) -> Result<(), HypeError> {
        let name_re = Regex::new(NAME_PATTERN).unwrap();

        if !name_re.is_match(&self.name) {
            return Err(HypeError::Execution(
                ModuleError::InvalidModuleName(self.name.clone()).to_string(),
            ));
        }

        Ok(())
    }

    /// Validate module version format (semver).
    fn validate_version(&self) -> Result<(), HypeError> {
        let version_re = Regex::new(VERSION_PATTERN).unwrap();

        if !version_re.is_match(&self.version) {
            return Err(HypeError::Execution(
                ModuleError::InvalidModuleVersion(self.version.clone()).to_string(),
            ));
        }

        Ok(())
    }

    /// Validate main entry point if present.
    fn validate_main(&self) -> Result<(), HypeError> {
        if let Some(main) = &self.main {
            if main.trim().is_empty() {
                return Err(HypeError::Execution(
                    ModuleError::InvalidManifest {
                        reason: "main entry point cannot be empty".to_string(),
                    }
                    .to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Validate dependencies if present.
    fn validate_dependencies(&self) -> Result<(), HypeError> {
        if let Some(deps) = &self.dependencies {
            let name_re = Regex::new(NAME_PATTERN).unwrap();

            for dep in deps {
                if !name_re.is_match(dep) {
                    return Err(HypeError::Execution(
                        ModuleError::InvalidManifest {
                            reason: format!("invalid dependency name: '{}'", dep),
                        }
                        .to_string(),
                    ));
                }
            }
        }

        Ok(())
    }

    /// Get the default main entry point if not specified.
    ///
    /// Returns "index.lua" as the convention when main is not set.
    pub fn default_main(&self) -> String {
        self.main.clone().unwrap_or_else(|| "index.lua".to_string())
    }

    /// Save the manifest to a file as JSON.
    pub fn save(&self, path: &Path) -> Result<(), HypeError> {
        let json = serde_json::to_string_pretty(self).map_err(|e| {
            HypeError::Execution(
                ModuleError::ManifestParseError {
                    path: path.to_path_buf(),
                    reason: format!("Failed to serialize manifest: {}", e),
                }
                .to_string(),
            )
        })?;

        fs::write(path, json).map_err(|e| {
            HypeError::Execution(
                ModuleError::ManifestReadError(path.to_path_buf(), e.to_string()).to_string(),
            )
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_manifest_new() {
        let manifest = HypeManifest::new("my_module".to_string(), "1.0.0".to_string());
        assert_eq!(manifest.name, "my_module");
        assert_eq!(manifest.version, "1.0.0");
        assert!(manifest.description.is_none());
        assert!(manifest.main.is_none());
    }

    #[test]
    fn test_manifest_builder() {
        let manifest = HypeManifest::new("test".to_string(), "1.0.0".to_string())
            .with_description("A test module".to_string())
            .with_main("main.lua".to_string())
            .with_dependencies(vec!["dep1".to_string(), "dep2".to_string()]);

        assert_eq!(manifest.name, "test");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.description, Some("A test module".to_string()));
        assert_eq!(manifest.main, Some("main.lua".to_string()));
        assert_eq!(
            manifest.dependencies,
            Some(vec!["dep1".to_string(), "dep2".to_string()])
        );
    }

    #[test]
    fn test_default_main() {
        let manifest_without_main = HypeManifest::new("test".to_string(), "1.0.0".to_string());
        assert_eq!(manifest_without_main.default_main(), "index.lua");

        let manifest_with_main = HypeManifest::new("test".to_string(), "1.0.0".to_string())
            .with_main("custom.lua".to_string());
        assert_eq!(manifest_with_main.default_main(), "custom.lua");
    }

    #[test]
    fn test_validate_name_valid() {
        let manifest = HypeManifest::new("valid_module".to_string(), "1.0.0".to_string());
        assert!(manifest.validate_name().is_ok());

        let manifest2 = HypeManifest::new("valid-module".to_string(), "1.0.0".to_string());
        assert!(manifest2.validate_name().is_ok());

        let manifest3 = HypeManifest::new("ValidModule123".to_string(), "1.0.0".to_string());
        assert!(manifest3.validate_name().is_ok());
    }

    #[test]
    fn test_validate_name_invalid() {
        let manifest = HypeManifest::new("invalid!name".to_string(), "1.0.0".to_string());
        assert!(manifest.validate_name().is_err());

        let manifest2 = HypeManifest::new("invalid@name".to_string(), "1.0.0".to_string());
        assert!(manifest2.validate_name().is_err());

        let manifest3 = HypeManifest::new("".to_string(), "1.0.0".to_string());
        assert!(manifest3.validate_name().is_err());
    }

    #[test]
    fn test_validate_version_valid() {
        let manifest = HypeManifest::new("test".to_string(), "1.0.0".to_string());
        assert!(manifest.validate_version().is_ok());

        let manifest2 = HypeManifest::new("test".to_string(), "0.0.1".to_string());
        assert!(manifest2.validate_version().is_ok());

        let manifest3 = HypeManifest::new("test".to_string(), "1.2.3-alpha".to_string());
        assert!(manifest3.validate_version().is_ok());

        let manifest4 = HypeManifest::new("test".to_string(), "1.2.3+build.123".to_string());
        assert!(manifest4.validate_version().is_ok());
    }

    #[test]
    fn test_validate_version_invalid() {
        let manifest = HypeManifest::new("test".to_string(), "1".to_string());
        assert!(manifest.validate_version().is_err());

        let manifest2 = HypeManifest::new("test".to_string(), "v1.0.0".to_string());
        assert!(manifest2.validate_version().is_err());

        let manifest3 = HypeManifest::new("test".to_string(), "1.0.0.0".to_string());
        assert!(manifest3.validate_version().is_err());
    }

    #[test]
    fn test_validate_main() {
        let manifest = HypeManifest::new("test".to_string(), "1.0.0".to_string())
            .with_main("main.lua".to_string());
        assert!(manifest.validate_main().is_ok());

        let manifest2 =
            HypeManifest::new("test".to_string(), "1.0.0".to_string()).with_main("  ".to_string());
        assert!(manifest2.validate_main().is_err());
    }

    #[test]
    fn test_validate_dependencies() {
        let manifest = HypeManifest::new("test".to_string(), "1.0.0".to_string())
            .with_dependencies(vec!["dep1".to_string(), "dep2".to_string()]);
        assert!(manifest.validate_dependencies().is_ok());

        let manifest2 = HypeManifest::new("test".to_string(), "1.0.0".to_string())
            .with_dependencies(vec!["invalid!dep".to_string()]);
        assert!(manifest2.validate_dependencies().is_err());
    }

    #[test]
    fn test_validate_full() {
        let manifest = HypeManifest::new("valid_module".to_string(), "1.0.0".to_string())
            .with_description("A valid module".to_string())
            .with_main("main.lua".to_string())
            .with_dependencies(vec!["dep1".to_string()]);

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_load_and_save() {
        let manifest = HypeManifest::new("test_module".to_string(), "1.0.0".to_string())
            .with_description("Test description".to_string())
            .with_main("main.lua".to_string());

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        manifest.save(&path).unwrap();

        let loaded = HypeManifest::load(&path).unwrap();
        assert_eq!(loaded.name, "test_module");
        assert_eq!(loaded.version, "1.0.0");
        assert_eq!(loaded.description, Some("Test description".to_string()));
        assert_eq!(loaded.main, Some("main.lua".to_string()));
    }

    #[test]
    fn test_load_nonexistent_file() {
        let path = Path::new("/nonexistent/path/hype.json");
        let result = HypeManifest::load(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_invalid_json() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"{ invalid json }").unwrap();
        temp_file.flush().unwrap();

        let result = HypeManifest::load(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_load_missing_required_fields() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"{}").unwrap();
        temp_file.flush().unwrap();

        let result = HypeManifest::load(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_serialization() {
        let manifest = HypeManifest::new("test".to_string(), "1.0.0".to_string())
            .with_description("Test".to_string())
            .with_dependencies(vec!["dep1".to_string()]);

        let json = serde_json::to_string(&manifest).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("1.0.0"));
        assert!(json.contains("Test"));
        assert!(json.contains("dep1"));
    }

    #[test]
    fn test_manifest_clone() {
        let manifest = HypeManifest::new("test".to_string(), "1.0.0".to_string())
            .with_description("Test".to_string());

        let cloned = manifest.clone();
        assert_eq!(cloned.name, "test");
        assert_eq!(cloned.version, "1.0.0");
        assert_eq!(cloned.description, Some("Test".to_string()));
    }
}
