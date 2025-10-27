pub mod error;
pub mod lua_bindings;
pub mod operations;

use serde_json::{json, Value as JsonValue};

use super::BuiltinModule;
use crate::error::HypeError;

pub use error::CryptoError;
pub use lua_bindings::create_crypto_module;
pub use operations::*;

pub struct CryptoModule;

impl CryptoModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CryptoModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinModule for CryptoModule {
    fn name(&self) -> &str {
        "crypto"
    }

    fn exports(&self) -> std::result::Result<JsonValue, HypeError> {
        Ok(json!({
            "__id": "crypto",
            "__desc": "Cryptographic operations for hashing, encryption, and random generation",
            "hash": {
                "__fn": "hash",
                "__desc": "Hash data using specified algorithm (sha256, sha512, sha1, md5)",
                "__signature": "hash(algorithm: string, data: string) -> string"
            },
            "hashFile": {
                "__fn": "hashFile",
                "__desc": "Hash file contents",
                "__signature": "hashFile(algorithm: string, path: string) -> string"
            },
            "hmac": {
                "__fn": "hmac",
                "__desc": "HMAC signing with key",
                "__signature": "hmac(algorithm: string, key: string, data: string) -> string"
            },
            "randomBytes": {
                "__fn": "randomBytes",
                "__desc": "Generate cryptographically secure random bytes",
                "__signature": "randomBytes(size: number) -> table"
            },
            "randomInt": {
                "__fn": "randomInt",
                "__desc": "Generate secure random integer in range [min, max)",
                "__signature": "randomInt(min: number, max: number) -> number"
            },
            "randomUUID": {
                "__fn": "randomUUID",
                "__desc": "Generate UUID v4",
                "__signature": "randomUUID() -> string"
            },
            "base64Encode": {
                "__fn": "base64Encode",
                "__desc": "Base64 encode string",
                "__signature": "base64Encode(data: string) -> string"
            },
            "base64Decode": {
                "__fn": "base64Decode",
                "__desc": "Base64 decode string",
                "__signature": "base64Decode(data: string) -> string"
            },
            "hexEncode": {
                "__fn": "hexEncode",
                "__desc": "Hex encode string",
                "__signature": "hexEncode(data: string) -> string"
            },
            "hexDecode": {
                "__fn": "hexDecode",
                "__desc": "Hex decode string",
                "__signature": "hexDecode(data: string) -> string"
            },
            "bcrypt": {
                "__fn": "bcrypt",
                "__desc": "Hash password with bcrypt",
                "__signature": "bcrypt(password: string, cost?: number) -> string"
            },
            "bcryptVerify": {
                "__fn": "bcryptVerify",
                "__desc": "Verify password against bcrypt hash",
                "__signature": "bcryptVerify(password: string, hash: string) -> boolean"
            },
            "timingSafeEqual": {
                "__fn": "timingSafeEqual",
                "__desc": "Constant-time string comparison",
                "__signature": "timingSafeEqual(a: string, b: string) -> boolean"
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_module_name() {
        let module = CryptoModule::new();
        assert_eq!(module.name(), "crypto");
    }

    #[test]
    fn test_crypto_module_exports() {
        let module = CryptoModule::new();
        let exports = module.exports().unwrap();
        assert!(exports.is_object());
        assert!(exports.get("hash").is_some());
        assert!(exports.get("hashFile").is_some());
        assert!(exports.get("hmac").is_some());
        assert!(exports.get("randomBytes").is_some());
        assert!(exports.get("randomInt").is_some());
        assert!(exports.get("randomUUID").is_some());
        assert!(exports.get("base64Encode").is_some());
        assert!(exports.get("base64Decode").is_some());
        assert!(exports.get("hexEncode").is_some());
        assert!(exports.get("hexDecode").is_some());
        assert!(exports.get("bcrypt").is_some());
        assert!(exports.get("bcryptVerify").is_some());
        assert!(exports.get("timingSafeEqual").is_some());
    }

    #[test]
    fn test_crypto_module_default() {
        let module = CryptoModule::default();
        assert_eq!(module.name(), "crypto");
    }
}
