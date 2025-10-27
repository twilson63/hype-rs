local crypto = require("crypto")

print("Testing crypto module...")

print("\n1. Testing hash SHA256:")
local sha256 = crypto.hash("sha256", "hello")
assert(sha256 == "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824", "SHA256 hash mismatch")
print("   ✓ SHA256 hash works")

print("\n2. Testing hash MD5:")
local md5 = crypto.hash("md5", "hello")
assert(md5 == "5d41402abc4b2a76b9719d911017c592", "MD5 hash mismatch")
print("   ✓ MD5 hash works")

print("\n3. Testing HMAC:")
local hmac = crypto.hmac("sha256", "secret", "hello")
assert(hmac == "88aab3ede8d3adf94d26ab90d3bafd4a2083070c3bcce9c014ee04a443847c0b", "HMAC mismatch")
print("   ✓ HMAC works")

print("\n4. Testing random bytes:")
local bytes = crypto.randomBytes(16)
assert(type(bytes) == "table", "Expected table")
local count = 0
for _ in pairs(bytes) do count = count + 1 end
assert(count == 16, "Expected 16 bytes")
print("   ✓ Random bytes works")

print("\n5. Testing random int:")
local num = crypto.randomInt(1, 100)
assert(type(num) == "number", "Expected number")
assert(num >= 1 and num < 100, "Number out of range")
print("   ✓ Random int works: " .. num)

print("\n6. Testing random UUID:")
local uuid = crypto.randomUUID()
assert(type(uuid) == "string", "Expected string")
assert(#uuid == 36, "UUID length mismatch")
print("   ✓ Random UUID works: " .. uuid)

print("\n7. Testing Base64 encode:")
local encoded = crypto.base64Encode("hello")
assert(encoded == "aGVsbG8=", "Base64 encode mismatch")
print("   ✓ Base64 encode works")

print("\n8. Testing Base64 decode:")
local decoded = crypto.base64Decode("aGVsbG8=")
assert(decoded == "hello", "Base64 decode mismatch")
print("   ✓ Base64 decode works")

print("\n9. Testing Base64 roundtrip:")
local original = "Hello, World! 123"
local base64_encoded = crypto.base64Encode(original)
local base64_decoded = crypto.base64Decode(base64_encoded)
assert(base64_decoded == original, "Roundtrip mismatch")
print("   ✓ Base64 roundtrip works")

print("\n10. Testing Hex encode:")
local hex_encoded = crypto.hexEncode("hello")
assert(hex_encoded == "68656c6c6f", "Hex encode mismatch")
print("   ✓ Hex encode works")

print("\n11. Testing Hex decode:")
local hex_decoded = crypto.hexDecode("68656c6c6f")
assert(hex_decoded == "hello", "Hex decode mismatch")
print("   ✓ Hex decode works")

print("\n12. Testing Hex roundtrip:")
local hex_original = "Test data 123!"
local hex_enc = crypto.hexEncode(hex_original)
local hex_dec = crypto.hexDecode(hex_enc)
assert(hex_dec == hex_original, "Hex roundtrip mismatch")
print("   ✓ Hex roundtrip works")

print("\n13. Testing bcrypt:")
local hash = crypto.bcrypt("password123", 4)
assert(type(hash) == "string", "Expected string")
assert(#hash > 50, "Hash too short")
print("   ✓ Bcrypt works")

print("\n14. Testing bcrypt verify:")
local password = "mypassword"
local bcrypt_hash = crypto.bcrypt(password, 4)
assert(crypto.bcryptVerify(password, bcrypt_hash) == true, "Verify failed for correct password")
assert(crypto.bcryptVerify("wrongpassword", bcrypt_hash) == false, "Verify succeeded for wrong password")
print("   ✓ Bcrypt verify works")

print("\n15. Testing timing safe equal:")
assert(crypto.timingSafeEqual("hello", "hello") == true, "Expected true for equal strings")
assert(crypto.timingSafeEqual("hello", "world") == false, "Expected false for different strings")
assert(crypto.timingSafeEqual("test", "testing") == false, "Expected false for different lengths")
print("   ✓ Timing safe equal works")

print("\n✅ All crypto tests passed!")
