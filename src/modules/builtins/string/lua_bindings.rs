use super::operations::*;
use mlua::{Lua, Result as LuaResult, Table, Value};

pub fn create_string_module(lua: &Lua) -> LuaResult<Table> {
    let globals = lua.globals();
    let builtin_string: Table = globals.get("string")?;
    let string = lua.create_table()?;

    for pair in builtin_string.pairs::<Value, Value>() {
        let (key, value) = pair?;
        string.set(key, value)?;
    }

    let split_fn = lua.create_function(|lua, (s, delimiter): (String, String)| {
        let result = split(&s, &delimiter);
        let table = lua.create_table()?;
        for (i, part) in result.iter().enumerate() {
            table.set(i + 1, part.clone())?;
        }
        Ok(table)
    })?;
    string.set("split", split_fn)?;

    let trim_fn = lua.create_function(|_, s: String| Ok(trim(&s)))?;
    string.set("trim", trim_fn)?;

    let trim_start_fn = lua.create_function(|_, s: String| Ok(trim_start(&s)))?;
    string.set("trimStart", trim_start_fn)?;

    let trim_end_fn = lua.create_function(|_, s: String| Ok(trim_end(&s)))?;
    string.set("trimEnd", trim_end_fn)?;

    let starts_with_fn =
        lua.create_function(|_, (s, prefix): (String, String)| Ok(starts_with(&s, &prefix)))?;
    string.set("startsWith", starts_with_fn)?;

    let ends_with_fn =
        lua.create_function(|_, (s, suffix): (String, String)| Ok(ends_with(&s, &suffix)))?;
    string.set("endsWith", ends_with_fn)?;

    let contains_fn =
        lua.create_function(|_, (s, substring): (String, String)| Ok(contains(&s, &substring)))?;
    string.set("contains", contains_fn)?;

    let pad_start_fn =
        lua.create_function(|_, (s, length, fill): (String, usize, Option<String>)| {
            Ok(pad_start(&s, length, fill.as_deref()))
        })?;
    string.set("padStart", pad_start_fn)?;

    let pad_end_fn =
        lua.create_function(|_, (s, length, fill): (String, usize, Option<String>)| {
            Ok(pad_end(&s, length, fill.as_deref()))
        })?;
    string.set("padEnd", pad_end_fn)?;

    let repeat_fn = lua.create_function(|_, (s, count): (String, usize)| Ok(repeat(&s, count)))?;
    string.set("repeat", repeat_fn)?;

    let replace_fn = lua.create_function(
        |_, (s, pattern, replacement, count): (String, String, String, Option<usize>)| {
            Ok(replace(&s, &pattern, &replacement, count))
        },
    )?;
    string.set("replace", replace_fn)?;

    let replace_all_fn =
        lua.create_function(|_, (s, pattern, replacement): (String, String, String)| {
            Ok(replace_all(&s, &pattern, &replacement))
        })?;
    string.set("replaceAll", replace_all_fn)?;

    let to_upper_case_fn = lua.create_function(|_, s: String| Ok(to_upper_case(&s)))?;
    string.set("toUpperCase", to_upper_case_fn)?;

    let to_lower_case_fn = lua.create_function(|_, s: String| Ok(to_lower_case(&s)))?;
    string.set("toLowerCase", to_lower_case_fn)?;

    let capitalize_fn = lua.create_function(|_, s: String| Ok(capitalize(&s)))?;
    string.set("capitalize", capitalize_fn)?;

    let lines_fn = lua.create_function(|lua, s: String| {
        let result = lines(&s);
        let table = lua.create_table()?;
        for (i, line) in result.iter().enumerate() {
            table.set(i + 1, line.clone())?;
        }
        Ok(table)
    })?;
    string.set("lines", lines_fn)?;

    let chars_fn = lua.create_function(|lua, s: String| {
        let result = chars(&s);
        let table = lua.create_table()?;
        for (i, char) in result.iter().enumerate() {
            table.set(i + 1, char.clone())?;
        }
        Ok(table)
    })?;
    string.set("chars", chars_fn)?;

    Ok(string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua;

    #[test]
    fn test_create_string_module() {
        let lua = Lua::new();
        let result = create_string_module(&lua);
        assert!(result.is_ok());

        let string = result.unwrap();
        assert!(string.contains_key("split").unwrap());
        assert!(string.contains_key("trim").unwrap());
        assert!(string.contains_key("trimStart").unwrap());
        assert!(string.contains_key("trimEnd").unwrap());
        assert!(string.contains_key("startsWith").unwrap());
        assert!(string.contains_key("endsWith").unwrap());
        assert!(string.contains_key("contains").unwrap());
        assert!(string.contains_key("padStart").unwrap());
        assert!(string.contains_key("padEnd").unwrap());
        assert!(string.contains_key("repeat").unwrap());
        assert!(string.contains_key("replace").unwrap());
        assert!(string.contains_key("replaceAll").unwrap());
        assert!(string.contains_key("toUpperCase").unwrap());
        assert!(string.contains_key("toLowerCase").unwrap());
        assert!(string.contains_key("capitalize").unwrap());
        assert!(string.contains_key("lines").unwrap());
        assert!(string.contains_key("chars").unwrap());
    }

    #[test]
    fn test_string_split() {
        let lua = Lua::new();
        let string = create_string_module(&lua).unwrap();
        lua.globals().set("string", string).unwrap();

        let result: mlua::Table = lua
            .load(r#"return string.split("a,b,c", ",")"#)
            .eval()
            .unwrap();

        assert_eq!(result.len().unwrap(), 3);
        assert_eq!(result.get::<_, String>(1).unwrap(), "a");
        assert_eq!(result.get::<_, String>(2).unwrap(), "b");
        assert_eq!(result.get::<_, String>(3).unwrap(), "c");
    }

    #[test]
    fn test_string_trim() {
        let lua = Lua::new();
        let string = create_string_module(&lua).unwrap();
        lua.globals().set("string", string).unwrap();

        let result: String = lua
            .load(r#"return string.trim("  hello  ")"#)
            .eval()
            .unwrap();

        assert_eq!(result, "hello");
    }

    #[test]
    fn test_string_to_upper_case() {
        let lua = Lua::new();
        let string = create_string_module(&lua).unwrap();
        lua.globals().set("string", string).unwrap();

        let result: String = lua
            .load(r#"return string.toUpperCase("hello")"#)
            .eval()
            .unwrap();

        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_string_capitalize() {
        let lua = Lua::new();
        let string = create_string_module(&lua).unwrap();
        lua.globals().set("string", string).unwrap();

        let result: String = lua
            .load(r#"return string.capitalize("hello")"#)
            .eval()
            .unwrap();

        assert_eq!(result, "Hello");
    }
}
