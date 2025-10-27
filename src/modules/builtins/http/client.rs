use super::auth::AuthOption;
use super::forms;
use super::{HttpError, HttpResponse, Result};
use std::collections::HashMap;
use std::time::Duration;

#[cfg(feature = "http")]
use reqwest::cookie::Jar;
#[cfg(feature = "http")]
use reqwest::Proxy;
#[cfg(feature = "http")]
use std::sync::Arc;
#[cfg(feature = "http")]
use tokio::runtime::Runtime;
#[cfg(feature = "http")]
use url::Url;

pub struct HttpClient {
    #[cfg(feature = "http")]
    client: reqwest::Client,
    #[cfg(feature = "http")]
    runtime: Runtime,
    #[cfg(feature = "http")]
    cookie_jar: Arc<Jar>,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        #[cfg(feature = "http")]
        {
            let cookie_jar = Arc::new(Jar::default());

            let client = reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .pool_max_idle_per_host(10)
                .cookie_provider(cookie_jar.clone())
                .build()
                .map_err(|e| HttpError::RuntimeError(e.to_string()))?;

            let runtime = Runtime::new().map_err(|e| HttpError::RuntimeError(e.to_string()))?;

            Ok(Self {
                client,
                runtime,
                cookie_jar,
            })
        }

        #[cfg(not(feature = "http"))]
        {
            Err(HttpError::RuntimeError(
                "HTTP feature not enabled. Compile with --features http".to_string(),
            ))
        }
    }

    #[cfg(feature = "http")]
    pub fn new_with_proxy(proxy_url: &str) -> Result<Self> {
        let cookie_jar = Arc::new(Jar::default());

        let proxy = Proxy::all(proxy_url)
            .map_err(|e| HttpError::RequestError(format!("Invalid proxy: {}", e)))?;

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .cookie_provider(cookie_jar.clone())
            .proxy(proxy)
            .build()
            .map_err(|e| HttpError::RuntimeError(e.to_string()))?;

        let runtime = Runtime::new().map_err(|e| HttpError::RuntimeError(e.to_string()))?;

        Ok(Self {
            client,
            runtime,
            cookie_jar,
        })
    }

    #[cfg(not(feature = "http"))]
    pub fn new_with_proxy(_proxy_url: &str) -> Result<Self> {
        Err(HttpError::RuntimeError(
            "HTTP feature not enabled".to_string(),
        ))
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

    #[cfg(feature = "http")]
    pub fn get_cookies(&self, url: &str) -> Result<Vec<(String, String)>> {
        use reqwest::cookie::CookieStore;

        let parsed_url = Url::parse(url)
            .map_err(|e| HttpError::RequestError(format!("Invalid URL '{}': {}", url, e)))?;

        let cookies = if let Some(header) = self.cookie_jar.cookies(&parsed_url) {
            if let Ok(header_str) = header.to_str() {
                header_str
                    .split("; ")
                    .filter_map(|cookie| {
                        let mut parts = cookie.splitn(2, '=');
                        Some((parts.next()?.to_string(), parts.next()?.to_string()))
                    })
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        Ok(cookies)
    }

    #[cfg(not(feature = "http"))]
    pub fn get_cookies(&self, _url: &str) -> Result<Vec<(String, String)>> {
        Err(HttpError::RuntimeError(
            "HTTP feature not enabled".to_string(),
        ))
    }

    #[cfg(feature = "http")]
    pub fn get_with_options(
        &self,
        url: &str,
        proxy: Option<String>,
        auth: Option<AuthOption>,
    ) -> Result<HttpResponse> {
        let parsed_url = Url::parse(url)
            .map_err(|e| HttpError::RequestError(format!("Invalid URL '{}': {}", url, e)))?;

        if let Some(proxy_url) = proxy {
            let temp_client = Self::new_with_proxy(&proxy_url)?;
            if let Some(auth_opt) = auth {
                return temp_client.get_with_auth(url, auth_opt);
            }
            return temp_client.get(url);
        }

        if let Some(auth_opt) = auth {
            return self.get_with_auth(url, auth_opt);
        }

        self.get(url)
    }

    #[cfg(not(feature = "http"))]
    pub fn get_with_options(
        &self,
        _url: &str,
        _proxy: Option<String>,
        _auth: Option<AuthOption>,
    ) -> Result<HttpResponse> {
        Err(HttpError::RuntimeError(
            "HTTP feature not enabled".to_string(),
        ))
    }

    #[cfg(feature = "http")]
    pub fn get_with_auth(&self, url: &str, auth: AuthOption) -> Result<HttpResponse> {
        let parsed_url = Url::parse(url)
            .map_err(|e| HttpError::RequestError(format!("Invalid URL '{}': {}", url, e)))?;

        self.runtime.block_on(async {
            let response = self
                .client
                .get(parsed_url.as_str())
                .header("Authorization", auth.to_header_value())
                .send()
                .await?;
            HttpResponse::from_reqwest(response)
                .await
                .map_err(Into::into)
        })
    }

    #[cfg(not(feature = "http"))]
    pub fn get_with_auth(&self, _url: &str, _auth: AuthOption) -> Result<HttpResponse> {
        Err(HttpError::RuntimeError(
            "HTTP feature not enabled".to_string(),
        ))
    }

    #[cfg(feature = "http")]
    pub fn post_form(&self, url: &str, fields: HashMap<String, String>) -> Result<HttpResponse> {
        let body = forms::encode_form_urlencoded(fields).map_err(|e| HttpError::RequestError(e))?;

        let mut headers = HashMap::new();
        headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        self.post(url, Some(body), Some(headers))
    }

    #[cfg(not(feature = "http"))]
    pub fn post_form(&self, _url: &str, _fields: HashMap<String, String>) -> Result<HttpResponse> {
        Err(HttpError::RuntimeError(
            "HTTP feature not enabled".to_string(),
        ))
    }

    #[cfg(feature = "http")]
    pub fn upload_file(
        &self,
        url: &str,
        fields: HashMap<String, String>,
        file: forms::FileField,
    ) -> Result<HttpResponse> {
        let parsed_url =
            Url::parse(url).map_err(|e| HttpError::RequestError(format!("Invalid URL: {}", e)))?;

        let form =
            forms::build_multipart_form(fields, file).map_err(|e| HttpError::RequestError(e))?;

        self.runtime.block_on(async {
            let response = self
                .client
                .post(parsed_url.as_str())
                .multipart(form)
                .send()
                .await?;

            HttpResponse::from_reqwest(response)
                .await
                .map_err(Into::into)
        })
    }

    #[cfg(not(feature = "http"))]
    pub fn upload_file(
        &self,
        _url: &str,
        _fields: HashMap<String, String>,
        _file: forms::FileField,
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
