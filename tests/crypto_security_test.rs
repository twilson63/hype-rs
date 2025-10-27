use hype_rs::modules::builtins::crypto::operations::*;
use std::collections::HashSet;

#[test]
fn test_hash_empty_input() {
    let result = hash("sha256", b"").unwrap();
    assert_eq!(
        result,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn test_hash_large_input() {
    let large_data = vec![b'a'; 1_000_000];
    let result = hash("sha256", &large_data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 64);
}

#[test]
fn test_hash_binary_data() {
    let binary_data: Vec<u8> = (0..=255).collect();
    let result = hash("sha256", &binary_data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 64);
}

#[test]
fn test_hash_unicode() {
    let unicode = "Hello 世界 🌍 Ñoño";
    let result = hash("sha256", unicode.as_bytes());
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 64);
}

#[test]
fn test_hash_deterministic() {
    let data = b"test data";
    let hash1 = hash("sha256", data).unwrap();
    let hash2 = hash("sha256", data).unwrap();
    assert_eq!(hash1, hash2, "Hash should be deterministic");
}

#[test]
fn test_hash_different_algorithms_different_outputs() {
    let data = b"test";
    let sha256 = hash("sha256", data).unwrap();
    let sha512 = hash("sha512", data).unwrap();
    let md5 = hash("md5", data).unwrap();
    let sha1 = hash("sha1", data).unwrap();

    assert_ne!(sha256, sha512);
    assert_ne!(sha256, md5);
    assert_ne!(sha256, sha1);
    assert_ne!(sha512, md5);
}

#[test]
fn test_hash_avalanche_effect() {
    let data1 = b"test";
    let data2 = b"Test";
    let hash1 = hash("sha256", data1).unwrap();
    let hash2 = hash("sha256", data2).unwrap();
    assert_ne!(
        hash1, hash2,
        "Single bit change should produce completely different hash"
    );
}

#[test]
fn test_hmac_empty_key() {
    let result = hmac_sign("sha256", b"", b"data");
    assert!(result.is_ok());
}

#[test]
fn test_hmac_empty_data() {
    let result = hmac_sign("sha256", b"key", b"");
    assert!(result.is_ok());
}

#[test]
fn test_hmac_deterministic() {
    let key = b"secret";
    let data = b"message";
    let hmac1 = hmac_sign("sha256", key, data).unwrap();
    let hmac2 = hmac_sign("sha256", key, data).unwrap();
    assert_eq!(hmac1, hmac2, "HMAC should be deterministic");
}

#[test]
fn test_hmac_different_keys() {
    let data = b"message";
    let hmac1 = hmac_sign("sha256", b"key1", data).unwrap();
    let hmac2 = hmac_sign("sha256", b"key2", data).unwrap();
    assert_ne!(hmac1, hmac2, "Different keys should produce different HMACs");
}

#[test]
fn test_hmac_key_sensitivity() {
    let data = b"message";
    let hmac1 = hmac_sign("sha256", b"secret", data).unwrap();
    let hmac2 = hmac_sign("sha256", b"Secret", data).unwrap();
    assert_ne!(
        hmac1, hmac2,
        "HMAC should be sensitive to key case changes"
    );
}

#[test]
fn test_random_bytes_distribution() {
    let size = 100;
    let bytes = random_bytes(size).unwrap();
    let mut all_same = true;
    let first = bytes[0];
    for &byte in &bytes {
        if byte != first {
            all_same = false;
            break;
        }
    }
    assert!(!all_same, "Random bytes should not all be the same");
}

#[test]
fn test_random_bytes_uniqueness() {
    let bytes1 = random_bytes(32).unwrap();
    let bytes2 = random_bytes(32).unwrap();
    assert_ne!(bytes1, bytes2, "Random bytes should be unique");
}

#[test]
fn test_random_int_distribution() {
    let mut results = HashSet::new();
    for _ in 0..100 {
        let num = random_int(0, 10).unwrap();
        results.insert(num);
    }
    assert!(
        results.len() > 1,
        "Random ints should have some distribution"
    );
}

#[test]
fn test_random_int_bounds() {
    for _ in 0..100 {
        let num = random_int(10, 20).unwrap();
        assert!(num >= 10 && num < 20, "Random int out of bounds: {}", num);
    }
}

#[test]
fn test_random_int_negative_range() {
    let num = random_int(-100, -50).unwrap();
    assert!(num >= -100 && num < -50);
}

#[test]
fn test_random_int_zero_boundary() {
    let num = random_int(-10, 10).unwrap();
    assert!(num >= -10 && num < 10);
}

#[test]
fn test_random_uuid_format() {
    let uuid = random_uuid();
    assert_eq!(uuid.len(), 36);
    let parts: Vec<&str> = uuid.split('-').collect();
    assert_eq!(parts.len(), 5);
    assert_eq!(parts[0].len(), 8);
    assert_eq!(parts[1].len(), 4);
    assert_eq!(parts[2].len(), 4);
    assert_eq!(parts[3].len(), 4);
    assert_eq!(parts[4].len(), 12);
}

#[test]
fn test_random_uuid_version() {
    let uuid = random_uuid();
    let chars: Vec<char> = uuid.chars().collect();
    assert_eq!(chars[14], '4', "Should be UUIDv4");
}

#[test]
fn test_random_uuid_uniqueness() {
    let mut uuids = HashSet::new();
    for _ in 0..1000 {
        let uuid = random_uuid();
        assert!(!uuids.contains(&uuid), "UUID collision detected!");
        uuids.insert(uuid);
    }
}

#[test]
fn test_base64_padding() {
    let test_cases = vec![
        ("a", "YQ=="),
        ("ab", "YWI="),
        ("abc", "YWJj"),
        ("abcd", "YWJjZA=="),
    ];
    for (input, expected) in test_cases {
        assert_eq!(base64_encode(input.as_bytes()), expected);
    }
}

#[test]
fn test_base64_special_chars() {
    let special = "!@#$%^&*()_+-=[]{}|;:,.<>?/";
    let encoded = base64_encode(special.as_bytes());
    let decoded = base64_decode(&encoded).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), special);
}

#[test]
fn test_base64_unicode() {
    let unicode = "Hello 世界 🌍";
    let encoded = base64_encode(unicode.as_bytes());
    let decoded = base64_decode(&encoded).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), unicode);
}

#[test]
fn test_base64_binary() {
    let binary: Vec<u8> = (0..=255).collect();
    let encoded = base64_encode(&binary);
    let decoded = base64_decode(&encoded).unwrap();
    assert_eq!(decoded, binary);
}

#[test]
fn test_base64_invalid_input() {
    let invalid = "SGVsbG8=!!!";
    let result = base64_decode(invalid);
    assert!(result.is_err(), "Should fail on invalid base64");
}

#[test]
fn test_hex_lowercase() {
    let result = hex_encode(b"HELLO");
    assert!(result.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
}

#[test]
fn test_hex_all_bytes() {
    let bytes: Vec<u8> = (0..=255).collect();
    let encoded = hex_encode(&bytes);
    let decoded = hex_decode(&encoded).unwrap();
    assert_eq!(decoded, bytes);
}

#[test]
fn test_hex_invalid_chars() {
    let invalid = "48656c6c6g";
    let result = hex_decode(invalid);
    assert!(result.is_err(), "Should fail on invalid hex character");
}

#[test]
fn test_hex_odd_length() {
    let invalid = "48656c6c6";
    let result = hex_decode(invalid);
    assert!(result.is_err(), "Should fail on odd-length hex string");
}

#[test]
fn test_bcrypt_different_costs() {
    let password = "test123";
    let hash4 = bcrypt_hash_password(password, Some(4)).unwrap();
    let hash5 = bcrypt_hash_password(password, Some(5)).unwrap();
    assert_ne!(hash4, hash5, "Different costs should produce different hashes");
}

#[test]
fn test_bcrypt_salt_randomness() {
    let password = "test123";
    let hash1 = bcrypt_hash_password(password, Some(4)).unwrap();
    let hash2 = bcrypt_hash_password(password, Some(4)).unwrap();
    assert_ne!(
        hash1, hash2,
        "Same password should produce different hashes due to random salt"
    );
}

#[test]
fn test_bcrypt_verify_case_sensitive() {
    let password = "Password123";
    let hash = bcrypt_hash_password(password, Some(4)).unwrap();
    assert!(bcrypt_verify_password(password, &hash).unwrap());
    assert!(!bcrypt_verify_password("password123", &hash).unwrap());
}

#[test]
fn test_bcrypt_empty_password() {
    let hash = bcrypt_hash_password("", Some(4)).unwrap();
    assert!(bcrypt_verify_password("", &hash).unwrap());
    assert!(!bcrypt_verify_password("a", &hash).unwrap());
}

#[test]
fn test_bcrypt_long_password() {
    let long_password = "a".repeat(100);
    let hash = bcrypt_hash_password(&long_password, Some(4)).unwrap();
    assert!(bcrypt_verify_password(&long_password, &hash).unwrap());
}

#[test]
fn test_bcrypt_special_chars() {
    let password = "P@ssw0rd!#$%^&*()_+-=[]{}|;:,.<>?/";
    let hash = bcrypt_hash_password(password, Some(4)).unwrap();
    assert!(bcrypt_verify_password(password, &hash).unwrap());
}

#[test]
fn test_bcrypt_unicode_password() {
    let password = "пароль密码🔐";
    let hash = bcrypt_hash_password(password, Some(4)).unwrap();
    assert!(bcrypt_verify_password(password, &hash).unwrap());
}

#[test]
fn test_timing_safe_equal_empty_strings() {
    assert!(timing_safe_equal(b"", b""));
}

#[test]
fn test_timing_safe_equal_single_char_diff() {
    assert!(!timing_safe_equal(b"a", b"b"));
}

#[test]
fn test_timing_safe_equal_similar_strings() {
    assert!(!timing_safe_equal(b"password", b"Password"));
    assert!(!timing_safe_equal(b"admin123", b"admin124"));
}

#[test]
fn test_timing_safe_equal_null_bytes() {
    assert!(timing_safe_equal(b"\0\0\0", b"\0\0\0"));
    assert!(!timing_safe_equal(b"\0\0\0", b"\0\0\x01"));
}

#[test]
fn test_timing_safe_equal_long_strings() {
    let s1 = vec![0u8; 10000];
    let s2 = vec![0u8; 10000];
    let mut s3 = vec![0u8; 10000];
    s3[9999] = 1;

    assert!(timing_safe_equal(&s1, &s2));
    assert!(!timing_safe_equal(&s1, &s3));
}

#[test]
fn test_hash_collision_resistance() {
    let similar_inputs = vec![
        b"test".to_vec(),
        b"Test".to_vec(),
        b"test ".to_vec(),
        b"tes t".to_vec(),
    ];

    let mut hashes = HashSet::new();
    for input in similar_inputs {
        let hash_result = hash("sha256", &input).unwrap();
        assert!(
            !hashes.contains(&hash_result),
            "Hash collision detected for similar inputs"
        );
        hashes.insert(hash_result);
    }
}

#[test]
fn test_random_bytes_max_size() {
    let result = random_bytes(1024 * 1024);
    assert!(result.is_ok());
}

#[test]
fn test_random_bytes_exceeds_max() {
    let result = random_bytes(1024 * 1024 + 1);
    assert!(result.is_err());
}

#[test]
fn test_bcrypt_min_cost_boundary() {
    let result = bcrypt_hash_password("test", Some(3));
    assert!(result.is_err(), "Should reject cost below 4");
}

#[test]
fn test_bcrypt_max_cost_boundary() {
    let result = bcrypt_hash_password("test", Some(32));
    assert!(result.is_err(), "Should reject cost above 31");
}
