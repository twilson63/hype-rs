use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status: u16, status_text: String, headers: HashMap<String, String>, body: String) -> Self {
        Self {
            status,
            status_text,
            headers,
            body,
        }
    }

    pub fn ok(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    pub fn json(&self) -> Result<JsonValue, serde_json::Error> {
        serde_json::from_str(&self.body)
    }

    pub fn text(&self) -> String {
        self.body.clone()
    }

    #[cfg(feature = "http")]
    pub async fn from_reqwest(response: reqwest::Response) -> Result<Self, reqwest::Error> {
        let status = response.status().as_u16();
        let status_text = response.status().canonical_reason()
            .unwrap_or("Unknown")
            .to_string();

        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(key.to_string(), value_str.to_string());
            }
        }

        let body = response.text().await?;

        Ok(Self::new(status, status_text, headers, body))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_creation() {
        let headers = HashMap::new();
        let response = HttpResponse::new(
            200,
            "OK".to_string(),
            headers,
            "test body".to_string(),
        );

        assert_eq!(response.status, 200);
        assert_eq!(response.status_text, "OK");
        assert_eq!(response.body, "test body");
        assert!(response.ok());
    }

    #[test]
    fn test_response_ok() {
        let headers = HashMap::new();
        
        let response = HttpResponse::new(200, "OK".to_string(), headers.clone(), String::new());
        assert!(response.ok());

        let response = HttpResponse::new(299, "OK".to_string(), headers.clone(), String::new());
        assert!(response.ok());

        let response = HttpResponse::new(300, "Redirect".to_string(), headers.clone(), String::new());
        assert!(!response.ok());

        let response = HttpResponse::new(404, "Not Found".to_string(), headers.clone(), String::new());
        assert!(!response.ok());

        let response = HttpResponse::new(500, "Error".to_string(), headers, String::new());
        assert!(!response.ok());
    }

    #[test]
    fn test_response_json() {
        let headers = HashMap::new();
        let json_body = r#"{"name": "test", "value": 123}"#;
        let response = HttpResponse::new(200, "OK".to_string(), headers, json_body.to_string());

        let parsed = response.json().unwrap();
        assert_eq!(parsed["name"], "test");
        assert_eq!(parsed["value"], 123);
    }

    #[test]
    fn test_response_text() {
        let headers = HashMap::new();
        let response = HttpResponse::new(200, "OK".to_string(), headers, "plain text".to_string());
        assert_eq!(response.text(), "plain text");
    }
}
