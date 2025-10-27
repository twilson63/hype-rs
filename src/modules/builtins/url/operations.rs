use super::error::UrlError;
use std::collections::HashMap;
use url::{form_urlencoded, Url};

pub struct ParsedUrl {
    pub protocol: String,
    pub host: Option<String>,
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub fn parse(url_str: &str) -> Result<ParsedUrl, UrlError> {
    let parsed = Url::parse(url_str).map_err(|e| UrlError::ParseError(e.to_string()))?;

    Ok(ParsedUrl {
        protocol: parsed.scheme().to_string(),
        host: parsed.host_str().map(|s| s.to_string()),
        hostname: parsed.host_str().map(|s| s.to_string()),
        port: parsed.port(),
        path: parsed.path().to_string(),
        query: parsed.query().map(|s| s.to_string()),
        fragment: parsed.fragment().map(|s| s.to_string()),
        username: if parsed.username().is_empty() {
            None
        } else {
            Some(parsed.username().to_string())
        },
        password: parsed.password().map(|s| s.to_string()),
    })
}

pub struct UrlComponents {
    pub protocol: Option<String>,
    pub host: Option<String>,
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub fn format(components: UrlComponents) -> Result<String, UrlError> {
    let protocol = components
        .protocol
        .ok_or_else(|| UrlError::MissingComponent("protocol".to_string()))?;

    let mut url_str = format!("{}://", protocol);

    if let Some(username) = components.username {
        url_str.push_str(&username);
        if let Some(password) = components.password {
            url_str.push(':');
            url_str.push_str(&password);
        }
        url_str.push('@');
    }

    if let Some(hostname) = components.hostname.or(components.host) {
        url_str.push_str(&hostname);
        if let Some(port) = components.port {
            url_str.push(':');
            url_str.push_str(&port.to_string());
        }
    }

    if let Some(path) = components.path {
        if !path.starts_with('/') {
            url_str.push('/');
        }
        url_str.push_str(&path);
    }

    if let Some(query) = components.query {
        url_str.push('?');
        url_str.push_str(&query);
    }

    if let Some(fragment) = components.fragment {
        url_str.push('#');
        url_str.push_str(&fragment);
    }

    Ok(url_str)
}

pub fn resolve(base: &str, relative: &str) -> Result<String, UrlError> {
    let base_url = Url::parse(base).map_err(|e| UrlError::ParseError(e.to_string()))?;

    let resolved = base_url
        .join(relative)
        .map_err(|e| UrlError::ParseError(e.to_string()))?;

    Ok(resolved.to_string())
}

pub fn encode(input: &str) -> String {
    form_urlencoded::byte_serialize(input.as_bytes()).collect()
}

pub fn decode(input: &str) -> Result<String, UrlError> {
    form_urlencoded::parse(input.as_bytes())
        .map(|(key, _)| key.to_string())
        .next()
        .ok_or_else(|| UrlError::InvalidComponent("empty string".to_string()))
}

pub fn encode_component(input: &str) -> String {
    form_urlencoded::byte_serialize(input.as_bytes()).collect()
}

pub fn decode_component(input: &str) -> Result<String, UrlError> {
    let with_spaces = input.replace('+', " ");
    let decoded = percent_encoding::percent_decode_str(&with_spaces)
        .decode_utf8()
        .map_err(|e| UrlError::InvalidComponent(e.to_string()))?;
    Ok(decoded.to_string())
}

pub fn parse_query(query: &str) -> HashMap<String, String> {
    form_urlencoded::parse(query.as_bytes())
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

pub fn format_query(params: HashMap<String, String>) -> String {
    let mut serializer = form_urlencoded::Serializer::new(String::new());
    for (key, value) in params {
        serializer.append_pair(&key, &value);
    }
    serializer.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let result = parse("https://example.com:8080/path?query=1#hash").unwrap();
        assert_eq!(result.protocol, "https");
        assert_eq!(result.hostname, Some("example.com".to_string()));
        assert_eq!(result.port, Some(8080));
        assert_eq!(result.path, "/path");
        assert_eq!(result.query, Some("query=1".to_string()));
        assert_eq!(result.fragment, Some("hash".to_string()));
    }

    #[test]
    fn test_parse_with_auth() {
        let result = parse("https://user:pass@example.com/path").unwrap();
        assert_eq!(result.username, Some("user".to_string()));
        assert_eq!(result.password, Some("pass".to_string()));
    }

    #[test]
    fn test_format() {
        let components = UrlComponents {
            protocol: Some("https".to_string()),
            hostname: Some("example.com".to_string()),
            host: None,
            port: Some(8080),
            path: Some("/path".to_string()),
            query: Some("key=value".to_string()),
            fragment: Some("hash".to_string()),
            username: None,
            password: None,
        };
        let result = format(components).unwrap();
        assert_eq!(result, "https://example.com:8080/path?key=value#hash");
    }

    #[test]
    fn test_resolve() {
        let result = resolve("https://example.com/foo/bar", "../baz").unwrap();
        assert_eq!(result, "https://example.com/baz");
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode("hello world"), "hello+world");
        assert_eq!(encode("foo@bar.com"), "foo%40bar.com");
    }

    #[test]
    fn test_decode_component() {
        assert_eq!(decode_component("hello%20world").unwrap(), "hello world");
        assert_eq!(decode_component("foo%40bar.com").unwrap(), "foo@bar.com");
    }

    #[test]
    fn test_parse_query() {
        let result = parse_query("foo=bar&baz=qux");
        assert_eq!(result.get("foo"), Some(&"bar".to_string()));
        assert_eq!(result.get("baz"), Some(&"qux".to_string()));
    }

    #[test]
    fn test_format_query() {
        let mut params = HashMap::new();
        params.insert("foo".to_string(), "bar".to_string());
        params.insert("baz".to_string(), "qux".to_string());
        let result = format_query(params);
        assert!(result.contains("foo=bar"));
        assert!(result.contains("baz=qux"));
    }
}
