use super::operations::*;
use mlua::{Lua, Result as LuaResult, Table, Value};

pub fn create_crypto_module(lua: &Lua) -> LuaResult<Table> {
    let crypto = lua.create_table()?;

    let hash_fn = lua.create_function(|_, (algorithm, data): (String, String)| {
        match hash(&algorithm, data.as_bytes()) {
            Ok(result) => Ok(result),
            Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
        }
    })?;
    crypto.set("hash", hash_fn)?;

    let hash_file_fn =
        lua.create_function(|_, (algorithm, path): (String, String)| {
            match hash_file(&algorithm, &path) {
                Ok(result) => Ok(result),
                Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
            }
        })?;
    crypto.set("hashFile", hash_file_fn)?;

    let hmac_fn =
        lua.create_function(
            |_, (algorithm, key, data): (String, String, String)| match hmac_sign(
                &algorithm,
                key.as_bytes(),
                data.as_bytes(),
            ) {
                Ok(result) => Ok(result),
                Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
            },
        )?;
    crypto.set("hmac", hmac_fn)?;

    let random_bytes_fn = lua.create_function(|lua, size: usize| match random_bytes(size) {
        Ok(bytes) => {
            let table = lua.create_table()?;
            for (i, byte) in bytes.iter().enumerate() {
                table.set(i + 1, *byte as i32)?;
            }
            Ok(table)
        }
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    crypto.set("randomBytes", random_bytes_fn)?;

    let random_int_fn =
        lua.create_function(|_, (min, max): (i64, i64)| match random_int(min, max) {
            Ok(result) => Ok(result),
            Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
        })?;
    crypto.set("randomInt", random_int_fn)?;

    let random_uuid_fn = lua.create_function(|_, ()| Ok(random_uuid()))?;
    crypto.set("randomUUID", random_uuid_fn)?;

    let base64_encode_fn =
        lua.create_function(|_, data: String| Ok(base64_encode(data.as_bytes())))?;
    crypto.set("base64Encode", base64_encode_fn)?;

    let base64_decode_fn = lua.create_function(|_, data: String| match base64_decode(&data) {
        Ok(bytes) => String::from_utf8(bytes)
            .map_err(|e| mlua::Error::RuntimeError(format!("Invalid UTF-8: {}", e))),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    crypto.set("base64Decode", base64_decode_fn)?;

    let hex_encode_fn = lua.create_function(|_, data: String| Ok(hex_encode(data.as_bytes())))?;
    crypto.set("hexEncode", hex_encode_fn)?;

    let hex_decode_fn = lua.create_function(|_, data: String| match hex_decode(&data) {
        Ok(bytes) => String::from_utf8(bytes)
            .map_err(|e| mlua::Error::RuntimeError(format!("Invalid UTF-8: {}", e))),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    crypto.set("hexDecode", hex_decode_fn)?;

    let bcrypt_fn =
        lua.create_function(
            |_, (password, cost): (String, Option<u32>)| match bcrypt_hash_password(&password, cost)
            {
                Ok(hash) => Ok(hash),
                Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
            },
        )?;
    crypto.set("bcrypt", bcrypt_fn)?;

    let bcrypt_verify_fn =
        lua.create_function(
            |_, (password, hash): (String, String)| match bcrypt_verify_password(&password, &hash) {
                Ok(result) => Ok(result),
                Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
            },
        )?;
    crypto.set("bcryptVerify", bcrypt_verify_fn)?;

    let timing_safe_equal_fn = lua.create_function(|_, (a, b): (String, String)| {
        Ok(timing_safe_equal(a.as_bytes(), b.as_bytes()))
    })?;
    crypto.set("timingSafeEqual", timing_safe_equal_fn)?;

    Ok(crypto)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua;

    #[test]
    fn test_create_crypto_module() {
        let lua = Lua::new();
        let result = create_crypto_module(&lua);
        assert!(result.is_ok());

        let crypto = result.unwrap();
        assert!(crypto.contains_key("hash").unwrap());
        assert!(crypto.contains_key("hashFile").unwrap());
        assert!(crypto.contains_key("hmac").unwrap());
        assert!(crypto.contains_key("randomBytes").unwrap());
        assert!(crypto.contains_key("randomInt").unwrap());
        assert!(crypto.contains_key("randomUUID").unwrap());
        assert!(crypto.contains_key("base64Encode").unwrap());
        assert!(crypto.contains_key("base64Decode").unwrap());
        assert!(crypto.contains_key("hexEncode").unwrap());
        assert!(crypto.contains_key("hexDecode").unwrap());
        assert!(crypto.contains_key("bcrypt").unwrap());
        assert!(crypto.contains_key("bcryptVerify").unwrap());
        assert!(crypto.contains_key("timingSafeEqual").unwrap());
    }

    #[test]
    fn test_crypto_hash() {
        let lua = Lua::new();
        let crypto = create_crypto_module(&lua).unwrap();
        lua.globals().set("crypto", crypto).unwrap();

        lua.load(
            r#"
local result = crypto.hash("sha256", "hello")
assert(result == "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")
"#,
        )
        .exec()
        .unwrap();
    }

    #[test]
    fn test_crypto_hmac() {
        let lua = Lua::new();
        let crypto = create_crypto_module(&lua).unwrap();
        lua.globals().set("crypto", crypto).unwrap();

        lua.load(
            r#"
local result = crypto.hmac("sha256", "secret", "hello")
assert(result == "88aab3ede8d3adf94d26ab90d3bafd4a2083070c3bcce9c014ee04a443847c0b")
"#,
        )
        .exec()
        .unwrap();
    }

    #[test]
    fn test_crypto_random_uuid() {
        let lua = Lua::new();
        let crypto = create_crypto_module(&lua).unwrap();
        lua.globals().set("crypto", crypto).unwrap();

        lua.load(
            r#"
local uuid = crypto.randomUUID()
assert(type(uuid) == "string")
assert(#uuid == 36)
"#,
        )
        .exec()
        .unwrap();
    }

    #[test]
    fn test_crypto_base64() {
        let lua = Lua::new();
        let crypto = create_crypto_module(&lua).unwrap();
        lua.globals().set("crypto", crypto).unwrap();

        lua.load(
            r#"
local encoded = crypto.base64Encode("hello")
assert(encoded == "aGVsbG8=")
local decoded = crypto.base64Decode(encoded)
assert(decoded == "hello")
"#,
        )
        .exec()
        .unwrap();
    }

    #[test]
    fn test_crypto_hex() {
        let lua = Lua::new();
        let crypto = create_crypto_module(&lua).unwrap();
        lua.globals().set("crypto", crypto).unwrap();

        lua.load(
            r#"
local encoded = crypto.hexEncode("hello")
assert(encoded == "68656c6c6f")
local decoded = crypto.hexDecode(encoded)
assert(decoded == "hello")
"#,
        )
        .exec()
        .unwrap();
    }

    #[test]
    fn test_crypto_timing_safe_equal() {
        let lua = Lua::new();
        let crypto = create_crypto_module(&lua).unwrap();
        lua.globals().set("crypto", crypto).unwrap();

        lua.load(
            r#"
assert(crypto.timingSafeEqual("hello", "hello") == true)
assert(crypto.timingSafeEqual("hello", "world") == false)
"#,
        )
        .exec()
        .unwrap();
    }
}
