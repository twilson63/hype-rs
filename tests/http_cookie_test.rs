#[cfg(feature = "http")]
mod http_cookie_tests {
    use hype_rs::modules::builtins::http::HttpClient;

    #[test]
    fn test_cookie_jar_creation() {
        let client = HttpClient::new();
        assert!(client.is_ok(), "HttpClient creation should succeed");
    }

    #[test]
    #[ignore] // Requires network access
    fn test_cookies_stored_and_sent() {
        let client = HttpClient::new().unwrap();

        // First request: set cookie via httpbin.org
        let res1 = client.get("https://httpbin.org/cookies/set?test=value");
        assert!(res1.is_ok(), "First request should succeed");
        assert_eq!(res1.unwrap().status, 200);

        // Second request: cookie should be sent automatically
        let res2 = client.get("https://httpbin.org/cookies");
        assert!(res2.is_ok(), "Second request should succeed");

        let response = res2.unwrap();
        assert_eq!(response.status, 200);
        assert!(
            response.body.contains("test"),
            "Response should contain cookie name"
        );
        assert!(
            response.body.contains("value"),
            "Response should contain cookie value"
        );
    }

    #[test]
    #[ignore] // Requires network access
    fn test_multiple_cookies() {
        let client = HttpClient::new().unwrap();

        // Set multiple cookies
        client
            .get("https://httpbin.org/cookies/set?cookie1=value1")
            .unwrap();
        client
            .get("https://httpbin.org/cookies/set?cookie2=value2")
            .unwrap();

        // Verify both cookies are sent
        let res = client.get("https://httpbin.org/cookies").unwrap();
        assert!(res.body.contains("cookie1"), "Should contain first cookie");
        assert!(res.body.contains("cookie2"), "Should contain second cookie");
    }

    #[test]
    #[ignore] // Requires network access
    fn test_get_cookies_api() {
        let client = HttpClient::new().unwrap();

        // Set a cookie
        client
            .get("https://httpbin.org/cookies/set?session=abc123")
            .unwrap();

        // Get cookies for the domain
        let cookies = client.get_cookies("https://httpbin.org");
        assert!(cookies.is_ok(), "get_cookies should succeed");

        let cookie_vec = cookies.unwrap();
        assert!(!cookie_vec.is_empty(), "Should have at least one cookie");

        // Verify our cookie is in the list
        let has_session = cookie_vec
            .iter()
            .any(|(name, value)| name == "session" && value == "abc123");
        assert!(has_session, "Should have session cookie with correct value");
    }

    #[test]
    fn test_get_cookies_with_invalid_url() {
        let client = HttpClient::new().unwrap();
        let result = client.get_cookies("not a valid url");
        assert!(result.is_err(), "Should fail with invalid URL");
    }

    #[test]
    fn test_get_cookies_without_cookies() {
        let client = HttpClient::new().unwrap();
        let cookies = client.get_cookies("https://example.com");
        assert!(cookies.is_ok(), "Should succeed even without cookies");
        assert!(cookies.unwrap().is_empty(), "Should return empty vec");
    }
}

#[cfg(not(feature = "http"))]
mod http_cookie_tests_disabled {
    use hype_rs::modules::builtins::http::HttpClient;

    #[test]
    fn test_http_disabled() {
        let result = HttpClient::new();
        assert!(result.is_err(), "Should fail when HTTP feature is disabled");
    }
}
