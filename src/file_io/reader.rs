use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use crate::error::{FileError, HypeError, Result};

const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB
const MAX_PATH_LENGTH: usize = 4096;

pub struct FileReader {
    max_file_size: u64,
    allow_binary: bool,
}

impl Default for FileReader {
    fn default() -> Self {
        Self {
            max_file_size: MAX_FILE_SIZE,
            allow_binary: false,
        }
    }
}

impl FileReader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_file_size(mut self, size: u64) -> Self {
        self.max_file_size = size;
        self
    }

    pub fn allow_binary(mut self, allow: bool) -> Self {
        self.allow_binary = allow;
        self
    }

    pub fn read_to_string(&self, path: &Path) -> Result<String> {
        self.validate_path(path)?;

        let canonical_path = self.canonicalize_path(path)?;
        self.check_file_exists(&canonical_path)?;
        self.check_file_permissions(&canonical_path)?;
        self.check_file_size(&canonical_path)?;

        if !self.allow_binary {
            self.check_text_file(&canonical_path)?;
        }

        let content = fs::read_to_string(&canonical_path)
            .map_err(|e| FileError::Other(canonical_path.clone(), e.to_string()))?;

        if content.trim().is_empty() {
            return Err(FileError::Other(
                canonical_path.clone(),
                "File is empty or contains only whitespace".to_string(),
            )
            .into());
        }

        Ok(content)
    }

    pub fn read_bytes(&self, path: &Path) -> Result<Vec<u8>> {
        self.validate_path(path)?;

        let canonical_path = self.canonicalize_path(path)?;
        self.check_file_exists(&canonical_path)?;
        self.check_file_permissions(&canonical_path)?;
        self.check_file_size(&canonical_path)?;

        fs::read(&canonical_path)
            .map_err(|e| FileError::Other(canonical_path.clone(), e.to_string()).into())
    }

    fn validate_path(&self, path: &Path) -> Result<()> {
        if path.as_os_str().is_empty() {
            return Err(FileError::Other(PathBuf::from(path), "Path is empty".to_string()).into());
        }

        let path_str = path.to_string_lossy();
        if path_str.len() > MAX_PATH_LENGTH {
            return Err(FileError::Other(
                PathBuf::from(path),
                "Path exceeds maximum length".to_string(),
            )
            .into());
        }

        // Check for invalid characters in path
        if path_str.contains('\0') {
            return Err(FileError::Other(
                PathBuf::from(path),
                "Path contains null character".to_string(),
            )
            .into());
        }

        Ok(())
    }

    fn canonicalize_path(&self, path: &Path) -> Result<PathBuf> {
        path.canonicalize()
            .map_err(|e| match e.kind() {
                io::ErrorKind::NotFound => FileError::NotFound(PathBuf::from(path)),
                io::ErrorKind::PermissionDenied => FileError::PermissionDenied(PathBuf::from(path)),
                _ => FileError::Other(
                    PathBuf::from(path),
                    format!("Failed to canonicalize path: {}", e),
                ),
            })
            .map_err(HypeError::from)
    }

    fn check_file_exists(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(FileError::NotFound(PathBuf::from(path)).into());
        }
        Ok(())
    }

    fn check_file_permissions(&self, path: &Path) -> Result<()> {
        if !path.is_file() {
            return Err(FileError::NotAFile(PathBuf::from(path)).into());
        }

        // Check read permissions by attempting to open the file
        match fs::File::open(path) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    Err(FileError::PermissionDenied(PathBuf::from(path)).into())
                }
                _ => Err(FileError::Other(PathBuf::from(path), e.to_string()).into()),
            },
        }
    }

    fn check_file_size(&self, path: &Path) -> Result<()> {
        let metadata =
            fs::metadata(path).map_err(|e| FileError::Other(PathBuf::from(path), e.to_string()))?;

        let file_size = metadata.len();
        if file_size > self.max_file_size {
            return Err(FileError::TooLarge(PathBuf::from(path), file_size).into());
        }

        if file_size == 0 {
            return Err(FileError::Other(PathBuf::from(path), "File is empty".to_string()).into());
        }

        Ok(())
    }

    fn check_text_file(&self, path: &Path) -> Result<()> {
        let mut file = fs::File::open(path)
            .map_err(|e| FileError::Other(PathBuf::from(path), e.to_string()))?;

        let mut buffer = [0; 1024];
        let bytes_read = file
            .read(&mut buffer)
            .map_err(|e| FileError::Other(PathBuf::from(path), e.to_string()))?;

        if bytes_read == 0 {
            return Err(FileError::Other(PathBuf::from(path), "File is empty".to_string()).into());
        }

        // Check for null bytes (common in binary files)
        if buffer[..bytes_read].contains(&0) {
            return Err(FileError::Other(
                PathBuf::from(path),
                "File appears to be binary (contains null bytes)".to_string(),
            )
            .into());
        }

        // Check for high proportion of non-printable characters
        let non_printable = buffer[..bytes_read]
            .iter()
            .filter(|&&b| b < 32 && b != b'\t' && b != b'\n' && b != b'\r')
            .count();

        let ratio = non_printable as f64 / bytes_read as f64;
        if ratio > 0.3 {
            return Err(FileError::Other(
                PathBuf::from(path),
                "File appears to be binary (high ratio of non-printable characters)".to_string(),
            )
            .into());
        }

        Ok(())
    }
}

pub fn read_lua_script(path: &Path) -> Result<String> {
    FileReader::new()
        .with_max_file_size(MAX_FILE_SIZE)
        .allow_binary(false)
        .read_to_string(path)
}
