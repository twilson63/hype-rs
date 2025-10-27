use super::error::CryptoError;
use base64::{engine::general_purpose, Engine as _};
use bcrypt::{hash as bcrypt_hash, verify as bcrypt_verify, DEFAULT_COST};
use hmac::{Hmac, Mac};
use md5::Md5;
use rand::Rng;
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha512};
use std::fs;
use uuid::Uuid;

pub fn hash(algorithm: &str, data: &[u8]) -> Result<String, CryptoError> {
    match algorithm.to_lowercase().as_str() {
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(data);
            Ok(hex::encode(hasher.finalize()))
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(data);
            Ok(hex::encode(hasher.finalize()))
        }
        "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(data);
            Ok(hex::encode(hasher.finalize()))
        }
        "md5" => {
            let mut hasher = Md5::new();
            hasher.update(data);
            Ok(hex::encode(hasher.finalize()))
        }
        _ => Err(CryptoError::InvalidAlgorithm(format!(
            "Unsupported algorithm: {}. Supported: sha256, sha512, sha1, md5",
            algorithm
        ))),
    }
}

pub fn hash_file(algorithm: &str, path: &str) -> Result<String, CryptoError> {
    let data = fs::read(path).map_err(|e| CryptoError::FileError(e.to_string()))?;
    hash(algorithm, &data)
}

pub fn hmac_sign(algorithm: &str, key: &[u8], data: &[u8]) -> Result<String, CryptoError> {
    match algorithm.to_lowercase().as_str() {
        "sha256" => {
            type HmacSha256 = Hmac<Sha256>;
            let mut mac = HmacSha256::new_from_slice(key)
                .map_err(|e| CryptoError::HashError(e.to_string()))?;
            mac.update(data);
            Ok(hex::encode(mac.finalize().into_bytes()))
        }
        "sha512" => {
            type HmacSha512 = Hmac<Sha512>;
            let mut mac = HmacSha512::new_from_slice(key)
                .map_err(|e| CryptoError::HashError(e.to_string()))?;
            mac.update(data);
            Ok(hex::encode(mac.finalize().into_bytes()))
        }
        "sha1" => {
            type HmacSha1 = Hmac<Sha1>;
            let mut mac =
                HmacSha1::new_from_slice(key).map_err(|e| CryptoError::HashError(e.to_string()))?;
            mac.update(data);
            Ok(hex::encode(mac.finalize().into_bytes()))
        }
        _ => Err(CryptoError::InvalidAlgorithm(format!(
            "Unsupported HMAC algorithm: {}. Supported: sha256, sha512, sha1",
            algorithm
        ))),
    }
}

pub fn random_bytes(size: usize) -> Result<Vec<u8>, CryptoError> {
    if size == 0 || size > 1024 * 1024 {
        return Err(CryptoError::InvalidInput(
            "Size must be between 1 and 1048576 bytes".to_string(),
        ));
    }
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..size).map(|_| rng.gen()).collect();
    Ok(bytes)
}

pub fn random_int(min: i64, max: i64) -> Result<i64, CryptoError> {
    if min >= max {
        return Err(CryptoError::InvalidInput(
            "min must be less than max".to_string(),
        ));
    }
    let mut rng = rand::thread_rng();
    Ok(rng.gen_range(min..max))
}

pub fn random_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn base64_encode(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

pub fn base64_decode(data: &str) -> Result<Vec<u8>, CryptoError> {
    general_purpose::STANDARD
        .decode(data)
        .map_err(|e| CryptoError::DecodeError(e.to_string()))
}

pub fn hex_encode(data: &[u8]) -> String {
    hex::encode(data)
}

pub fn hex_decode(data: &str) -> Result<Vec<u8>, CryptoError> {
    hex::decode(data).map_err(|e| CryptoError::DecodeError(e.to_string()))
}

pub fn bcrypt_hash_password(password: &str, cost: Option<u32>) -> Result<String, CryptoError> {
    let cost = cost.unwrap_or(DEFAULT_COST);
    if cost < 4 || cost > 31 {
        return Err(CryptoError::InvalidInput(
            "Cost must be between 4 and 31".to_string(),
        ));
    }
    bcrypt_hash(password, cost).map_err(|e| CryptoError::BcryptError(e.to_string()))
}

pub fn bcrypt_verify_password(password: &str, hash: &str) -> Result<bool, CryptoError> {
    bcrypt_verify(password, hash).map_err(|e| CryptoError::BcryptError(e.to_string()))
}

pub fn timing_safe_equal(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (byte_a, byte_b) in a.iter().zip(b.iter()) {
        result |= byte_a ^ byte_b;
    }
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_sha256() {
        let result = hash("sha256", b"hello").unwrap();
        assert_eq!(
            result,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_hash_md5() {
        let result = hash("md5", b"hello").unwrap();
        assert_eq!(result, "5d41402abc4b2a76b9719d911017c592");
    }

    #[test]
    fn test_hash_invalid_algorithm() {
        let result = hash("invalid", b"hello");
        assert!(result.is_err());
    }

    #[test]
    fn test_hmac_sha256() {
        let result = hmac_sign("sha256", b"secret", b"hello").unwrap();
        assert_eq!(
            result,
            "88aab3ede8d3adf94d26ab90d3bafd4a2083070c3bcce9c014ee04a443847c0b"
        );
    }

    #[test]
    fn test_random_bytes() {
        let bytes = random_bytes(16).unwrap();
        assert_eq!(bytes.len(), 16);
    }

    #[test]
    fn test_random_bytes_invalid_size() {
        let result = random_bytes(0);
        assert!(result.is_err());
        let result = random_bytes(2_000_000);
        assert!(result.is_err());
    }

    #[test]
    fn test_random_int() {
        let num = random_int(1, 100).unwrap();
        assert!(num >= 1 && num < 100);
    }

    #[test]
    fn test_random_int_invalid_range() {
        let result = random_int(10, 10);
        assert!(result.is_err());
        let result = random_int(100, 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_random_uuid() {
        let uuid = random_uuid();
        assert_eq!(uuid.len(), 36);
        assert!(uuid.contains('-'));
    }

    #[test]
    fn test_base64_encode() {
        let result = base64_encode(b"hello");
        assert_eq!(result, "aGVsbG8=");
    }

    #[test]
    fn test_base64_decode() {
        let result = base64_decode("aGVsbG8=").unwrap();
        assert_eq!(result, b"hello");
    }

    #[test]
    fn test_base64_roundtrip() {
        let original = b"Hello, World! 123";
        let encoded = base64_encode(original);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_hex_encode() {
        let result = hex_encode(b"hello");
        assert_eq!(result, "68656c6c6f");
    }

    #[test]
    fn test_hex_decode() {
        let result = hex_decode("68656c6c6f").unwrap();
        assert_eq!(result, b"hello");
    }

    #[test]
    fn test_hex_roundtrip() {
        let original = b"Test data 123!";
        let encoded = hex_encode(original);
        let decoded = hex_decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_bcrypt_hash() {
        let hash = bcrypt_hash_password("password123", Some(4)).unwrap();
        assert!(hash.starts_with("$2"));
        assert!(hash.len() > 50);
    }

    #[test]
    fn test_bcrypt_verify() {
        let hash = bcrypt_hash_password("password123", Some(4)).unwrap();
        assert!(bcrypt_verify_password("password123", &hash).unwrap());
        assert!(!bcrypt_verify_password("wrongpassword", &hash).unwrap());
    }

    #[test]
    fn test_bcrypt_invalid_cost() {
        let result = bcrypt_hash_password("password", Some(3));
        assert!(result.is_err());
        let result = bcrypt_hash_password("password", Some(32));
        assert!(result.is_err());
    }

    #[test]
    fn test_timing_safe_equal_same() {
        assert!(timing_safe_equal(b"hello", b"hello"));
    }

    #[test]
    fn test_timing_safe_equal_different() {
        assert!(!timing_safe_equal(b"hello", b"world"));
    }

    #[test]
    fn test_timing_safe_equal_different_length() {
        assert!(!timing_safe_equal(b"hello", b"helloworld"));
    }
}
