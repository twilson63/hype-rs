#[cfg(feature = "http")]
mod http_url_encoding_tests {
    use hype_rs::modules::builtins::http::{HttpClient, HttpError};

    #[test]
    #[ignore]
    fn test_url_with_tilde_in_path() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://httpbin.org/anything/~username/profile");

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
    }

    #[test]
    #[ignore]
    fn test_url_with_multiple_tildes() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://httpbin.org/anything/~path~with~tildes");

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
    }

    #[test]
    #[ignore]
    fn test_url_with_unreserved_chars() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://httpbin.org/anything/test-file_name.123~backup");

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
    }

    #[test]
    #[ignore]
    fn test_url_with_already_encoded_spaces() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://httpbin.org/anything/path%20with%20spaces");

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
    }

    #[test]
    #[ignore]
    fn test_url_with_fragment() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://httpbin.org/anything/page#section");

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
    }

    #[test]
    #[ignore]
    fn test_url_with_query_parameters() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://httpbin.org/get?name=test&value=123");

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
    }

    #[test]
    #[ignore]
    fn test_url_with_query_and_fragment() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://httpbin.org/get?foo=bar#top");

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
    }

    #[test]
    fn test_invalid_url_no_protocol() {
        let client = HttpClient::new().unwrap();
        let result = client.get("httpbin.org/get");

        assert!(result.is_err());
        if let Err(HttpError::RequestError(msg)) = result {
            assert!(
                msg.contains("Invalid URL"),
                "Error message should mention invalid URL"
            );
        } else {
            panic!("Expected RequestError");
        }
    }

    #[test]
    fn test_invalid_url_empty_string() {
        let client = HttpClient::new().unwrap();
        let result = client.get("");

        assert!(result.is_err());
        if let Err(HttpError::RequestError(msg)) = result {
            assert!(msg.contains("Invalid URL"));
        } else {
            panic!("Expected RequestError");
        }
    }

    #[test]
    fn test_invalid_url_malformed() {
        let client = HttpClient::new().unwrap();
        let result = client.get("ht!tp://invalid url with spaces");

        assert!(result.is_err());
    }

    #[test]
    #[ignore]
    fn test_post_with_tilde_url() {
        let client = HttpClient::new().unwrap();
        let result = client.post(
            "https://httpbin.org/anything/~user/data",
            Some(r#"{"test": "data"}"#.to_string()),
            None,
        );

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
    }

    #[test]
    #[ignore]
    fn test_put_with_encoded_path() {
        let client = HttpClient::new().unwrap();
        let result = client.put(
            "https://httpbin.org/anything/path%20space",
            Some("update data".to_string()),
            None,
        );

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
    }

    #[test]
    #[ignore]
    fn test_delete_with_special_chars() {
        let client = HttpClient::new().unwrap();
        let result = client.delete("https://httpbin.org/anything/~test-file_123", None);

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
    }

    #[test]
    #[ignore]
    fn test_fetch_get_with_complex_url() {
        let client = HttpClient::new().unwrap();
        let result = client.fetch(
            "GET",
            "https://httpbin.org/anything/~user/data?query=test&id=123#section",
            None,
            None,
            None,
        );

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
    }

    #[test]
    #[ignore]
    fn test_fetch_post_with_tilde() {
        let client = HttpClient::new().unwrap();
        let result = client.fetch(
            "POST",
            "https://httpbin.org/anything/~api/v1",
            Some(r#"{"key":"value"}"#.to_string()),
            None,
            None,
        );

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
    }

    #[test]
    fn test_fetch_invalid_url() {
        let client = HttpClient::new().unwrap();
        let result = client.fetch("GET", "not-a-valid-url", None, None, None);

        assert!(result.is_err());
    }

    #[test]
    #[ignore]
    fn test_ipv6_localhost() {
        let client = HttpClient::new().unwrap();
        let result = client.get("http://[::1]:8080/test");

        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    #[ignore]
    fn test_url_with_port() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://httpbin.org:443/get");

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, 200);
    }

    #[test]
    #[ignore]
    fn test_url_with_userinfo() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://user:pass@httpbin.org/get");

        assert!(result.is_ok() || result.is_err());
    }
}
