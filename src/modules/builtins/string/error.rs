use std::fmt;

#[derive(Debug)]
pub enum StringError {
    InvalidUtf8(String),
    InvalidIndex(usize, usize),
    InvalidCount(i64),
    InvalidLength(usize),
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringError::InvalidUtf8(msg) => write!(f, "Invalid UTF-8: {}", msg),
            StringError::InvalidIndex(index, len) => {
                write!(f, "Index {} out of bounds (length: {})", index, len)
            }
            StringError::InvalidCount(count) => {
                write!(f, "Invalid count: {} (must be non-negative)", count)
            }
            StringError::InvalidLength(len) => {
                write!(f, "Invalid length: {} (must be non-negative)", len)
            }
        }
    }
}

impl std::error::Error for StringError {}

impl From<StringError> for crate::error::HypeError {
    fn from(err: StringError) -> Self {
        crate::error::HypeError::Execution(err.to_string())
    }
}
