use hype_rs::lua::require::setup_require_fn;
use hype_rs::modules::loader::ModuleLoader;
use mlua::Lua;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

fn setup_lua() -> Lua {
    let lua = Lua::new();
    let loader = Arc::new(Mutex::new(ModuleLoader::new(PathBuf::from("."))));
    setup_require_fn(&lua, loader).unwrap();
    lua
}

#[test]
fn test_crypto_hash_sha256() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local result = crypto.hash("sha256", "hello")
assert(result == "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_hash_md5() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local result = crypto.hash("md5", "hello")
assert(result == "5d41402abc4b2a76b9719d911017c592")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_hmac() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local result = crypto.hmac("sha256", "secret", "hello")
assert(result == "88aab3ede8d3adf94d26ab90d3bafd4a2083070c3bcce9c014ee04a443847c0b")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_random_bytes() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local bytes = crypto.randomBytes(16)
assert(type(bytes) == "table")
local count = 0
for _ in pairs(bytes) do count = count + 1 end
assert(count == 16)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_random_int() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local num = crypto.randomInt(1, 100)
assert(type(num) == "number")
assert(num >= 1 and num < 100)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_random_uuid() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local uuid = crypto.randomUUID()
assert(type(uuid) == "string")
assert(#uuid == 36)
assert(string.find(uuid, "-", 1, true) ~= nil)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_base64_encode() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local encoded = crypto.base64Encode("hello")
assert(encoded == "aGVsbG8=")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_base64_decode() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local decoded = crypto.base64Decode("aGVsbG8=")
assert(decoded == "hello")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_base64_roundtrip() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local original = "Hello, World! 123"
local encoded = crypto.base64Encode(original)
local decoded = crypto.base64Decode(encoded)
assert(decoded == original)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_hex_encode() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local encoded = crypto.hexEncode("hello")
assert(encoded == "68656c6c6f")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_hex_decode() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local decoded = crypto.hexDecode("68656c6c6f")
assert(decoded == "hello")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_hex_roundtrip() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local original = "Test data 123!"
local encoded = crypto.hexEncode(original)
local decoded = crypto.hexDecode(encoded)
assert(decoded == original)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_bcrypt() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local hash = crypto.bcrypt("password123", 4)
assert(type(hash) == "string")
assert(#hash > 50)
assert(string.sub(hash, 1, 3) == "$2b" or string.sub(hash, 1, 3) == "$2y")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_bcrypt_verify() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local hash = crypto.bcrypt("password123", 4)
assert(crypto.bcryptVerify("password123", hash) == true)
assert(crypto.bcryptVerify("wrongpassword", hash) == false)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_timing_safe_equal() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
assert(crypto.timingSafeEqual("hello", "hello") == true)
assert(crypto.timingSafeEqual("hello", "world") == false)
assert(crypto.timingSafeEqual("test", "testing") == false)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_require_loads_module() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
assert(type(crypto) == "table")
assert(type(crypto.hash) == "function")
assert(type(crypto.hashFile) == "function")
assert(type(crypto.hmac) == "function")
assert(type(crypto.randomBytes) == "function")
assert(type(crypto.randomInt) == "function")
assert(type(crypto.randomUUID) == "function")
assert(type(crypto.base64Encode) == "function")
assert(type(crypto.base64Decode) == "function")
assert(type(crypto.hexEncode) == "function")
assert(type(crypto.hexDecode) == "function")
assert(type(crypto.bcrypt) == "function")
assert(type(crypto.bcryptVerify) == "function")
assert(type(crypto.timingSafeEqual) == "function")
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_multiple_hashes() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local data = "test data"
local sha256 = crypto.hash("sha256", data)
local sha512 = crypto.hash("sha512", data)
local md5 = crypto.hash("md5", data)
assert(#sha256 == 64)
assert(#sha512 == 128)
assert(#md5 == 32)
"#,
    )
    .exec()
    .unwrap();
}

#[test]
fn test_crypto_uuid_uniqueness() {
    let lua = setup_lua();
    lua.load(
        r#"
local crypto = require("crypto")
local uuid1 = crypto.randomUUID()
local uuid2 = crypto.randomUUID()
assert(uuid1 ~= uuid2)
"#,
    )
    .exec()
    .unwrap();
}
