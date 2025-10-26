use super::{HttpError, HttpResponse, Result};
use std::collections::HashMap;
use std::time::Duration;

#[cfg(feature = "http")]
use tokio::runtime::Runtime;

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

            let runtime = Runtime::new()
                .map_err(|e| HttpError::RuntimeError(e.to_string()))?;

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
        self.runtime.block_on(async {
            let response = self.client.get(url).send().await?;
            HttpResponse::from_reqwest(response).await.map_err(Into::into)
        })
    }

    #[cfg(not(feature = "http"))]
    pub fn get(&self, _url: &str) -> Result<HttpResponse> {
        Err(HttpError::RuntimeError("HTTP feature not enabled".to_string()))
    }

    #[cfg(feature = "http")]
    pub fn post(&self, url: &str, body: Option<String>, headers: Option<HashMap<String, String>>) -> Result<HttpResponse> {
        self.runtime.block_on(async {
            let mut request = self.client.post(url);

            if let Some(headers_map) = headers {
                for (key, value) in headers_map {
                    request = request.header(key, value);
                }
            }

            if let Some(body_content) = body {
                request = request.body(body_content);
            }

            let response = request.send().await?;
            HttpResponse::from_reqwest(response).await.map_err(Into::into)
        })
    }

    #[cfg(not(feature = "http"))]
    pub fn post(&self, _url: &str, _body: Option<String>, _headers: Option<HashMap<String, String>>) -> Result<HttpResponse> {
        Err(HttpError::RuntimeError("HTTP feature not enabled".to_string()))
    }

    #[cfg(feature = "http")]
    pub fn put(&self, url: &str, body: Option<String>, headers: Option<HashMap<String, String>>) -> Result<HttpResponse> {
        self.runtime.block_on(async {
            let mut request = self.client.put(url);

            if let Some(headers_map) = headers {
                for (key, value) in headers_map {
                    request = request.header(key, value);
                }
            }

            if let Some(body_content) = body {
                request = request.body(body_content);
            }

            let response = request.send().await?;
            HttpResponse::from_reqwest(response).await.map_err(Into::into)
        })
    }

    #[cfg(not(feature = "http"))]
    pub fn put(&self, _url: &str, _body: Option<String>, _headers: Option<HashMap<String, String>>) -> Result<HttpResponse> {
        Err(HttpError::RuntimeError("HTTP feature not enabled".to_string()))
    }

    #[cfg(feature = "http")]
    pub fn delete(&self, url: &str, headers: Option<HashMap<String, String>>) -> Result<HttpResponse> {
        self.runtime.block_on(async {
            let mut request = self.client.delete(url);

            if let Some(headers_map) = headers {
                for (key, value) in headers_map {
                    request = request.header(key, value);
                }
            }

            let response = request.send().await?;
            HttpResponse::from_reqwest(response).await.map_err(Into::into)
        })
    }

    #[cfg(not(feature = "http"))]
    pub fn delete(&self, _url: &str, _headers: Option<HashMap<String, String>>) -> Result<HttpResponse> {
        Err(HttpError::RuntimeError("HTTP feature not enabled".to_string()))
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
        self.runtime.block_on(async {
            let mut request = match method.to_uppercase().as_str() {
                "GET" => self.client.get(url),
                "POST" => self.client.post(url),
                "PUT" => self.client.put(url),
                "DELETE" => self.client.delete(url),
                "PATCH" => self.client.patch(url),
                "HEAD" => self.client.head(url),
                "OPTIONS" => self.client.request(reqwest::Method::OPTIONS, url),
                _ => return Err(HttpError::RequestError(format!("Unsupported HTTP method: {}", method))),
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
            HttpResponse::from_reqwest(response).await.map_err(Into::into)
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
        Err(HttpError::RuntimeError("HTTP feature not enabled".to_string()))
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
}
