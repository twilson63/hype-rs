use super::error::StringError;

pub fn split(s: &str, delimiter: &str) -> Vec<String> {
    if delimiter.is_empty() {
        return s.chars().map(|c| c.to_string()).collect();
    }
    s.split(delimiter).map(|s| s.to_string()).collect()
}

pub fn trim(s: &str) -> String {
    s.trim().to_string()
}

pub fn trim_start(s: &str) -> String {
    s.trim_start().to_string()
}

pub fn trim_end(s: &str) -> String {
    s.trim_end().to_string()
}

pub fn starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

pub fn ends_with(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

pub fn contains(s: &str, substring: &str) -> bool {
    s.contains(substring)
}

pub fn pad_start(s: &str, length: usize, fill: Option<&str>) -> String {
    let fill_char = fill.and_then(|f| f.chars().next()).unwrap_or(' ');
    let current_len = s.chars().count();

    if current_len >= length {
        return s.to_string();
    }

    let padding_needed = length - current_len;
    let padding: String = std::iter::repeat(fill_char).take(padding_needed).collect();
    format!("{}{}", padding, s)
}

pub fn pad_end(s: &str, length: usize, fill: Option<&str>) -> String {
    let fill_char = fill.and_then(|f| f.chars().next()).unwrap_or(' ');
    let current_len = s.chars().count();

    if current_len >= length {
        return s.to_string();
    }

    let padding_needed = length - current_len;
    let padding: String = std::iter::repeat(fill_char).take(padding_needed).collect();
    format!("{}{}", s, padding)
}

pub fn repeat(s: &str, count: usize) -> String {
    s.repeat(count)
}

pub fn replace(s: &str, pattern: &str, replacement: &str, count: Option<usize>) -> String {
    if pattern.is_empty() {
        return s.to_string();
    }

    match count {
        Some(n) if n > 0 => {
            let mut result = s.to_string();
            for _ in 0..n {
                if let Some(pos) = result.find(pattern) {
                    result.replace_range(pos..pos + pattern.len(), replacement);
                } else {
                    break;
                }
            }
            result
        }
        _ => s.to_string(),
    }
}

pub fn replace_all(s: &str, pattern: &str, replacement: &str) -> String {
    if pattern.is_empty() {
        return s.to_string();
    }
    s.replace(pattern, replacement)
}

pub fn to_upper_case(s: &str) -> String {
    s.to_uppercase()
}

pub fn to_lower_case(s: &str) -> String {
    s.to_lowercase()
}

pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

pub fn lines(s: &str) -> Vec<String> {
    s.lines().map(|line| line.to_string()).collect()
}

pub fn chars(s: &str) -> Vec<String> {
    s.chars().map(|c| c.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        assert_eq!(split("a,b,c", ","), vec!["a", "b", "c"]);
        assert_eq!(split("hello", ""), vec!["h", "e", "l", "l", "o"]);
        assert_eq!(split("one", ","), vec!["one"]);
    }

    #[test]
    fn test_trim() {
        assert_eq!(trim("  hello  "), "hello");
        assert_eq!(trim("hello"), "hello");
        assert_eq!(trim("  "), "");
    }

    #[test]
    fn test_trim_start() {
        assert_eq!(trim_start("  hello  "), "hello  ");
        assert_eq!(trim_start("hello"), "hello");
    }

    #[test]
    fn test_trim_end() {
        assert_eq!(trim_end("  hello  "), "  hello");
        assert_eq!(trim_end("hello"), "hello");
    }

    #[test]
    fn test_starts_with() {
        assert!(starts_with("hello", "hel"));
        assert!(!starts_with("hello", "ell"));
        assert!(starts_with("", ""));
    }

    #[test]
    fn test_ends_with() {
        assert!(ends_with("hello", "llo"));
        assert!(!ends_with("hello", "ell"));
        assert!(ends_with("", ""));
    }

    #[test]
    fn test_contains() {
        assert!(contains("hello world", "wo"));
        assert!(!contains("hello", "xyz"));
        assert!(contains("", ""));
    }

    #[test]
    fn test_pad_start() {
        assert_eq!(pad_start("5", 3, None), "  5");
        assert_eq!(pad_start("5", 3, Some("0")), "005");
        assert_eq!(pad_start("hello", 3, None), "hello");
    }

    #[test]
    fn test_pad_end() {
        assert_eq!(pad_end("5", 3, None), "5  ");
        assert_eq!(pad_end("5", 3, Some("0")), "500");
        assert_eq!(pad_end("hello", 3, None), "hello");
    }

    #[test]
    fn test_repeat() {
        assert_eq!(repeat("ab", 3), "ababab");
        assert_eq!(repeat("x", 0), "");
        assert_eq!(repeat("", 5), "");
    }

    #[test]
    fn test_replace() {
        assert_eq!(replace("hello hello", "l", "L", Some(2)), "heLLo hello");
        assert_eq!(replace("abc", "", "x", Some(1)), "abc");
        assert_eq!(replace("aaa", "a", "b", Some(0)), "aaa");
    }

    #[test]
    fn test_replace_all() {
        assert_eq!(replace_all("hello hello", "l", "L"), "heLLo heLLo");
        assert_eq!(replace_all("abc", "x", "y"), "abc");
        assert_eq!(replace_all("", "a", "b"), "");
    }

    #[test]
    fn test_to_upper_case() {
        assert_eq!(to_upper_case("hello"), "HELLO");
        assert_eq!(to_upper_case("Hello"), "HELLO");
        assert_eq!(to_upper_case(""), "");
    }

    #[test]
    fn test_to_lower_case() {
        assert_eq!(to_lower_case("HELLO"), "hello");
        assert_eq!(to_lower_case("Hello"), "hello");
        assert_eq!(to_lower_case(""), "");
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize("HELLO"), "HELLO");
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("h"), "H");
    }

    #[test]
    fn test_lines() {
        assert_eq!(lines("a\nb\nc"), vec!["a", "b", "c"]);
        assert_eq!(lines("one"), vec!["one"]);
        assert_eq!(lines(""), Vec::<String>::new());
    }

    #[test]
    fn test_chars() {
        assert_eq!(chars("abc"), vec!["a", "b", "c"]);
        assert_eq!(chars(""), Vec::<String>::new());
        assert_eq!(chars("x"), vec!["x"]);
    }
}
