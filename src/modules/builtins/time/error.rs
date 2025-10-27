use std::fmt;

#[derive(Debug)]
pub enum TimeError {
    ParseError(String),
    FormatError(String),
    InvalidTimestamp(i64),
    InvalidDuration(i64),
}

impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeError::ParseError(msg) => write!(f, "Failed to parse time: {}", msg),
            TimeError::FormatError(msg) => write!(f, "Failed to format time: {}", msg),
            TimeError::InvalidTimestamp(ts) => write!(f, "Invalid timestamp: {}", ts),
            TimeError::InvalidDuration(dur) => write!(f, "Invalid duration: {}", dur),
        }
    }
}

impl std::error::Error for TimeError {}

impl From<TimeError> for crate::error::HypeError {
    fn from(err: TimeError) -> Self {
        crate::error::HypeError::Execution(err.to_string())
    }
}
