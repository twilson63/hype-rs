use super::operations::*;
use mlua::{Lua, Result as LuaResult, Table, Value};
use std::collections::HashMap;

pub fn create_url_module(lua: &Lua) -> LuaResult<Table> {
    let url = lua.create_table()?;

    let parse_fn = lua.create_function(|lua, url_str: String| match parse(&url_str) {
        Ok(parsed) => {
            let table = lua.create_table()?;
            table.set("protocol", parsed.protocol)?;
            table.set("host", parsed.host)?;
            table.set("hostname", parsed.hostname)?;
            table.set("port", parsed.port)?;
            table.set("path", parsed.path)?;
            table.set("query", parsed.query)?;
            table.set("hash", parsed.fragment)?;
            table.set("username", parsed.username)?;
            table.set("password", parsed.password)?;
            Ok(table)
        }
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    url.set("parse", parse_fn)?;

    let format_fn = lua.create_function(|_, components: Table| {
        let url_components = UrlComponents {
            protocol: components.get("protocol")?,
            host: components.get("host")?,
            hostname: components.get("hostname")?,
            port: components.get("port")?,
            path: components.get("path")?,
            query: components.get("query")?,
            fragment: components.get("hash")?,
            username: components.get("username")?,
            password: components.get("password")?,
        };

        match format(url_components) {
            Ok(url_str) => Ok(url_str),
            Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
        }
    })?;
    url.set("format", format_fn)?;

    let resolve_fn =
        lua.create_function(|_, (base, relative): (String, String)| {
            match resolve(&base, &relative) {
                Ok(resolved) => Ok(resolved),
                Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
            }
        })?;
    url.set("resolve", resolve_fn)?;

    let encode_fn = lua.create_function(|_, input: String| Ok(encode(&input)))?;
    url.set("encode", encode_fn)?;

    let decode_fn = lua.create_function(|_, input: String| match decode(&input) {
        Ok(decoded) => Ok(decoded),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    url.set("decode", decode_fn)?;

    let encode_component_fn =
        lua.create_function(|_, input: String| Ok(encode_component(&input)))?;
    url.set("encodeComponent", encode_component_fn)?;

    let decode_component_fn =
        lua.create_function(|_, input: String| match decode_component(&input) {
            Ok(decoded) => Ok(decoded),
            Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
        })?;
    url.set("decodeComponent", decode_component_fn)?;

    let parse_query_fn = lua.create_function(|lua, query: String| {
        let parsed = parse_query(&query);
        let table = lua.create_table()?;
        for (key, value) in parsed {
            table.set(key, value)?;
        }
        Ok(table)
    })?;
    url.set("parseQuery", parse_query_fn)?;

    let format_query_fn = lua.create_function(|_, table: Table| {
        let mut params = HashMap::new();
        for pair in table.pairs::<String, String>() {
            let (key, value) = pair?;
            params.insert(key, value);
        }
        Ok(format_query(params))
    })?;
    url.set("formatQuery", format_query_fn)?;

    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua;

    #[test]
    fn test_create_url_module() {
        let lua = Lua::new();
        let result = create_url_module(&lua);
        assert!(result.is_ok());

        let url = result.unwrap();
        assert!(url.contains_key("parse").unwrap());
        assert!(url.contains_key("format").unwrap());
        assert!(url.contains_key("resolve").unwrap());
        assert!(url.contains_key("encode").unwrap());
        assert!(url.contains_key("decode").unwrap());
        assert!(url.contains_key("encodeComponent").unwrap());
        assert!(url.contains_key("decodeComponent").unwrap());
        assert!(url.contains_key("parseQuery").unwrap());
        assert!(url.contains_key("formatQuery").unwrap());
    }

    #[test]
    fn test_url_parse() {
        let lua = Lua::new();
        let url = create_url_module(&lua).unwrap();
        lua.globals().set("url", url).unwrap();

        lua.load(
            r#"
local parsed = url.parse("https://example.com:8080/path?query=1#hash")
assert(parsed.protocol == "https")
assert(parsed.hostname == "example.com")
assert(parsed.port == 8080)
assert(parsed.path == "/path")
assert(parsed.query == "query=1")
assert(parsed.hash == "hash")
"#,
        )
        .exec()
        .unwrap();
    }

    #[test]
    fn test_url_encode_decode() {
        let lua = Lua::new();
        let url = create_url_module(&lua).unwrap();
        lua.globals().set("url", url).unwrap();

        lua.load(
            r#"
local encoded = url.encodeComponent("hello world")
assert(encoded == "hello+world", "Expected 'hello+world' but got: " .. encoded)
local decoded = url.decodeComponent(encoded)
assert(decoded == "hello world")
"#,
        )
        .exec()
        .unwrap();
    }
}
