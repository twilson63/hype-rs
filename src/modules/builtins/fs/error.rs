use std::fmt;

#[derive(Debug)]
pub enum FsError {
    IoError(std::io::Error),
    InvalidPath(String),
    PermissionDenied(String),
    NotFound(String),
    AlreadyExists(String),
    InvalidOperation(String),
}

impl fmt::Display for FsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FsError::IoError(e) => write!(f, "IO error: {}", e),
            FsError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
            FsError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            FsError::NotFound(msg) => write!(f, "Not found: {}", msg),
            FsError::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            FsError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl std::error::Error for FsError {}

impl From<std::io::Error> for FsError {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind;
        match err.kind() {
            ErrorKind::NotFound => FsError::NotFound(err.to_string()),
            ErrorKind::PermissionDenied => FsError::PermissionDenied(err.to_string()),
            ErrorKind::AlreadyExists => FsError::AlreadyExists(err.to_string()),
            _ => FsError::IoError(err),
        }
    }
}
