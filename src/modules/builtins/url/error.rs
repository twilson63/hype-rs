use std::fmt;

#[derive(Debug)]
pub enum UrlError {
    ParseError(String),
    InvalidUrl(String),
    InvalidComponent(String),
    MissingComponent(String),
}

impl fmt::Display for UrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UrlError::ParseError(msg) => write!(f, "Failed to parse URL: {}", msg),
            UrlError::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
            UrlError::InvalidComponent(comp) => write!(f, "Invalid URL component: {}", comp),
            UrlError::MissingComponent(comp) => write!(f, "Missing URL component: {}", comp),
        }
    }
}

impl std::error::Error for UrlError {}

impl From<UrlError> for crate::error::HypeError {
    fn from(err: UrlError) -> Self {
        crate::error::HypeError::Execution(err.to_string())
    }
}
