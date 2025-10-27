use super::operations::*;
use mlua::{Lua, Table, Value as LuaValue};

pub fn create_json_module(lua: &Lua) -> mlua::Result<Table> {
    let json_table = lua.create_table()?;

    register_encode(lua, &json_table)?;
    register_decode(lua, &json_table)?;
    register_stringify(lua, &json_table)?;
    register_parse(lua, &json_table)?;

    Ok(json_table)
}

fn register_encode(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let encode_fn = lua.create_function(|lua, (value, pretty): (LuaValue, Option<bool>)| {
        let json_value = lua_to_json(lua, value)?;

        let result = if pretty.unwrap_or(false) {
            encode_pretty(&json_value)
        } else {
            encode(&json_value)
        };

        result.map_err(mlua::Error::external)
    })?;
    table.set("encode", encode_fn)?;
    Ok(())
}

fn register_decode(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let decode_fn = lua.create_function(move |lua, json_str: String| {
        let json_value = decode(&json_str).map_err(mlua::Error::external)?;
        json_to_lua(lua, &json_value)
    })?;
    table.set("decode", decode_fn)?;
    Ok(())
}

fn register_stringify(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let stringify_fn = lua.create_function(|lua, (value, pretty): (LuaValue, Option<bool>)| {
        let json_value = lua_to_json(lua, value)?;

        let result = if pretty.unwrap_or(false) {
            encode_pretty(&json_value)
        } else {
            encode(&json_value)
        };

        result.map_err(mlua::Error::external)
    })?;
    table.set("stringify", stringify_fn)?;
    Ok(())
}

fn register_parse(lua: &Lua, table: &Table) -> mlua::Result<()> {
    let parse_fn = lua.create_function(move |lua, json_str: String| {
        let json_value = decode(&json_str).map_err(mlua::Error::external)?;
        json_to_lua(lua, &json_value)
    })?;
    table.set("parse", parse_fn)?;
    Ok(())
}

fn lua_to_json(_lua: &Lua, value: LuaValue) -> mlua::Result<serde_json::Value> {
    match value {
        LuaValue::Nil => Ok(serde_json::Value::Null),
        LuaValue::Boolean(b) => Ok(serde_json::Value::Bool(b)),
        LuaValue::Integer(i) => Ok(serde_json::Value::Number(i.into())),
        LuaValue::Number(n) => {
            if let Some(num) = serde_json::Number::from_f64(n) {
                Ok(serde_json::Value::Number(num))
            } else {
                Err(mlua::Error::external("Invalid number for JSON"))
            }
        }
        LuaValue::String(s) => Ok(serde_json::Value::String(s.to_str()?.to_string())),
        LuaValue::Table(t) => {
            if is_array(&t)? {
                let mut arr = Vec::new();
                for pair in t.sequence_values::<LuaValue>() {
                    arr.push(lua_to_json(_lua, pair?)?);
                }
                Ok(serde_json::Value::Array(arr))
            } else {
                let mut map = serde_json::Map::new();
                for pair in t.pairs::<LuaValue, LuaValue>() {
                    let (k, v) = pair?;
                    let key = match k {
                        LuaValue::String(s) => s.to_str()?.to_string(),
                        LuaValue::Integer(i) => i.to_string(),
                        LuaValue::Number(n) => n.to_string(),
                        _ => {
                            return Err(mlua::Error::external(
                                "Table keys must be strings or numbers",
                            ))
                        }
                    };
                    map.insert(key, lua_to_json(_lua, v)?);
                }
                Ok(serde_json::Value::Object(map))
            }
        }
        _ => Err(mlua::Error::external(
            "Unsupported Lua type for JSON conversion",
        )),
    }
}

fn json_to_lua<'lua>(lua: &'lua Lua, value: &serde_json::Value) -> mlua::Result<LuaValue<'lua>> {
    match value {
        serde_json::Value::Null => Ok(LuaValue::Nil),
        serde_json::Value::Bool(b) => Ok(LuaValue::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(LuaValue::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(LuaValue::Number(f))
            } else {
                Err(mlua::Error::external("Invalid JSON number"))
            }
        }
        serde_json::Value::String(s) => Ok(LuaValue::String(lua.create_string(s)?)),
        serde_json::Value::Array(arr) => {
            let table = lua.create_table()?;
            for (i, item) in arr.iter().enumerate() {
                table.set(i + 1, json_to_lua(lua, item)?)?;
            }
            Ok(LuaValue::Table(table))
        }
        serde_json::Value::Object(obj) => {
            let table = lua.create_table()?;
            for (key, val) in obj {
                table.set(key.as_str(), json_to_lua(lua, val)?)?;
            }
            Ok(LuaValue::Table(table))
        }
    }
}

fn is_array(table: &Table) -> mlua::Result<bool> {
    let mut max_index = 0;
    let mut count = 0;

    for pair in table.clone().pairs::<LuaValue, LuaValue>() {
        let (key, _) = pair?;
        match key {
            LuaValue::Integer(i) if i > 0 => {
                max_index = max_index.max(i);
                count += 1;
            }
            _ => return Ok(false),
        }
    }

    Ok(count > 0 && count == max_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_json_module() {
        let lua = Lua::new();
        let result = create_json_module(&lua);
        assert!(result.is_ok());

        let json_table = result.unwrap();
        assert!(json_table.contains_key("encode").unwrap());
        assert!(json_table.contains_key("decode").unwrap());
        assert!(json_table.contains_key("stringify").unwrap());
        assert!(json_table.contains_key("parse").unwrap());
    }

    #[test]
    fn test_lua_json_roundtrip() {
        let lua = Lua::new();
        let json_table = create_json_module(&lua).unwrap();
        lua.globals().set("json", json_table).unwrap();

        let code = r#"
            local data = {name = "Alice", age = 30, active = true}
            local encoded = json.encode(data)
            local decoded = json.decode(encoded)
            return decoded.name, decoded.age, decoded.active
        "#;

        let (name, age, active): (String, i64, bool) = lua.load(code).eval().unwrap();
        assert_eq!(name, "Alice");
        assert_eq!(age, 30);
        assert!(active);
    }
}
