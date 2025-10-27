use std::fmt;

#[derive(Debug)]
pub enum JsonError {
    SerializationError(String),
    DeserializationError(String),
    InvalidUtf8,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonError::SerializationError(msg) => write!(f, "JSON serialization error: {}", msg),
            JsonError::DeserializationError(msg) => {
                write!(f, "JSON deserialization error: {}", msg)
            }
            JsonError::InvalidUtf8 => write!(f, "Invalid UTF-8 in JSON string"),
        }
    }
}

impl std::error::Error for JsonError {}

impl From<serde_json::Error> for JsonError {
    fn from(err: serde_json::Error) -> Self {
        if err.is_syntax() || err.is_data() || err.is_eof() {
            JsonError::DeserializationError(err.to_string())
        } else {
            JsonError::SerializationError(err.to_string())
        }
    }
}
