use mlua::{Lua, Table, Value};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;

use super::{HttpClient, HttpResponse};

#[cfg(feature = "http")]
pub fn create_http_module(lua: &Lua) -> mlua::Result<Table> {
    let http_table = lua.create_table()?;
    
    let client = Arc::new(
        HttpClient::new()
            .map_err(|e| mlua::Error::external(e))?
    );
    
    register_get(lua, &http_table, client.clone())?;
    register_post(lua, &http_table, client.clone())?;
    register_put(lua, &http_table, client.clone())?;
    register_delete(lua, &http_table, client.clone())?;
    register_patch(lua, &http_table, client.clone())?;
    register_head(lua, &http_table, client.clone())?;
    register_fetch(lua, &http_table, client.clone())?;
    register_post_json(lua, &http_table, client.clone())?;
    register_put_json(lua, &http_table, client)?;
    
    Ok(http_table)
}

#[cfg(not(feature = "http"))]
pub fn create_http_module(lua: &Lua) -> mlua::Result<Table> {
    let http_table = lua.create_table()?;
    
    let error_fn = lua.create_function(|_, _: mlua::MultiValue| {
        Err(mlua::Error::external(std::io::Error::new(
            std::io::ErrorKind::Other,
            "HTTP feature not enabled. Compile with --features http"
        )))
    })?;
    
    http_table.set("get", error_fn.clone())?;
    http_table.set("post", error_fn.clone())?;
    http_table.set("put", error_fn.clone())?;
    http_table.set("delete", error_fn.clone())?;
    http_table.set("patch", error_fn.clone())?;
    http_table.set("head", error_fn.clone())?;
    http_table.set("fetch", error_fn.clone())?;
    http_table.set("postJson", error_fn.clone())?;
    http_table.set("putJson", error_fn)?;
    
    Ok(http_table)
}

#[cfg(feature = "http")]
fn register_get(lua: &Lua, table: &Table, client: Arc<HttpClient>) -> mlua::Result<()> {
    let get_fn = lua.create_function(move |lua, url: String| {
        let response = client.get(&url)
            .map_err(|e| mlua::Error::external(e))?;
        create_response_table(lua, response)
    })?;
    table.set("get", get_fn)?;
    Ok(())
}

#[cfg(feature = "http")]
fn register_post(lua: &Lua, table: &Table, client: Arc<HttpClient>) -> mlua::Result<()> {
    let post_fn = lua.create_function(move |lua, (url, options): (String, Option<Table>)| {
        let (body, headers) = parse_request_options(options)?;
        let response = client.post(&url, body, headers)
            .map_err(|e| mlua::Error::external(e))?;
        create_response_table(lua, response)
    })?;
    table.set("post", post_fn)?;
    Ok(())
}

#[cfg(feature = "http")]
fn register_put(lua: &Lua, table: &Table, client: Arc<HttpClient>) -> mlua::Result<()> {
    let put_fn = lua.create_function(move |lua, (url, options): (String, Option<Table>)| {
        let (body, headers) = parse_request_options(options)?;
        let response = client.put(&url, body, headers)
            .map_err(|e| mlua::Error::external(e))?;
        create_response_table(lua, response)
    })?;
    table.set("put", put_fn)?;
    Ok(())
}

#[cfg(feature = "http")]
fn register_delete(lua: &Lua, table: &Table, client: Arc<HttpClient>) -> mlua::Result<()> {
    let delete_fn = lua.create_function(move |lua, (url, options): (String, Option<Table>)| {
        let headers = options.and_then(|opts| parse_headers(&opts).ok().flatten());
        let response = client.delete(&url, headers)
            .map_err(|e| mlua::Error::external(e))?;
        create_response_table(lua, response)
    })?;
    table.set("delete", delete_fn)?;
    Ok(())
}

#[cfg(feature = "http")]
fn register_patch(lua: &Lua, table: &Table, client: Arc<HttpClient>) -> mlua::Result<()> {
    let patch_fn = lua.create_function(move |lua, (url, options): (String, Option<Table>)| {
        let (body, headers) = parse_request_options(options)?;
        
        let method = "PATCH";
        let response = client.fetch(method, &url, body, headers, None)
            .map_err(|e| mlua::Error::external(e))?;
        create_response_table(lua, response)
    })?;
    table.set("patch", patch_fn)?;
    Ok(())
}

#[cfg(feature = "http")]
fn register_head(lua: &Lua, table: &Table, client: Arc<HttpClient>) -> mlua::Result<()> {
    let head_fn = lua.create_function(move |lua, (url, options): (String, Option<Table>)| {
        let headers = options.and_then(|opts| parse_headers(&opts).ok().flatten());
        
        let method = "HEAD";
        let response = client.fetch(method, &url, None, headers, None)
            .map_err(|e| mlua::Error::external(e))?;
        create_response_table(lua, response)
    })?;
    table.set("head", head_fn)?;
    Ok(())
}

#[cfg(feature = "http")]
fn register_fetch(lua: &Lua, table: &Table, client: Arc<HttpClient>) -> mlua::Result<()> {
    let fetch_fn = lua.create_function(move |lua, (url, options): (String, Option<Table>)| {
        let opts = parse_fetch_options(options)?;
        let response = client.fetch(
            &opts.method,
            &url,
            opts.body,
            opts.headers,
            opts.timeout,
        ).map_err(|e| mlua::Error::external(e))?;
        create_response_table(lua, response)
    })?;
    table.set("fetch", fetch_fn)?;
    Ok(())
}

#[cfg(feature = "http")]
fn register_post_json(lua: &Lua, table: &Table, client: Arc<HttpClient>) -> mlua::Result<()> {
    let post_json_fn = lua.create_function(move |lua, (url, data): (String, Table)| {
        let json_value = lua_table_to_json(lua, &data)?;
        let body = serde_json::to_string(&json_value)
            .map_err(|e| mlua::Error::external(e))?;
        
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        let response = client.post(&url, Some(body), Some(headers))
            .map_err(|e| mlua::Error::external(e))?;
        create_response_table(lua, response)
    })?;
    table.set("postJson", post_json_fn)?;
    Ok(())
}

#[cfg(feature = "http")]
fn register_put_json(lua: &Lua, table: &Table, client: Arc<HttpClient>) -> mlua::Result<()> {
    let put_json_fn = lua.create_function(move |lua, (url, data): (String, Table)| {
        let json_value = lua_table_to_json(lua, &data)?;
        let body = serde_json::to_string(&json_value)
            .map_err(|e| mlua::Error::external(e))?;
        
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        let response = client.put(&url, Some(body), Some(headers))
            .map_err(|e| mlua::Error::external(e))?;
        create_response_table(lua, response)
    })?;
    table.set("putJson", put_json_fn)?;
    Ok(())
}

fn create_response_table<'lua>(lua: &'lua Lua, response: HttpResponse) -> mlua::Result<Table<'lua>> {
    let table = lua.create_table()?;
    
    table.set("status", response.status)?;
    table.set("statusText", response.status_text.clone())?;
    table.set("body", response.body.clone())?;
    
    let headers = lua.create_table()?;
    for (k, v) in &response.headers {
        headers.set(k.as_str(), v.as_str())?;
    }
    table.set("headers", headers)?;
    
    let body_for_text = response.body.clone();
    table.set("text", lua.create_function(move |_, ()| {
        Ok(body_for_text.clone())
    })?)?;
    
    let body_for_json = response.body.clone();
    table.set("json", lua.create_function(move |lua, ()| {
        let json: JsonValue = serde_json::from_str(&body_for_json)
            .map_err(|e| mlua::Error::external(e))?;
        json_to_lua_value(lua, &json)
    })?)?;
    
    let status = response.status;
    table.set("ok", lua.create_function(move |_, ()| {
        Ok(status >= 200 && status < 300)
    })?)?;
    
    Ok(table)
}

struct FetchOptions {
    method: String,
    body: Option<String>,
    headers: Option<HashMap<String, String>>,
    timeout: Option<u64>,
}

fn parse_fetch_options(options: Option<Table>) -> mlua::Result<FetchOptions> {
    let Some(opts) = options else {
        return Ok(FetchOptions {
            method: "GET".to_string(),
            body: None,
            headers: None,
            timeout: None,
        });
    };
    
    let method = opts.get::<_, Option<String>>("method")?
        .unwrap_or_else(|| "GET".to_string())
        .to_uppercase();
    
    let body = opts.get::<_, Option<String>>("body")?;
    let headers = parse_headers(&opts)?;
    let timeout = opts.get::<_, Option<u64>>("timeout")?;
    
    Ok(FetchOptions {
        method,
        body,
        headers,
        timeout,
    })
}

fn parse_request_options(options: Option<Table>) -> mlua::Result<(Option<String>, Option<HashMap<String, String>>)> {
    let Some(opts) = options else {
        return Ok((None, None));
    };
    
    let body = opts.get::<_, Option<String>>("body")?;
    let headers = parse_headers(&opts)?;
    
    Ok((body, headers))
}

fn parse_headers(opts: &Table) -> mlua::Result<Option<HashMap<String, String>>> {
    let headers_table: Option<Table> = opts.get("headers")?;
    
    let Some(headers_table) = headers_table else {
        return Ok(None);
    };
    
    let mut headers = HashMap::new();
    for pair in headers_table.pairs::<String, String>() {
        let (key, value) = pair?;
        headers.insert(key, value);
    }
    
    if headers.is_empty() {
        Ok(None)
    } else {
        Ok(Some(headers))
    }
}

fn json_to_lua_value<'lua>(lua: &'lua Lua, value: &JsonValue) -> mlua::Result<Value<'lua>> {
    match value {
        JsonValue::Null => Ok(Value::Nil),
        JsonValue::Bool(b) => Ok(Value::Boolean(*b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::Number(f))
            } else {
                Ok(Value::Number(0.0))
            }
        }
        JsonValue::String(s) => Ok(Value::String(lua.create_string(s)?)),
        JsonValue::Array(arr) => {
            let table = lua.create_table()?;
            for (i, item) in arr.iter().enumerate() {
                let lua_val = json_to_lua_value(lua, item)?;
                table.set(i + 1, lua_val)?;
            }
            Ok(Value::Table(table))
        }
        JsonValue::Object(obj) => {
            let table = lua.create_table()?;
            for (key, val) in obj.iter() {
                let lua_val = json_to_lua_value(lua, val)?;
                table.set(key.as_str(), lua_val)?;
            }
            Ok(Value::Table(table))
        }
    }
}

fn lua_table_to_json<'lua>(lua: &'lua Lua, table: &Table<'lua>) -> mlua::Result<JsonValue> {
    let len = table.raw_len();
    
    if len > 0 {
        let mut array = Vec::new();
        for i in 1..=len {
            let value: Value = table.get(i)?;
            array.push(lua_value_to_json(lua, value)?);
        }
        Ok(JsonValue::Array(array))
    } else {
        let mut object = serde_json::Map::new();
        for pair in table.clone().pairs::<Value, Value>() {
            let (key, value) = pair?;
            let key_str = match key {
                Value::String(s) => s.to_str()?.to_string(),
                Value::Integer(i) => i.to_string(),
                Value::Number(n) => n.to_string(),
                _ => continue,
            };
            object.insert(key_str, lua_value_to_json(lua, value)?);
        }
        Ok(JsonValue::Object(object))
    }
}

fn lua_value_to_json(lua: &Lua, value: Value) -> mlua::Result<JsonValue> {
    match value {
        Value::Nil => Ok(JsonValue::Null),
        Value::Boolean(b) => Ok(JsonValue::Bool(b)),
        Value::Integer(i) => Ok(JsonValue::Number(serde_json::Number::from(i))),
        Value::Number(n) => {
            if let Some(num) = serde_json::Number::from_f64(n) {
                Ok(JsonValue::Number(num))
            } else {
                Ok(JsonValue::Null)
            }
        }
        Value::String(s) => Ok(JsonValue::String(s.to_str()?.to_string())),
        Value::Table(t) => lua_table_to_json(lua, &t),
        _ => Ok(JsonValue::Null),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_http_module() {
        let lua = Lua::new();
        let result = create_http_module(&lua);
        
        #[cfg(feature = "http")]
        {
            assert!(result.is_ok());
            let http_table = result.unwrap();
            assert!(http_table.contains_key("get").unwrap());
            assert!(http_table.contains_key("post").unwrap());
            assert!(http_table.contains_key("put").unwrap());
            assert!(http_table.contains_key("delete").unwrap());
            assert!(http_table.contains_key("fetch").unwrap());
            assert!(http_table.contains_key("postJson").unwrap());
            assert!(http_table.contains_key("putJson").unwrap());
        }
        
        #[cfg(not(feature = "http"))]
        {
            assert!(result.is_ok());
        }
    }
    
    #[test]
    fn test_json_to_lua_value() {
        let lua = Lua::new();
        
        let json = serde_json::json!({
            "name": "test",
            "value": 42,
            "active": true,
            "data": [1, 2, 3]
        });
        
        let result = json_to_lua_value(&lua, &json);
        assert!(result.is_ok());
        
        if let Ok(Value::Table(table)) = result {
            assert!(table.contains_key("name").unwrap());
            assert!(table.contains_key("value").unwrap());
            assert!(table.contains_key("active").unwrap());
            assert!(table.contains_key("data").unwrap());
        } else {
            panic!("Expected table");
        }
    }
    
    #[test]
    fn test_lua_table_to_json() {
        let lua = Lua::new();
        let table = lua.create_table().unwrap();
        table.set("name", "test").unwrap();
        table.set("value", 42).unwrap();
        table.set("active", true).unwrap();
        
        let result = lua_table_to_json(&lua, &table);
        assert!(result.is_ok());
        
        let json = result.unwrap();
        assert!(json.is_object());
        assert_eq!(json["name"], "test");
        assert_eq!(json["value"], 42);
        assert_eq!(json["active"], true);
    }
}
