use std::path::{Path, PathBuf};

use crate::error::{Result, ValidationError};

pub struct FileValidator {
    allowed_extensions: Vec<String>,
    require_extension: bool,
    allow_empty: bool,
    allow_binary: bool,
}

impl Default for FileValidator {
    fn default() -> Self {
        Self {
            allowed_extensions: vec!["lua".to_string()],
            require_extension: true,
            allow_empty: false,
            allow_binary: false,
        }
    }
}

impl FileValidator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_allowed_extensions(mut self, extensions: Vec<&str>) -> Self {
        self.allowed_extensions = extensions.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn require_extension(mut self, require: bool) -> Self {
        self.require_extension = require;
        self
    }

    pub fn allow_empty(mut self, allow: bool) -> Self {
        self.allow_empty = allow;
        self
    }

    pub fn allow_binary(mut self, allow: bool) -> Self {
        self.allow_binary = allow;
        self
    }

    pub fn validate(&self, path: &Path) -> Result<()> {
        self.validate_path(path)?;
        self.validate_extension(path)?;
        self.validate_file_content(path)?;
        Ok(())
    }

    fn validate_path(&self, path: &Path) -> Result<()> {
        if path.as_os_str().is_empty() {
            return Err(ValidationError::InvalidPath(
                PathBuf::from(path),
                "Path is empty".to_string(),
            )
            .into());
        }

        // Check for relative path components that could be problematic
        let path_str = path.to_string_lossy();
        if path_str.contains("..") {
            return Err(ValidationError::InvalidPath(
                PathBuf::from(path),
                "Path contains parent directory components (..)".to_string(),
            )
            .into());
        }

        Ok(())
    }

    fn validate_extension(&self, path: &Path) -> Result<()> {
        if !self.require_extension {
            return Ok(());
        }

        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| {
                ValidationError::InvalidExtension(PathBuf::from(path), "none".to_string())
            })?;

        if !self.allowed_extensions.contains(&extension.to_string()) {
            return Err(ValidationError::InvalidExtension(
                PathBuf::from(path),
                extension.to_string(),
            )
            .into());
        }

        Ok(())
    }

    fn validate_file_content(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(ValidationError::InvalidPath(
                PathBuf::from(path),
                "File does not exist".to_string(),
            )
            .into());
        }

        if !path.is_file() {
            return Err(ValidationError::InvalidPath(
                PathBuf::from(path),
                "Path is not a file".to_string(),
            )
            .into());
        }

        // Check file size
        let metadata = std::fs::metadata(path).map_err(|e| {
            ValidationError::InvalidPath(
                PathBuf::from(path),
                format!("Failed to read metadata: {}", e),
            )
        })?;

        let file_size = metadata.len();
        if file_size == 0 && !self.allow_empty {
            return Err(ValidationError::EmptyFile(PathBuf::from(path)).into());
        }

        // Check if file is binary (only if not allowed)
        if !self.allow_binary && file_size > 0 {
            self.check_if_binary(path)?;
        }

        Ok(())
    }

    fn check_if_binary(&self, path: &Path) -> Result<()> {
        use std::io::Read;

        let mut file = std::fs::File::open(path).map_err(|e| {
            ValidationError::InvalidPath(PathBuf::from(path), format!("Failed to open file: {}", e))
        })?;

        let mut buffer = [0; 512];
        let bytes_read = file.read(&mut buffer).map_err(|e| {
            ValidationError::InvalidPath(PathBuf::from(path), format!("Failed to read file: {}", e))
        })?;

        if bytes_read == 0 {
            return Ok(()); // Empty file handled elsewhere
        }

        // Check for UTF-8 validity
        match std::str::from_utf8(&buffer[..bytes_read]) {
            Ok(_) => Ok(()), // Valid UTF-8, likely text
            Err(_) => {
                // Not valid UTF-8, check for other indicators of binary
                let null_count = buffer[..bytes_read].iter().filter(|&&b| b == 0).count();
                if null_count > 0 {
                    return Err(ValidationError::BinaryFile(PathBuf::from(path)).into());
                }

                // Check for high ratio of non-printable ASCII
                let non_printable = buffer[..bytes_read]
                    .iter()
                    .filter(|&&b| b < 32 && b != b'\t' && b != b'\n' && b != b'\r')
                    .count();

                let ratio = non_printable as f64 / bytes_read as f64;
                if ratio > 0.3 {
                    return Err(ValidationError::BinaryFile(PathBuf::from(path)).into());
                }

                Ok(())
            }
        }
    }
}

pub fn validate_lua_file(path: &Path) -> Result<()> {
    FileValidator::new()
        .with_allowed_extensions(vec!["lua"])
        .require_extension(true)
        .allow_empty(false)
        .allow_binary(false)
        .validate(path)
}
