use serde_urlencoded;
use std::collections::HashMap;

#[cfg(feature = "http")]
use reqwest::multipart;

pub fn encode_form_urlencoded(fields: HashMap<String, String>) -> Result<String, String> {
    serde_urlencoded::to_string(&fields).map_err(|e| format!("Form encoding error: {}", e))
}

#[cfg(feature = "http")]
pub struct FileField {
    pub field_name: String,
    pub filename: String,
    pub content: Vec<u8>,
    pub content_type: String,
}

#[cfg(feature = "http")]
pub fn build_multipart_form(
    fields: HashMap<String, String>,
    file: FileField,
) -> Result<multipart::Form, String> {
    let mut form = multipart::Form::new();

    for (key, value) in fields {
        form = form.text(key, value);
    }

    let part = multipart::Part::bytes(file.content)
        .file_name(file.filename)
        .mime_str(&file.content_type)
        .map_err(|e| format!("Invalid MIME type: {}", e))?;

    form = form.part(file.field_name, part);

    Ok(form)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_encoding_simple() {
        let mut fields = HashMap::new();
        fields.insert("field1".to_string(), "value1".to_string());
        fields.insert("field2".to_string(), "value2".to_string());

        let encoded = encode_form_urlencoded(fields).unwrap();
        assert!(encoded.contains("field1=value1"));
        assert!(encoded.contains("field2=value2"));
    }

    #[test]
    fn test_form_encoding_special_chars() {
        let mut fields = HashMap::new();
        fields.insert("email".to_string(), "user@example.com".to_string());
        fields.insert("message".to_string(), "Hello World!".to_string());

        let encoded = encode_form_urlencoded(fields).unwrap();
        assert!(encoded.contains("email=user%40example.com"));
        assert!(
            encoded.contains("message=Hello+World%21")
                || encoded.contains("message=Hello%20World%21")
        );
    }

    #[test]
    fn test_form_encoding_unicode() {
        let mut fields = HashMap::new();
        fields.insert("name".to_string(), "José".to_string());

        let encoded = encode_form_urlencoded(fields).unwrap();
        assert!(encoded.contains("name=Jos%C3%A9"));
    }

    #[test]
    fn test_form_encoding_empty() {
        let fields = HashMap::new();
        let encoded = encode_form_urlencoded(fields).unwrap();
        assert_eq!(encoded, "");
    }

    #[cfg(feature = "http")]
    #[test]
    fn test_multipart_form_creation() {
        let mut fields = HashMap::new();
        fields.insert("description".to_string(), "Test file".to_string());

        let file = FileField {
            field_name: "file".to_string(),
            filename: "test.txt".to_string(),
            content: b"Hello World".to_vec(),
            content_type: "text/plain".to_string(),
        };

        let result = build_multipart_form(fields, file);
        assert!(result.is_ok());
    }

    #[cfg(feature = "http")]
    #[test]
    fn test_multipart_form_invalid_mime() {
        let fields = HashMap::new();

        let file = FileField {
            field_name: "file".to_string(),
            filename: "test.txt".to_string(),
            content: b"Hello World".to_vec(),
            content_type: "invalid mime type".to_string(),
        };

        let result = build_multipart_form(fields, file);
        assert!(result.is_err());
    }
}
