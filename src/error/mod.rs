use mlua::prelude::LuaError;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum HypeError {
    Lua(String),
    Io(std::io::Error),
    Config(String),
    Execution(String),
    File(FileError),
    Validation(ValidationError),
    StateManagement(String),
    Security(String),
}

#[derive(Debug)]
pub enum FileError {
    NotFound(PathBuf),
    PermissionDenied(PathBuf),
    NotAFile(PathBuf),
    TooLarge(PathBuf, u64),
    InvalidEncoding(PathBuf, std::string::FromUtf8Error),
    SymlinkLoop(PathBuf),
    Other(PathBuf, String),
}

#[derive(Debug)]
pub enum ValidationError {
    InvalidExtension(PathBuf, String),
    EmptyFile(PathBuf),
    BinaryFile(PathBuf),
    PathTooLong(PathBuf),
    InvalidPath(PathBuf, String),
}

impl fmt::Display for HypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HypeError::Lua(msg) => write!(f, "Lua error: {}", msg),
            HypeError::Io(err) => write!(f, "IO error: {}", err),
            HypeError::Config(msg) => write!(f, "Configuration error: {}", msg),
            HypeError::Execution(msg) => write!(f, "Execution error: {}", msg),
            HypeError::File(err) => write!(f, "File error: {}", err),
            HypeError::Validation(err) => write!(f, "Validation error: {}", err),
            HypeError::StateManagement(msg) => write!(f, "State management error: {}", msg),
            HypeError::Security(msg) => write!(f, "Security error: {}", msg),
        }
    }
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileError::NotFound(path) => {
                write!(f, "File not found: {}", path.display())
            }
            FileError::PermissionDenied(path) => {
                write!(f, "Permission denied: {}", path.display())
            }
            FileError::NotAFile(path) => {
                write!(f, "Path is not a file: {}", path.display())
            }
            FileError::TooLarge(path, size) => {
                write!(f, "File too large ({} bytes): {}", size, path.display())
            }
            FileError::InvalidEncoding(path, _) => {
                write!(f, "Invalid file encoding: {}", path.display())
            }
            FileError::SymlinkLoop(path) => {
                write!(f, "Symbolic link loop detected: {}", path.display())
            }
            FileError::Other(path, msg) => {
                write!(f, "File error ({}): {}", path.display(), msg)
            }
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::InvalidExtension(path, ext) => {
                write!(
                    f,
                    "Invalid file extension '.{}' for file: {}",
                    ext,
                    path.display()
                )
            }
            ValidationError::EmptyFile(path) => {
                write!(f, "File is empty: {}", path.display())
            }
            ValidationError::BinaryFile(path) => {
                write!(f, "File appears to be binary: {}", path.display())
            }
            ValidationError::PathTooLong(path) => {
                write!(f, "Path too long: {}", path.display())
            }
            ValidationError::InvalidPath(path, reason) => {
                write!(f, "Invalid path ({}): {}", reason, path.display())
            }
        }
    }
}

impl std::error::Error for HypeError {}
impl std::error::Error for FileError {}
impl std::error::Error for ValidationError {}

impl From<std::io::Error> for HypeError {
    fn from(err: std::io::Error) -> Self {
        HypeError::Io(err)
    }
}

impl From<FileError> for HypeError {
    fn from(err: FileError) -> Self {
        HypeError::File(err)
    }
}

impl From<ValidationError> for HypeError {
    fn from(err: ValidationError) -> Self {
        HypeError::Validation(err)
    }
}

impl From<LuaError> for HypeError {
    fn from(err: LuaError) -> Self {
        HypeError::Lua(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, HypeError>;
