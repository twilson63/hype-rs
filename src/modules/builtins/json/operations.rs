use super::error::JsonError;
use serde_json::Value as JsonValue;

pub type Result<T> = std::result::Result<T, JsonError>;

pub fn encode(value: &JsonValue) -> Result<String> {
    serde_json::to_string(value).map_err(Into::into)
}

pub fn encode_pretty(value: &JsonValue) -> Result<String> {
    serde_json::to_string_pretty(value).map_err(Into::into)
}

pub fn decode(json_str: &str) -> Result<JsonValue> {
    serde_json::from_str(json_str).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_encode_simple() {
        let value = json!({"name": "Alice", "age": 30});
        let result = encode(&value).unwrap();
        assert!(result.contains("Alice"));
        assert!(result.contains("30"));
    }

    #[test]
    fn test_encode_pretty() {
        let value = json!({"name": "Bob"});
        let result = encode_pretty(&value).unwrap();
        assert!(result.contains('\n'));
        assert!(result.contains("Bob"));
    }

    #[test]
    fn test_decode_object() {
        let json_str = r#"{"name":"Carol","age":25}"#;
        let result = decode(json_str).unwrap();
        assert_eq!(result["name"], "Carol");
        assert_eq!(result["age"], 25);
    }

    #[test]
    fn test_decode_array() {
        let json_str = r#"[1,2,3,4,5]"#;
        let result = decode(json_str).unwrap();
        assert!(result.is_array());
        assert_eq!(result.as_array().unwrap().len(), 5);
    }

    #[test]
    fn test_decode_invalid() {
        let json_str = r#"{"invalid": }"#;
        let result = decode(json_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_roundtrip() {
        let original = json!({
            "string": "test",
            "number": 42,
            "bool": true,
            "null": null,
            "array": [1, 2, 3],
            "nested": {"key": "value"}
        });

        let encoded = encode(&original).unwrap();
        let decoded = decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_unicode() {
        let value = json!({"text": "Hello 世界 🚀"});
        let encoded = encode(&value).unwrap();
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded["text"], "Hello 世界 🚀");
    }
}
