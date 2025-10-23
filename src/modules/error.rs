use std::fmt;
use std::path::PathBuf;

/// Errors that can occur during module operations.
#[derive(Debug)]
pub enum ModuleError {
    /// Manifest file not found
    ManifestNotFound(PathBuf),
    /// Manifest file could not be read
    ManifestReadError(PathBuf, String),
    /// Manifest JSON parsing failed
    ManifestParseError { path: PathBuf, reason: String },
    /// Manifest validation failed
    InvalidManifest { reason: String },
    /// Module not found in registry
    ModuleNotFound(String),
    /// Invalid module name format
    InvalidModuleName(String),
    /// Invalid module version format
    InvalidModuleVersion(String),
    /// Registry operation failed
    RegistryError(String),
    /// Module already exists
    ModuleAlreadyExists(String),
    /// Lock operation failed on registry
    LockPoisoned,
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModuleError::ManifestNotFound(path) => {
                write!(f, "Manifest not found: {}", path.display())
            }
            ModuleError::ManifestReadError(path, reason) => {
                write!(f, "Failed to read manifest {}: {}", path.display(), reason)
            }
            ModuleError::ManifestParseError { path, reason } => {
                write!(f, "Failed to parse manifest {}: {}", path.display(), reason)
            }
            ModuleError::InvalidManifest { reason } => {
                write!(f, "Invalid manifest: {}", reason)
            }
            ModuleError::ModuleNotFound(name) => {
                write!(f, "Module not found: '{}'", name)
            }
            ModuleError::InvalidModuleName(name) => {
                write!(f, "Invalid module name: '{}'", name)
            }
            ModuleError::InvalidModuleVersion(version) => {
                write!(f, "Invalid module version: '{}'", version)
            }
            ModuleError::RegistryError(msg) => {
                write!(f, "Registry error: {}", msg)
            }
            ModuleError::ModuleAlreadyExists(name) => {
                write!(f, "Module already exists: '{}'", name)
            }
            ModuleError::LockPoisoned => {
                write!(f, "Lock poisoned: concurrent access error")
            }
        }
    }
}

impl std::error::Error for ModuleError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_error_display() {
        let err = ModuleError::ModuleNotFound("test_module".to_string());
        assert!(err.to_string().contains("test_module"));
    }

    #[test]
    fn test_module_error_invalid_name() {
        let err = ModuleError::InvalidModuleName("invalid!name".to_string());
        assert!(err.to_string().contains("invalid!name"));
    }

    #[test]
    fn test_module_error_invalid_version() {
        let err = ModuleError::InvalidModuleVersion("v1.2.3.4.5".to_string());
        assert!(err.to_string().contains("v1.2.3.4.5"));
    }

    #[test]
    fn test_module_error_lock_poisoned() {
        let err = ModuleError::LockPoisoned;
        assert!(err.to_string().contains("Lock poisoned"));
    }

    #[test]
    fn test_module_error_invalid_manifest() {
        let err = ModuleError::InvalidManifest {
            reason: "missing 'name' field".to_string(),
        };
        assert!(err.to_string().contains("missing 'name' field"));
    }
}
