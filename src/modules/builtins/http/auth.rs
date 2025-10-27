use base64::{engine::general_purpose::STANDARD, Engine as _};

#[derive(Debug, Clone)]
pub enum AuthOption {
    Basic { username: String, password: String },
    Bearer(String),
}

impl AuthOption {
    pub fn to_header_value(&self) -> String {
        match self {
            AuthOption::Basic { username, password } => {
                let credentials = format!("{}:{}", username, password);
                let encoded = STANDARD.encode(credentials.as_bytes());
                format!("Basic {}", encoded)
            }
            AuthOption::Bearer(token) => {
                format!("Bearer {}", token)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_auth_encoding() {
        let auth = AuthOption::Basic {
            username: "user".to_string(),
            password: "pass".to_string(),
        };
        let header = auth.to_header_value();
        assert_eq!(header, "Basic dXNlcjpwYXNz");
    }

    #[test]
    fn test_basic_auth_special_chars() {
        let auth = AuthOption::Basic {
            username: "user@example.com".to_string(),
            password: "p@ss:w0rd!".to_string(),
        };
        let header = auth.to_header_value();
        assert!(header.starts_with("Basic "));

        let encoded_part = &header[6..];
        let decoded = STANDARD.decode(encoded_part).unwrap();
        let decoded_str = String::from_utf8(decoded).unwrap();
        assert_eq!(decoded_str, "user@example.com:p@ss:w0rd!");
    }

    #[test]
    fn test_bearer_token() {
        let auth = AuthOption::Bearer("abc123".to_string());
        let header = auth.to_header_value();
        assert_eq!(header, "Bearer abc123");
    }

    #[test]
    fn test_bearer_token_jwt() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U";
        let auth = AuthOption::Bearer(token.to_string());
        let header = auth.to_header_value();
        assert_eq!(header, format!("Bearer {}", token));
    }
}
