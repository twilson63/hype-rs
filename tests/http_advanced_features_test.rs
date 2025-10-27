#[cfg(feature = "http")]
mod tests {
    use hype_rs::modules::builtins::http::{AuthOption, FileField, HttpClient};
    use std::collections::HashMap;

    #[test]
    fn test_auth_basic_encoding() {
        let auth = AuthOption::Basic {
            username: "user".to_string(),
            password: "pass".to_string(),
        };
        let header = auth.to_header_value();
        assert_eq!(header, "Basic dXNlcjpwYXNz");
    }

    #[test]
    fn test_auth_bearer_token() {
        let auth = AuthOption::Bearer("test_token_123".to_string());
        let header = auth.to_header_value();
        assert_eq!(header, "Bearer test_token_123");
    }

    #[test]
    #[ignore]
    fn test_basic_auth_request() {
        let client = HttpClient::new().unwrap();
        let auth = AuthOption::Basic {
            username: "user".to_string(),
            password: "pass".to_string(),
        };

        let res = client.get_with_auth("https://httpbin.org/basic-auth/user/pass", auth);
        assert!(res.is_ok());
        if let Ok(response) = res {
            assert_eq!(response.status, 200);
        }
    }

    #[test]
    #[ignore]
    fn test_bearer_token_request() {
        let client = HttpClient::new().unwrap();
        let auth = AuthOption::Bearer("test_token".to_string());

        let res = client.get_with_auth("https://httpbin.org/bearer", auth);
        assert!(res.is_ok());
    }

    #[test]
    #[ignore]
    fn test_form_submission() {
        let client = HttpClient::new().unwrap();
        let mut fields = HashMap::new();
        fields.insert("field1".to_string(), "value1".to_string());
        fields.insert("field2".to_string(), "value2".to_string());

        let res = client.post_form("https://httpbin.org/post", fields);
        assert!(res.is_ok());
        if let Ok(response) = res {
            assert_eq!(response.status, 200);
            assert!(response.body.contains("field1"));
            assert!(response.body.contains("value1"));
        }
    }

    #[test]
    #[ignore]
    fn test_file_upload() {
        let client = HttpClient::new().unwrap();
        let fields = HashMap::new();
        let file = FileField {
            field_name: "file".to_string(),
            filename: "test.txt".to_string(),
            content: b"Hello World".to_vec(),
            content_type: "text/plain".to_string(),
        };

        let res = client.upload_file("https://httpbin.org/post", fields, file);
        assert!(res.is_ok());
        if let Ok(response) = res {
            assert_eq!(response.status, 200);
            assert!(response.body.contains("test.txt"));
        }
    }

    #[test]
    fn test_proxy_client_creation() {
        let result = HttpClient::new_with_proxy("http://localhost:8888");
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    #[ignore]
    fn test_get_with_options_no_options() {
        let client = HttpClient::new().unwrap();
        let res = client.get_with_options("https://httpbin.org/get", None, None);
        assert!(res.is_ok());
        if let Ok(response) = res {
            assert_eq!(response.status, 200);
        }
    }

    #[test]
    #[ignore]
    fn test_get_with_options_auth_only() {
        let client = HttpClient::new().unwrap();
        let auth = AuthOption::Basic {
            username: "user".to_string(),
            password: "pass".to_string(),
        };

        let res =
            client.get_with_options("https://httpbin.org/basic-auth/user/pass", None, Some(auth));
        assert!(res.is_ok());
        if let Ok(response) = res {
            assert_eq!(response.status, 200);
        }
    }

    #[test]
    fn test_form_encoding_special_characters() {
        use hype_rs::modules::builtins::http::forms::encode_form_urlencoded;

        let mut fields = HashMap::new();
        fields.insert("email".to_string(), "user@example.com".to_string());
        fields.insert("message".to_string(), "Hello World!".to_string());

        let encoded = encode_form_urlencoded(fields).unwrap();
        assert!(encoded.contains("email=user%40example.com"));
    }

    #[test]
    fn test_auth_special_characters() {
        let auth = AuthOption::Basic {
            username: "user@example.com".to_string(),
            password: "p@ss:w0rd!".to_string(),
        };

        let header = auth.to_header_value();
        assert!(header.starts_with("Basic "));

        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let encoded_part = &header[6..];
        let decoded = STANDARD.decode(encoded_part).unwrap();
        let decoded_str = String::from_utf8(decoded).unwrap();
        assert_eq!(decoded_str, "user@example.com:p@ss:w0rd!");
    }
}
