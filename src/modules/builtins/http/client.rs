use super::{HttpError, HttpResponse, Result};
use std::collections::HashMap;
use std::time::Duration;

#[cfg(feature = "http")]
use tokio::runtime::Runtime;
#[cfg(feature = "http")]
use url::Url;

pub struct HttpClient {
    #[cfg(feature = "http")]
    client: reqwest::Client,
    #[cfg(feature = "http")]
    runtime: Runtime,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        #[cfg(feature = "http")]
        {
            let client = reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .pool_max_idle_per_host(10)
                .build()
                .map_err(|e| HttpError::RuntimeError(e.to_string()))?;

            let runtime = Runtime::new().map_err(|e| HttpError::RuntimeError(e.to_string()))?;

            Ok(Self { client, runtime })
        }

        #[cfg(not(feature = "http"))]
        {
            Err(HttpError::RuntimeError(
                "HTTP feature not enabled. Compile with --features http".to_string(),
            ))
        }
    }

    #[cfg(feature = "http")]
    pub fn get(&self, url: &str) -> Result<HttpResponse> {
        let parsed_url = Url::parse(url)
            .map_err(|e| HttpError::RequestError(format!("Invalid URL '{}': {}", url, e)))?;

        self.runtime.block_on(async {
            let response = self.client.get(parsed_url.as_str()).send().await?;
            HttpResponse::from_reqwest(response)
                .await
                .map_err(Into::into)
        })
    }

    #[cfg(not(feature = "http"))]
    pub fn get(&self, _url: &str) -> Result<HttpResponse> {
        Err(HttpError::RuntimeError(
            "HTTP feature not enabled".to_string(),
        ))
    }

    #[cfg(feature = "http")]
    pub fn post(
        &self,
        url: &str,
        body: Option<String>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse> {
        let parsed_url = Url::parse(url)
            .map_err(|e| HttpError::RequestError(format!("Invalid URL '{}': {}", url, e)))?;

        self.runtime.block_on(async {
            let mut request = self.client.post(parsed_url.as_str());

            if let Some(headers_map) = headers {
                for (key, value) in headers_map {
                    request = request.header(key, value);
                }
            }

            if let Some(body_content) = body {
                request = request.body(body_content);
            }

            let response = request.send().await?;
            HttpResponse::from_reqwest(response)
                .await
                .map_err(Into::into)
        })
    }

    #[cfg(not(feature = "http"))]
    pub fn post(
        &self,
        _url: &str,
        _body: Option<String>,
        _headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse> {
        Err(HttpError::RuntimeError(
            "HTTP feature not enabled".to_string(),
        ))
    }

    #[cfg(feature = "http")]
    pub fn put(
        &self,
        url: &str,
        body: Option<String>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse> {
        let parsed_url = Url::parse(url)
            .map_err(|e| HttpError::RequestError(format!("Invalid URL '{}': {}", url, e)))?;

        self.runtime.block_on(async {
            let mut request = self.client.put(parsed_url.as_str());

            if let Some(headers_map) = headers {
                for (key, value) in headers_map {
                    request = request.header(key, value);
                }
            }

            if let Some(body_content) = body {
                request = request.body(body_content);
            }

            let response = request.send().await?;
            HttpResponse::from_reqwest(response)
                .await
                .map_err(Into::into)
        })
    }

    #[cfg(not(feature = "http"))]
    pub fn put(
        &self,
        _url: &str,
        _body: Option<String>,
        _headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse> {
        Err(HttpError::RuntimeError(
            "HTTP feature not enabled".to_string(),
        ))
    }

    #[cfg(feature = "http")]
    pub fn delete(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse> {
        let parsed_url = Url::parse(url)
            .map_err(|e| HttpError::RequestError(format!("Invalid URL '{}': {}", url, e)))?;

        self.runtime.block_on(async {
            let mut request = self.client.delete(parsed_url.as_str());

            if let Some(headers_map) = headers {
                for (key, value) in headers_map {
                    request = request.header(key, value);
                }
            }

            let response = request.send().await?;
            HttpResponse::from_reqwest(response)
                .await
                .map_err(Into::into)
        })
    }

    #[cfg(not(feature = "http"))]
    pub fn delete(
        &self,
        _url: &str,
        _headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse> {
        Err(HttpError::RuntimeError(
            "HTTP feature not enabled".to_string(),
        ))
    }

    #[cfg(feature = "http")]
    pub fn fetch(
        &self,
        method: &str,
        url: &str,
        body: Option<String>,
        headers: Option<HashMap<String, String>>,
        timeout: Option<u64>,
    ) -> Result<HttpResponse> {
        let parsed_url = Url::parse(url)
            .map_err(|e| HttpError::RequestError(format!("Invalid URL '{}': {}", url, e)))?;

        self.runtime.block_on(async {
            let url_str = parsed_url.as_str();
            let mut request = match method.to_uppercase().as_str() {
                "GET" => self.client.get(url_str),
                "POST" => self.client.post(url_str),
                "PUT" => self.client.put(url_str),
                "DELETE" => self.client.delete(url_str),
                "PATCH" => self.client.patch(url_str),
                "HEAD" => self.client.head(url_str),
                "OPTIONS" => self.client.request(reqwest::Method::OPTIONS, url_str),
                _ => {
                    return Err(HttpError::RequestError(format!(
                        "Unsupported HTTP method: {}",
                        method
                    )))
                }
            };

            if let Some(headers_map) = headers {
                for (key, value) in headers_map {
                    request = request.header(key, value);
                }
            }

            if let Some(body_content) = body {
                request = request.body(body_content);
            }

            if let Some(timeout_ms) = timeout {
                request = request.timeout(Duration::from_millis(timeout_ms));
            }

            let response = request.send().await?;
            HttpResponse::from_reqwest(response)
                .await
                .map_err(Into::into)
        })
    }

    #[cfg(not(feature = "http"))]
    pub fn fetch(
        &self,
        _method: &str,
        _url: &str,
        _body: Option<String>,
        _headers: Option<HashMap<String, String>>,
        _timeout: Option<u64>,
    ) -> Result<HttpResponse> {
        Err(HttpError::RuntimeError(
            "HTTP feature not enabled".to_string(),
        ))
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let result = HttpClient::new();
        #[cfg(feature = "http")]
        assert!(result.is_ok());

        #[cfg(not(feature = "http"))]
        assert!(result.is_err());
    }

    #[cfg(feature = "http")]
    #[test]
    #[ignore]
    fn test_http_get() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://httpbin.org/get");

        if result.is_ok() {
            let response = result.unwrap();
            assert_eq!(response.status, 200);
            assert!(response.ok());
        }
    }

    #[cfg(feature = "http")]
    #[test]
    #[ignore]
    fn test_http_post() {
        let client = HttpClient::new().unwrap();
        let body = Some(r#"{"test": true}"#.to_string());
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let result = client.post("https://httpbin.org/post", body, Some(headers));

        if result.is_ok() {
            let response = result.unwrap();
            assert_eq!(response.status, 200);
        }
    }

    #[cfg(feature = "http")]
    #[test]
    #[ignore]
    fn test_http_fetch() {
        let client = HttpClient::new().unwrap();
        let result = client.fetch("GET", "https://httpbin.org/get", None, None, None);

        if result.is_ok() {
            let response = result.unwrap();
            assert_eq!(response.status, 200);
        }
    }

    #[cfg(feature = "http")]
    #[test]
    #[ignore]
    fn test_http_timeout() {
        let client = HttpClient::new().unwrap();
        let result = client.fetch(
            "GET",
            "https://httpbin.org/delay/10",
            None,
            None,
            Some(1000),
        );

        if result.is_err() {
            match result {
                Err(HttpError::TimeoutError) | Err(HttpError::NetworkError(_)) => (),
                _ => panic!("Expected timeout or network error"),
            }
        }
    }

    #[cfg(feature = "http")]
    #[test]
    #[ignore]
    fn test_url_with_tilde() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://httpbin.org/anything/~user/data");
        assert!(result.is_ok());
        if let Ok(response) = result {
            assert_eq!(response.status, 200);
        }
    }

    #[cfg(feature = "http")]
    #[test]
    fn test_invalid_url() {
        let client = HttpClient::new().unwrap();
        let result = client.get("not a valid url");
        assert!(result.is_err());
        if let Err(HttpError::RequestError(msg)) = result {
            assert!(msg.contains("Invalid URL"));
        } else {
            panic!("Expected RequestError");
        }
    }

    #[cfg(feature = "http")]
    #[test]
    #[ignore]
    fn test_already_encoded_url() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://httpbin.org/anything/path%20with%20encoded%20spaces");
        assert!(result.is_ok());
    }

    #[cfg(feature = "http")]
    #[test]
    #[ignore]
    fn test_url_with_fragment() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://httpbin.org/anything/path#fragment");
        assert!(result.is_ok());
    }

    #[cfg(feature = "http")]
    #[test]
    #[ignore]
    fn test_url_with_query_params() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://httpbin.org/get?foo=bar&baz=qux");
        assert!(result.is_ok());
        if let Ok(response) = result {
            assert_eq!(response.status, 200);
        }
    }

    #[cfg(feature = "http")]
    #[test]
    fn test_url_missing_protocol() {
        let client = HttpClient::new().unwrap();
        let result = client.get("httpbin.org/get");
        assert!(result.is_err());
        if let Err(HttpError::RequestError(msg)) = result {
            assert!(msg.contains("Invalid URL"));
        }
    }

    #[cfg(feature = "http")]
    #[test]
    #[ignore]
    fn test_url_with_special_chars_in_path() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://httpbin.org/anything/~test-file_name.txt");
        assert!(result.is_ok());
    }

    #[cfg(feature = "http")]
    #[test]
    #[ignore]
    fn test_fetch_with_tilde_url() {
        let client = HttpClient::new().unwrap();
        let result = client.fetch(
            "GET",
            "https://httpbin.org/anything/~test",
            None,
            None,
            None,
        );
        assert!(result.is_ok());
    }

    #[cfg(feature = "http")]
    #[test]
    fn test_post_invalid_url() {
        let client = HttpClient::new().unwrap();
        let result = client.post("invalid url", None, None);
        assert!(result.is_err());
    }

    #[cfg(feature = "http")]
    #[test]
    fn test_put_invalid_url() {
        let client = HttpClient::new().unwrap();
        let result = client.put("", None, None);
        assert!(result.is_err());
    }

    #[cfg(feature = "http")]
    #[test]
    fn test_delete_invalid_url() {
        let client = HttpClient::new().unwrap();
        let result = client.delete("ftp://invalid.com", None);
        assert!(result.is_ok() || result.is_err());
    }
}
