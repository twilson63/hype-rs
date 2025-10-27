use std::fmt;

#[derive(Debug)]
pub enum QueryStringError {
    ParseError(String),
    EncodeError(String),
    InvalidUtf8(String),
}

impl fmt::Display for QueryStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryStringError::ParseError(msg) => write!(f, "Failed to parse query string: {}", msg),
            QueryStringError::EncodeError(msg) => {
                write!(f, "Failed to encode query string: {}", msg)
            }
            QueryStringError::InvalidUtf8(msg) => write!(f, "Invalid UTF-8: {}", msg),
        }
    }
}

impl std::error::Error for QueryStringError {}

impl From<QueryStringError> for crate::error::HypeError {
    fn from(err: QueryStringError) -> Self {
        crate::error::HypeError::Execution(err.to_string())
    }
}
