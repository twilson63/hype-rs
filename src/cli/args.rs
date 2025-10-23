use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ParsedArguments {
    pub script_name: String,
    pub indexed_args: Vec<String>,
    pub named_args: HashMap<String, String>,
    pub flags: HashMap<String, bool>,
    pub raw_args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ArgumentParser {
    pub allow_named_args: bool,
    pub allow_flags: bool,
    pub flag_prefixes: Vec<String>,
    pub key_value_separators: Vec<String>,
}

impl Default for ArgumentParser {
    fn default() -> Self {
        Self {
            allow_named_args: true,
            allow_flags: true,
            flag_prefixes: vec!["--".to_string(), "-".to_string()],
            key_value_separators: vec!["=".to_string(), ":".to_string()],
        }
    }
}

impl ArgumentParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(&self, script_path: &PathBuf, raw_args: &[String]) -> ParsedArguments {
        let script_name = script_path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "script".to_string());

        let mut indexed_args = Vec::new();
        let mut named_args = HashMap::new();
        let mut flags = HashMap::new();

        for arg in raw_args {
            if self.is_flag(arg) {
                if let Some((flag_name, flag_value)) = self.parse_flag(arg) {
                    if let Some(value) = flag_value {
                        named_args.insert(flag_name, value);
                    } else {
                        flags.insert(flag_name, true);
                    }
                }
            } else {
                indexed_args.push(arg.clone());
            }
        }

        ParsedArguments {
            script_name,
            indexed_args,
            named_args,
            flags,
            raw_args: raw_args.to_vec(),
        }
    }

    fn is_flag(&self, arg: &str) -> bool {
        self.flag_prefixes
            .iter()
            .any(|prefix| arg.starts_with(prefix))
    }

    fn parse_flag(&self, arg: &str) -> Option<(String, Option<String>)> {
        for prefix in &self.flag_prefixes {
            if arg.starts_with(prefix) {
                let flag_part = &arg[prefix.len()..];

                // Check for key=value format
                for separator in &self.key_value_separators {
                    if let Some(pos) = flag_part.find(separator) {
                        let key = flag_part[..pos].to_string();
                        let value = flag_part[pos + separator.len()..].to_string();
                        return Some((key, Some(value)));
                    }
                }

                // It's a boolean flag
                return Some((flag_part.to_string(), None));
            }
        }
        None
    }

    pub fn generate_help(&self, script_name: &str, description: Option<&str>) -> String {
        let mut help = String::new();

        if let Some(desc) = description {
            help.push_str(&format!("{}\n\n", desc));
        }

        help.push_str(&format!("Usage: {} [options] [arguments]\n\n", script_name));

        help.push_str("Arguments:\n");
        help.push_str(
            "  <args>...        Arguments to pass to the script (accessible via args table)\n\n",
        );

        if self.allow_flags {
            help.push_str("Options:\n");
            help.push_str("  --key=value     Set named argument (accessible via arg.key)\n");
            help.push_str("  -k=value        Short form of named argument\n");
            help.push_str("  --flag          Set boolean flag (accessible via arg.flag)\n");
            help.push_str("  -f              Short form of boolean flag\n\n");
        }

        help.push_str("Examples:\n");
        help.push_str(&format!("  {} arg1 arg2 arg3\n", script_name));
        help.push_str(&format!("  {} --name=John --age=25 arg1\n", script_name));
        help.push_str(&format!("  {} --verbose --debug input.txt\n", script_name));

        help
    }
}

pub fn create_standard_arg_table(
    lua: &mlua::Lua,
    parsed_args: &ParsedArguments,
) -> Result<(), mlua::Error> {
    let globals = lua.globals();

    // Create the standard Lua 'arg' table
    let arg_table = lua.create_table()?;

    // arg[0] = script name
    arg_table.set(0, parsed_args.script_name.clone())?;

    // arg[1], arg[2], ... = indexed arguments
    for (i, arg) in parsed_args.indexed_args.iter().enumerate() {
        arg_table.set(i + 1, arg.clone())?;
    }

    // Set negative indices for flags and named args (like standard Lua)
    let mut neg_index = 1;

    // Add flags as negative indices
    for (flag, value) in &parsed_args.flags {
        if *value {
            arg_table.set(-neg_index, format!("--{}", flag))?;
            neg_index += 1;
        }
    }

    // Add named args as negative indices
    for (key, value) in &parsed_args.named_args {
        arg_table.set(-neg_index, format!("--{}={}", key, value))?;
        neg_index += 1;
    }

    globals.set("arg", &arg_table)?;

    // Create enhanced args table with additional features
    let args_table = lua.create_table()?;

    // Set arg[0] = script name
    args_table.set(0, parsed_args.script_name.clone())?;

    // Set indexed arguments starting from 1
    for (i, arg) in parsed_args.indexed_args.iter().enumerate() {
        args_table.set(i + 1, arg.clone())?;
    }

    // Clone the data we need for the closures
    let flags = parsed_args.flags.clone();
    let named_args = parsed_args.named_args.clone();

    // Add convenience methods
    args_table.set(
        "has_flag",
        lua.create_function(move |_, (flag,): (String,)| Ok(flags.contains_key(&flag)))?,
    )?;

    args_table.set(
        "get_named",
        lua.create_function(move |_, (key,): (String,)| Ok(named_args.get(&key).cloned()))?,
    )?;

    args_table.set("count", parsed_args.indexed_args.len())?;
    args_table.set("flags", parsed_args.flags.clone())?;
    args_table.set("named", parsed_args.named_args.clone())?;

    globals.set("args", args_table)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_basic_argument_parsing() {
        let parser = ArgumentParser::new();
        let script_path = PathBuf::from("test.lua");
        let args = vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string()];

        let parsed = parser.parse(&script_path, &args);

        assert_eq!(parsed.script_name, "test");
        assert_eq!(parsed.indexed_args, args);
        assert!(parsed.named_args.is_empty());
        assert!(parsed.flags.is_empty());
    }

    #[test]
    fn test_flag_parsing() {
        let parser = ArgumentParser::new();
        let script_path = PathBuf::from("test.lua");
        let args = vec![
            "--verbose".to_string(),
            "--debug".to_string(),
            "arg1".to_string(),
        ];

        let parsed = parser.parse(&script_path, &args);

        assert_eq!(parsed.script_name, "test");
        assert_eq!(parsed.indexed_args, vec!["arg1"]);
        assert!(parsed.flags.get("verbose").unwrap());
        assert!(parsed.flags.get("debug").unwrap());
    }

    #[test]
    fn test_named_argument_parsing() {
        let parser = ArgumentParser::new();
        let script_path = PathBuf::from("test.lua");
        let args = vec![
            "--name=John".to_string(),
            "--age=25".to_string(),
            "arg1".to_string(),
        ];

        let parsed = parser.parse(&script_path, &args);

        assert_eq!(parsed.script_name, "test");
        assert_eq!(parsed.indexed_args, vec!["arg1"]);
        assert_eq!(parsed.named_args.get("name"), Some(&"John".to_string()));
        assert_eq!(parsed.named_args.get("age"), Some(&"25".to_string()));
    }

    #[test]
    fn test_mixed_arguments() {
        let parser = ArgumentParser::new();
        let script_path = PathBuf::from("test.lua");
        let args = vec![
            "--verbose".to_string(),
            "input.txt".to_string(),
            "--output=result.txt".to_string(),
            "--debug".to_string(),
            "arg2".to_string(),
        ];

        let parsed = parser.parse(&script_path, &args);

        assert_eq!(parsed.script_name, "test");
        assert_eq!(parsed.indexed_args, vec!["input.txt", "arg2"]);
        assert!(parsed.flags.get("verbose").unwrap());
        assert!(parsed.flags.get("debug").unwrap());
        assert_eq!(
            parsed.named_args.get("output"),
            Some(&"result.txt".to_string())
        );
    }

    #[test]
    fn test_short_flags() {
        let parser = ArgumentParser::new();
        let script_path = PathBuf::from("test.lua");
        let args = vec!["-v".to_string(), "-f=file.txt".to_string()];

        let parsed = parser.parse(&script_path, &args);

        assert_eq!(parsed.script_name, "test");
        assert!(parsed.flags.get("v").unwrap());
        assert_eq!(parsed.named_args.get("f"), Some(&"file.txt".to_string()));
    }
}
