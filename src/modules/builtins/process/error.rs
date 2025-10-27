use std::fmt;

#[derive(Debug)]
pub enum ProcessError {
    IoError(String),
    InvalidPath(String),
    InvalidExitCode,
    PermissionDenied(String),
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessError::IoError(msg) => write!(f, "Process I/O error: {}", msg),
            ProcessError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
            ProcessError::InvalidExitCode => write!(f, "Invalid exit code (must be 0-255)"),
            ProcessError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
        }
    }
}

impl std::error::Error for ProcessError {}

impl From<std::io::Error> for ProcessError {
    fn from(err: std::io::Error) -> Self {
        ProcessError::IoError(err.to_string())
    }
}
