use std::fmt;

#[derive(Debug)]
pub enum CryptoError {
    HashError(String),
    EncodeError(String),
    DecodeError(String),
    RandomError(String),
    BcryptError(String),
    InvalidAlgorithm(String),
    InvalidInput(String),
    FileError(String),
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::HashError(msg) => write!(f, "Hash error: {}", msg),
            CryptoError::EncodeError(msg) => write!(f, "Encode error: {}", msg),
            CryptoError::DecodeError(msg) => write!(f, "Decode error: {}", msg),
            CryptoError::RandomError(msg) => write!(f, "Random generation error: {}", msg),
            CryptoError::BcryptError(msg) => write!(f, "Bcrypt error: {}", msg),
            CryptoError::InvalidAlgorithm(msg) => write!(f, "Invalid algorithm: {}", msg),
            CryptoError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CryptoError::FileError(msg) => write!(f, "File error: {}", msg),
        }
    }
}

impl std::error::Error for CryptoError {}

impl From<CryptoError> for crate::error::HypeError {
    fn from(err: CryptoError) -> Self {
        crate::error::HypeError::Execution(err.to_string())
    }
}
