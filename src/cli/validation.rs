use crate::cli::args::ParsedArguments;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ArgumentValidator {
    pub required_args: Vec<String>,
    pub optional_args: HashMap<String, String>,
    pub arg_types: HashMap<String, ArgumentType>,
    pub default_values: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArgumentType {
    String,
    Integer,
    Float,
    Boolean,
    Path,
    Email,
    Url,
}

impl ArgumentValidator {
    pub fn new() -> Self {
        Self {
            required_args: Vec::new(),
            optional_args: HashMap::new(),
            arg_types: HashMap::new(),
            default_values: HashMap::new(),
        }
    }

    pub fn require(mut self, name: &str, arg_type: ArgumentType) -> Self {
        self.required_args.push(name.to_string());
        self.arg_types.insert(name.to_string(), arg_type);
        self
    }

    pub fn optional(mut self, name: &str, arg_type: ArgumentType, default: &str) -> Self {
        self.optional_args
            .insert(name.to_string(), format!("Optional {}", name));
        self.arg_types.insert(name.to_string(), arg_type);
        self.default_values
            .insert(name.to_string(), default.to_string());
        self
    }

    pub fn validate(&self, parsed_args: &ParsedArguments) -> Result<(), String> {
        // Check required arguments
        for required in &self.required_args {
            if !parsed_args.named_args.contains_key(required)
                && !parsed_args.flags.contains_key(required)
            {
                return Err(format!("Missing required argument: --{}", required));
            }
        }

        // Validate argument types
        for (name, value) in &parsed_args.named_args {
            if let Some(expected_type) = self.arg_types.get(name) {
                if let Err(e) = self.validate_type(name, value, expected_type) {
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    fn validate_type(
        &self,
        name: &str,
        value: &str,
        expected_type: &ArgumentType,
    ) -> Result<(), String> {
        match expected_type {
            ArgumentType::String => Ok(()), // Any string is valid
            ArgumentType::Integer => {
                value.parse::<i64>().map_err(|_| {
                    format!("Argument '{}' must be an integer, got: {}", name, value)
                })?;
                Ok(())
            }
            ArgumentType::Float => {
                value
                    .parse::<f64>()
                    .map_err(|_| format!("Argument '{}' must be a number, got: {}", name, value))?;
                Ok(())
            }
            ArgumentType::Boolean => {
                if !["true", "false", "1", "0", "yes", "no"]
                    .contains(&value.to_lowercase().as_str())
                {
                    return Err(format!(
                        "Argument '{}' must be a boolean, got: {}",
                        name, value
                    ));
                }
                Ok(())
            }
            ArgumentType::Path => {
                if value.is_empty() {
                    return Err(format!(
                        "Argument '{}' must be a valid path, got empty string",
                        name
                    ));
                }
                Ok(())
            }
            ArgumentType::Email => {
                if !value.contains('@') || !value.contains('.') {
                    return Err(format!(
                        "Argument '{}' must be a valid email, got: {}",
                        name, value
                    ));
                }
                Ok(())
            }
            ArgumentType::Url => {
                if !(value.starts_with("http://")
                    || value.starts_with("https://")
                    || value.starts_with("file://"))
                {
                    return Err(format!(
                        "Argument '{}' must be a valid URL, got: {}",
                        name, value
                    ));
                }
                Ok(())
            }
        }
    }

    pub fn apply_defaults(&self, parsed_args: &mut ParsedArguments) {
        for (name, default_value) in &self.default_values {
            if !parsed_args.named_args.contains_key(name) && !parsed_args.flags.contains_key(name) {
                parsed_args
                    .named_args
                    .insert(name.clone(), default_value.clone());
            }
        }
    }

    pub fn get_help(&self, script_name: &str) -> String {
        let mut help = String::new();
        help.push_str(&format!("Usage: {} [options]\n\n", script_name));

        if !self.required_args.is_empty() {
            help.push_str("Required arguments:\n");
            for arg in &self.required_args {
                let type_str = match self.arg_types.get(arg) {
                    Some(ArgumentType::String) => "string",
                    Some(ArgumentType::Integer) => "integer",
                    Some(ArgumentType::Float) => "number",
                    Some(ArgumentType::Boolean) => "boolean",
                    Some(ArgumentType::Path) => "path",
                    Some(ArgumentType::Email) => "email",
                    Some(ArgumentType::Url) => "url",
                    None => "string",
                };
                help.push_str(&format!("  --{} <{}>\n", arg, type_str));
            }
            help.push('\n');
        }

        if !self.optional_args.is_empty() {
            help.push_str("Optional arguments:\n");
            for (arg, _) in &self.optional_args {
                let type_str = match self.arg_types.get(arg) {
                    Some(ArgumentType::String) => "string",
                    Some(ArgumentType::Integer) => "integer",
                    Some(ArgumentType::Float) => "number",
                    Some(ArgumentType::Boolean) => "boolean",
                    Some(ArgumentType::Path) => "path",
                    Some(ArgumentType::Email) => "email",
                    Some(ArgumentType::Url) => "url",
                    None => "string",
                };
                let default_str = self
                    .default_values
                    .get(arg)
                    .cloned()
                    .unwrap_or_else(|| "".to_string());
                help.push_str(&format!(
                    "  --{} <{}> (default: {})\n",
                    arg, type_str, default_str
                ));
            }
            help.push('\n');
        }

        help.push_str("Examples:\n");
        if !self.required_args.is_empty() {
            let example_args: Vec<String> = self
                .required_args
                .iter()
                .map(|arg| format!("--{}=value", arg))
                .collect();
            help.push_str(&format!("  {} {}\n", script_name, example_args.join(" ")));
        }

        help
    }
}

pub fn convert_argument_value(value: &str, target_type: &ArgumentType) -> Result<String, String> {
    match target_type {
        ArgumentType::String => Ok(value.to_string()),
        ArgumentType::Integer => value
            .parse::<i64>()
            .map(|v| v.to_string())
            .map_err(|_| format!("Cannot convert '{}' to integer", value)),
        ArgumentType::Float => value
            .parse::<f64>()
            .map(|v| v.to_string())
            .map_err(|_| format!("Cannot convert '{}' to number", value)),
        ArgumentType::Boolean => match value.to_lowercase().as_str() {
            "true" | "1" | "yes" => Ok("true".to_string()),
            "false" | "0" | "no" => Ok("false".to_string()),
            _ => Err(format!("Cannot convert '{}' to boolean", value)),
        },
        ArgumentType::Path => Ok(value.to_string()),
        ArgumentType::Email => Ok(value.to_string()),
        ArgumentType::Url => Ok(value.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argument_validation() {
        let validator = ArgumentValidator::new()
            .require("name", ArgumentType::String)
            .require("age", ArgumentType::Integer)
            .optional("verbose", ArgumentType::Boolean, "false");

        let mut parsed_args = ParsedArguments {
            script_name: "test".to_string(),
            indexed_args: vec![],
            named_args: HashMap::new(),
            flags: HashMap::new(),
            raw_args: vec![],
        };

        parsed_args
            .named_args
            .insert("name".to_string(), "John".to_string());
        parsed_args
            .named_args
            .insert("age".to_string(), "25".to_string());

        assert!(validator.validate(&parsed_args).is_ok());
    }

    #[test]
    fn test_missing_required_argument() {
        let validator = ArgumentValidator::new()
            .require("name", ArgumentType::String)
            .require("age", ArgumentType::Integer);

        let mut parsed_args = ParsedArguments {
            script_name: "test".to_string(),
            indexed_args: vec![],
            named_args: HashMap::new(),
            flags: HashMap::new(),
            raw_args: vec![],
        };

        parsed_args
            .named_args
            .insert("name".to_string(), "John".to_string());

        assert!(validator.validate(&parsed_args).is_err());
    }

    #[test]
    fn test_type_validation() {
        let validator = ArgumentValidator::new().require("age", ArgumentType::Integer);

        let mut parsed_args = ParsedArguments {
            script_name: "test".to_string(),
            indexed_args: vec![],
            named_args: HashMap::new(),
            flags: HashMap::new(),
            raw_args: vec![],
        };

        parsed_args
            .named_args
            .insert("age".to_string(), "not_a_number".to_string());

        assert!(validator.validate(&parsed_args).is_err());
    }

    #[test]
    fn test_default_values() {
        let validator = ArgumentValidator::new().optional("verbose", ArgumentType::Boolean, "true");

        let mut parsed_args = ParsedArguments {
            script_name: "test".to_string(),
            indexed_args: vec![],
            named_args: HashMap::new(),
            flags: HashMap::new(),
            raw_args: vec![],
        };

        validator.apply_defaults(&mut parsed_args);

        assert_eq!(
            parsed_args.named_args.get("verbose"),
            Some(&"true".to_string())
        );
    }
}
