# crypto - Cryptographic Operations

> **Security-focused module for hashing, encryption, random generation, and password management.**

## Table of Contents
- [Import](#import)
- [Hashing](#hashing)
- [HMAC](#hmac)
- [Random Generation](#random-generation)
- [Encoding](#encoding)
- [Password Security](#password-security)
- [Security Best Practices](#security-best-practices)

---

## Import

```lua
local crypto = require("crypto")
```

---

## Hashing

### crypto.hash(algorithm, data)

Hash data using the specified algorithm.

**Parameters:**
- `algorithm: string` - Hash algorithm: `"sha256"`, `"sha512"`, `"sha1"`, `"md5"`
- `data: string` - Data to hash

**Returns:** `string` - Hex-encoded hash

**Example:**
```lua
local crypto = require("crypto")

-- SHA256 (recommended)
local hash = crypto.hash("sha256", "hello")
print(hash)  -- 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824

-- Different algorithms
print(crypto.hash("sha512", "hello"))  -- 128 hex chars
print(crypto.hash("sha1", "hello"))    -- 40 hex chars
print(crypto.hash("md5", "hello"))     -- 32 hex chars
```

**Security Note:**
- ✅ Use `sha256` or `sha512` for security-critical applications
- ⚠️ `md5` and `sha1` are cryptographically weak - use only for checksums

---

### crypto.hashFile(algorithm, path)

Hash the contents of a file.

**Parameters:**
- `algorithm: string` - Hash algorithm
- `path: string` - Path to file

**Returns:** `string` - Hex-encoded hash

**Example:**
```lua
local crypto = require("crypto")
local fs = require("fs")

-- Create a test file
fs.write("test.txt", "file contents")

-- Hash the file
local hash = crypto.hashFile("sha256", "test.txt")
print("File hash:", hash)

-- Cleanup
fs.remove("test.txt")
```

**Use Cases:**
- File integrity verification
- Duplicate file detection
- Content-addressable storage

---

## HMAC

### crypto.hmac(algorithm, key, data)

Generate HMAC (Hash-based Message Authentication Code).

**Parameters:**
- `algorithm: string` - Hash algorithm: `"sha256"`, `"sha512"`, `"sha1"`
- `key: string` - Secret key
- `data: string` - Message to authenticate

**Returns:** `string` - Hex-encoded HMAC

**Example:**
```lua
local crypto = require("crypto")

-- Create HMAC signature
local key = "my-secret-key"
local message = "important data"
local signature = crypto.hmac("sha256", key, message)
print("Signature:", signature)

-- Verify message (use timing-safe comparison)
local received_sig = crypto.hmac("sha256", key, message)
local is_valid = crypto.timingSafeEqual(signature, received_sig)
print("Valid:", is_valid)  -- true
```

**Use Cases:**
- API request signing
- Message authentication
- JWT token creation
- Webhook verification

---

## Random Generation

### crypto.randomBytes(size)

Generate cryptographically secure random bytes.

**Parameters:**
- `size: number` - Number of bytes (1 to 1,048,576)

**Returns:** `table` - Array of bytes (1-indexed)

**Example:**
```lua
local crypto = require("crypto")

-- Generate 16 random bytes
local bytes = crypto.randomBytes(16)

-- Convert to hex string
local hex = {}
for i = 1, #bytes do
    table.insert(hex, string.format("%02x", bytes[i]))
end
local hex_string = table.concat(hex)
print("Random hex:", hex_string)

-- Generate API key
local api_key_bytes = crypto.randomBytes(32)
local api_key = crypto.hexEncode(table.concat(api_key_bytes))
print("API Key:", api_key)
```

---

### crypto.randomInt(min, max)

Generate a secure random integer in range [min, max).

**Parameters:**
- `min: number` - Minimum value (inclusive)
- `max: number` - Maximum value (exclusive)

**Returns:** `number` - Random integer

**Example:**
```lua
local crypto = require("crypto")

-- Random number 1-100
local num = crypto.randomInt(1, 101)
print("Random:", num)

-- Random dice roll
local roll = crypto.randomInt(1, 7)
print("Dice:", roll)

-- Negative range
local temp = crypto.randomInt(-10, 11)
print("Temperature:", temp)
```

---

### crypto.randomUUID()

Generate a UUID v4 (universally unique identifier).

**Returns:** `string` - UUID in format `xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx`

**Example:**
```lua
local crypto = require("crypto")

-- Generate UUIDs
local id1 = crypto.randomUUID()
local id2 = crypto.randomUUID()

print(id1)  -- e.g., "550e8400-e29b-41d4-a716-446655440000"
print(id2)  -- Different UUID

-- Use cases
local session_id = crypto.randomUUID()
local request_id = crypto.randomUUID()
local user_id = crypto.randomUUID()
```

---

## Encoding

### crypto.base64Encode(data)

Encode string to Base64.

**Parameters:**
- `data: string` - Data to encode

**Returns:** `string` - Base64-encoded string

**Example:**
```lua
local crypto = require("crypto")

-- Encode text
local encoded = crypto.base64Encode("Hello, World!")
print(encoded)  -- SGVsbG8sIFdvcmxkIQ==

-- Encode binary data
local binary = "\x00\x01\x02\x03"
local b64 = crypto.base64Encode(binary)
print("Binary:", b64)

-- Unicode support
local unicode = crypto.base64Encode("Hello 世界 🌍")
print("Unicode:", unicode)
```

---

### crypto.base64Decode(data)

Decode Base64 string.

**Parameters:**
- `data: string` - Base64-encoded string

**Returns:** `string` - Decoded data

**Example:**
```lua
local crypto = require("crypto")

-- Decode
local decoded = crypto.base64Decode("SGVsbG8sIFdvcmxkIQ==")
print(decoded)  -- Hello, World!

-- Roundtrip
local original = "Test data 123"
local encoded = crypto.base64Encode(original)
local result = crypto.base64Decode(encoded)
assert(result == original)
```

---

### crypto.hexEncode(data)

Encode string to hexadecimal.

**Parameters:**
- `data: string` - Data to encode

**Returns:** `string` - Hex-encoded string (lowercase)

**Example:**
```lua
local crypto = require("crypto")

local hex = crypto.hexEncode("hello")
print(hex)  -- 68656c6c6f

-- Encode bytes
local bytes = "\x00\xFF\x42"
print(crypto.hexEncode(bytes))  -- 00ff42
```

---

### crypto.hexDecode(data)

Decode hexadecimal string.

**Parameters:**
- `data: string` - Hex-encoded string

**Returns:** `string` - Decoded data

**Example:**
```lua
local crypto = require("crypto")

local decoded = crypto.hexDecode("68656c6c6f")
print(decoded)  -- hello

-- Roundtrip
local data = "Secret123"
local hex = crypto.hexEncode(data)
local result = crypto.hexDecode(hex)
assert(result == data)
```

---

## Password Security

### crypto.bcrypt(password, cost?)

Hash a password using bcrypt.

**Parameters:**
- `password: string` - Password to hash
- `cost?: number` - Cost factor 4-31 (default: 12)

**Returns:** `string` - Bcrypt hash (includes salt)

**Example:**
```lua
local crypto = require("crypto")

-- Hash password (production)
local hash = crypto.bcrypt("user_password_123", 12)
print("Hash:", hash)
-- $2b$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUW

-- Fast hashing (testing only)
local test_hash = crypto.bcrypt("test", 4)

-- Each call produces different hash (random salt)
local hash1 = crypto.bcrypt("same_password", 10)
local hash2 = crypto.bcrypt("same_password", 10)
assert(hash1 ~= hash2)  -- Different hashes, same password
```

**Cost Recommendations:**
- 4-6: Testing only
- 10: Fast but secure
- 12: Recommended for production (default)
- 14+: High security, slower

---

### crypto.bcryptVerify(password, hash)

Verify a password against a bcrypt hash.

**Parameters:**
- `password: string` - Password to verify
- `hash: string` - Bcrypt hash

**Returns:** `boolean` - `true` if password matches

**Example:**
```lua
local crypto = require("crypto")

-- Hash password
local hash = crypto.bcrypt("correct_password", 10)

-- Verify
local is_correct = crypto.bcryptVerify("correct_password", hash)
print(is_correct)  -- true

local is_wrong = crypto.bcryptVerify("wrong_password", hash)
print(is_wrong)  -- false

-- Login example
function login(username, password, stored_hash)
    if crypto.bcryptVerify(password, stored_hash) then
        return "Login successful"
    else
        return "Invalid password"
    end
end
```

---

### crypto.timingSafeEqual(a, b)

Compare two strings in constant time (prevents timing attacks).

**Parameters:**
- `a: string` - First string
- `b: string` - Second string

**Returns:** `boolean` - `true` if strings are equal

**Example:**
```lua
local crypto = require("crypto")

-- Safe comparison
local token1 = "secret_token_abc123"
local token2 = "secret_token_abc123"
local is_valid = crypto.timingSafeEqual(token1, token2)
print(is_valid)  -- true

-- Prevent timing attacks on tokens/signatures
function verify_api_signature(request_sig, secret)
    local expected = crypto.hmac("sha256", secret, request_data)
    return crypto.timingSafeEqual(request_sig, expected)
end

-- ⚠️ DO NOT use regular comparison for secrets
-- if token == stored_token then  -- VULNERABLE to timing attacks
-- if crypto.timingSafeEqual(token, stored_token) then  -- ✅ SAFE
```

---

## Security Best Practices

### ✅ DO

```lua
-- Use SHA256+ for hashing
local hash = crypto.hash("sha256", data)

-- Use bcrypt for passwords with cost 10+
local hash = crypto.bcrypt(password, 12)

-- Use HMAC for message authentication
local sig = crypto.hmac("sha256", secret, message)

-- Use timing-safe comparison for secrets
local valid = crypto.timingSafeEqual(sig1, sig2)

-- Generate secure random for tokens
local token = crypto.randomUUID()
```

### ❌ DON'T

```lua
-- Don't use MD5/SHA1 for security (legacy only)
local hash = crypto.hash("md5", data)  -- ⚠️ Weak

-- Don't use low bcrypt cost in production
local hash = crypto.bcrypt(password, 4)  -- ⚠️ Too fast

-- Don't use regular equality for secrets
if token == stored_token then  -- ⚠️ Timing attack
```

### Common Patterns

**API Key Generation:**
```lua
local bytes = crypto.randomBytes(32)
local api_key = crypto.hexEncode(table.concat(bytes))
```

**Password Storage:**
```lua
-- Registration
local hash = crypto.bcrypt(user_password, 12)
-- Store hash in database

-- Login
local is_valid = crypto.bcryptVerify(attempt, stored_hash)
```

**Message Signing:**
```lua
local signature = crypto.hmac("sha256", api_secret, request_body)
-- Send signature with request
-- Verify on server using timing-safe comparison
```

**Data Integrity:**
```lua
local checksum = crypto.hash("sha256", file_data)
-- Store checksum
-- Later verify: hash(data) == stored_checksum
```

---

## Error Handling

```lua
-- Invalid algorithm
local ok, err = pcall(function()
    return crypto.hash("invalid", "data")
end)
if not ok then
    print("Error:", err)  -- Invalid algorithm
end

-- Invalid cost
local ok, err = pcall(function()
    return crypto.bcrypt("pass", 50)  -- > 31
end)
if not ok then
    print("Error:", err)  -- Cost must be 4-31
end

-- Invalid size
local ok, err = pcall(function()
    return crypto.randomBytes(0)  -- Must be 1+
end)
```

---

## Performance Notes

- **Hash functions**: Very fast (microseconds)
- **HMAC**: Fast (microseconds)
- **Bcrypt**: Intentionally slow (10-500ms depending on cost)
  - Cost 10: ~10ms
  - Cost 12: ~50ms
  - Cost 14: ~200ms
- **Random generation**: Fast, uses OS entropy
- **Encoding**: Very fast (microseconds)

---

## See Also

- [Security Tests](../../CRYPTO_SECURITY_TESTS.md) - Comprehensive security validation
- [Examples](../../examples/crypto-demo.lua) - More examples
- [Tests](../../tests/crypto_module_test.rs) - Test suite

---

**Module**: crypto  
**Functions**: 13  
**Status**: ✅ Production Ready  
**Last Updated**: October 27, 2025
