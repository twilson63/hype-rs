use super::error::QueryStringError;
use std::collections::HashMap;
use url::form_urlencoded;

pub fn parse(query: &str) -> HashMap<String, String> {
    form_urlencoded::parse(query.as_bytes())
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

pub fn stringify(params: HashMap<String, String>) -> String {
    let mut serializer = form_urlencoded::Serializer::new(String::new());
    for (key, value) in params {
        serializer.append_pair(&key, &value);
    }
    serializer.finish()
}

pub fn escape(input: &str) -> String {
    form_urlencoded::byte_serialize(input.as_bytes()).collect()
}

pub fn unescape(input: &str) -> Result<String, QueryStringError> {
    let with_spaces = input.replace('+', " ");
    let decoded = percent_encoding::percent_decode_str(&with_spaces)
        .decode_utf8()
        .map_err(|e| QueryStringError::InvalidUtf8(e.to_string()))?;
    Ok(decoded.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let result = parse("foo=bar&baz=qux");
        assert_eq!(result.get("foo"), Some(&"bar".to_string()));
        assert_eq!(result.get("baz"), Some(&"qux".to_string()));
    }

    #[test]
    fn test_parse_with_encoding() {
        let result = parse("name=John+Doe&email=test%40example.com");
        assert_eq!(result.get("name"), Some(&"John Doe".to_string()));
        assert_eq!(result.get("email"), Some(&"test@example.com".to_string()));
    }

    #[test]
    fn test_stringify() {
        let mut params = HashMap::new();
        params.insert("foo".to_string(), "bar".to_string());
        params.insert("baz".to_string(), "qux".to_string());
        let result = stringify(params);
        assert!(result.contains("foo=bar"));
        assert!(result.contains("baz=qux"));
    }

    #[test]
    fn test_stringify_with_spaces() {
        let mut params = HashMap::new();
        params.insert("name".to_string(), "John Doe".to_string());
        let result = stringify(params);
        assert!(result.contains("name=John+Doe"));
    }

    #[test]
    fn test_escape() {
        assert_eq!(escape("hello world"), "hello+world");
        assert_eq!(escape("foo@bar.com"), "foo%40bar.com");
        assert_eq!(escape("a&b=c"), "a%26b%3Dc");
    }

    #[test]
    fn test_unescape() {
        assert_eq!(unescape("hello+world").unwrap(), "hello world");
        assert_eq!(unescape("foo%40bar.com").unwrap(), "foo@bar.com");
        assert_eq!(unescape("a%26b%3Dc").unwrap(), "a&b=c");
    }

    #[test]
    fn test_escape_unescape_roundtrip() {
        let original = "hello world & foo=bar @#$";
        let escaped = escape(original);
        let unescaped = unescape(&escaped).unwrap();
        assert_eq!(unescaped, original);
    }
}
