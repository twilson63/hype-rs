use super::operations::*;
use mlua::{Lua, Result as LuaResult, Table};
use std::collections::HashMap;

pub fn create_querystring_module(lua: &Lua) -> LuaResult<Table> {
    let querystring = lua.create_table()?;

    let parse_fn = lua.create_function(|lua, query: String| {
        let parsed = parse(&query);
        let table = lua.create_table()?;
        for (key, value) in parsed {
            table.set(key, value)?;
        }
        Ok(table)
    })?;
    querystring.set("parse", parse_fn)?;

    let stringify_fn = lua.create_function(|_, table: Table| {
        let mut params = HashMap::new();
        for pair in table.pairs::<String, String>() {
            let (key, value) = pair?;
            params.insert(key, value);
        }
        Ok(stringify(params))
    })?;
    querystring.set("stringify", stringify_fn)?;

    let escape_fn = lua.create_function(|_, input: String| Ok(escape(&input)))?;
    querystring.set("escape", escape_fn)?;

    let unescape_fn = lua.create_function(|_, input: String| match unescape(&input) {
        Ok(decoded) => Ok(decoded),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    querystring.set("unescape", unescape_fn)?;

    Ok(querystring)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua;

    #[test]
    fn test_create_querystring_module() {
        let lua = Lua::new();
        let result = create_querystring_module(&lua);
        assert!(result.is_ok());

        let querystring = result.unwrap();
        assert!(querystring.contains_key("parse").unwrap());
        assert!(querystring.contains_key("stringify").unwrap());
        assert!(querystring.contains_key("escape").unwrap());
        assert!(querystring.contains_key("unescape").unwrap());
    }

    #[test]
    fn test_querystring_parse() {
        let lua = Lua::new();
        let querystring = create_querystring_module(&lua).unwrap();
        lua.globals().set("querystring", querystring).unwrap();

        lua.load(
            r#"
local parsed = querystring.parse("foo=bar&baz=qux")
assert(parsed.foo == "bar")
assert(parsed.baz == "qux")
"#,
        )
        .exec()
        .unwrap();
    }

    #[test]
    fn test_querystring_stringify() {
        let lua = Lua::new();
        let querystring = create_querystring_module(&lua).unwrap();
        lua.globals().set("querystring", querystring).unwrap();

        lua.load(
            r#"
local result = querystring.stringify({foo = "bar", baz = "qux"})
assert(string.find(result, "foo=bar") ~= nil)
assert(string.find(result, "baz=qux") ~= nil)
"#,
        )
        .exec()
        .unwrap();
    }

    #[test]
    fn test_querystring_escape_unescape() {
        let lua = Lua::new();
        let querystring = create_querystring_module(&lua).unwrap();
        lua.globals().set("querystring", querystring).unwrap();

        lua.load(
            r#"
local escaped = querystring.escape("hello world")
assert(escaped == "hello+world")
local unescaped = querystring.unescape(escaped)
assert(unescaped == "hello world")
"#,
        )
        .exec()
        .unwrap();
    }
}
