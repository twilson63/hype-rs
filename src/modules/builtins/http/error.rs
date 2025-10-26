use std::fmt;

#[derive(Debug)]
pub enum HttpError {
    NetworkError(String),
    TimeoutError,
    InvalidUrl(String),
    RequestError(String),
    ResponseError(u16, String),
    JsonParseError(String),
    RuntimeError(String),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            HttpError::TimeoutError => write!(f, "Request timeout"),
            HttpError::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
            HttpError::RequestError(msg) => write!(f, "Request error: {}", msg),
            HttpError::ResponseError(status, msg) => write!(f, "HTTP {} {}", status, msg),
            HttpError::JsonParseError(msg) => write!(f, "JSON parse error: {}", msg),
            HttpError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl std::error::Error for HttpError {}

#[cfg(feature = "http")]
impl From<reqwest::Error> for HttpError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            HttpError::TimeoutError
        } else if err.is_request() {
            HttpError::RequestError(err.to_string())
        } else {
            HttpError::NetworkError(err.to_string())
        }
    }
}

impl From<serde_json::Error> for HttpError {
    fn from(err: serde_json::Error) -> Self {
        HttpError::JsonParseError(err.to_string())
    }
}

impl From<std::io::Error> for HttpError {
    fn from(err: std::io::Error) -> Self {
        HttpError::NetworkError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = HttpError::NetworkError("connection failed".to_string());
        assert_eq!(err.to_string(), "Network error: connection failed");

        let err = HttpError::TimeoutError;
        assert_eq!(err.to_string(), "Request timeout");

        let err = HttpError::InvalidUrl("not a url".to_string());
        assert_eq!(err.to_string(), "Invalid URL: not a url");
    }

    #[test]
    fn test_json_error_conversion() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json");
        assert!(json_err.is_err());
        
        let http_err: HttpError = json_err.unwrap_err().into();
        match http_err {
            HttpError::JsonParseError(_) => (),
            _ => panic!("Expected JsonParseError"),
        }
    }
}
