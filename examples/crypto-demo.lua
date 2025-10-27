local crypto = require("crypto")

print("=== Crypto Module Demo ===\n")

print("1. Hashing:")
local data = "Hello, World!"
print("   Data: " .. data)
print("   SHA256: " .. crypto.hash("sha256", data))
print("   SHA512: " .. crypto.hash("sha512", data))
print("   MD5:    " .. crypto.hash("md5", data))
print("   SHA1:   " .. crypto.hash("sha1", data))

print("\n2. HMAC Signing:")
local secret = "my-secret-key"
local message = "important message"
print("   Secret: " .. secret)
print("   Message: " .. message)
print("   HMAC-SHA256: " .. crypto.hmac("sha256", secret, message))

print("\n3. Random Generation:")
print("   Random UUID: " .. crypto.randomUUID())
print("   Random UUID: " .. crypto.randomUUID())
print("   Random Int (1-100): " .. crypto.randomInt(1, 100))
print("   Random Int (1-1000): " .. crypto.randomInt(1, 1000))

local bytes = crypto.randomBytes(8)
local hex_bytes = {}
for i = 1, 8 do
    table.insert(hex_bytes, string.format("%02x", bytes[i]))
end
print("   Random Bytes (8): " .. table.concat(hex_bytes, " "))

print("\n4. Base64 Encoding:")
local text = "Hello, World! 🚀"
print("   Original: " .. text)
local b64 = crypto.base64Encode(text)
print("   Encoded:  " .. b64)
print("   Decoded:  " .. crypto.base64Decode(b64))

print("\n5. Hex Encoding:")
local hex_text = "Secret Data"
print("   Original: " .. hex_text)
local hex = crypto.hexEncode(hex_text)
print("   Encoded:  " .. hex)
print("   Decoded:  " .. crypto.hexDecode(hex))

print("\n6. Password Hashing (bcrypt):")
local password = "user_password_123"
print("   Password: " .. password)
local hash = crypto.bcrypt(password, 6)
print("   Hash:     " .. hash)
print("   Verify correct: " .. tostring(crypto.bcryptVerify(password, hash)))
print("   Verify wrong:   " .. tostring(crypto.bcryptVerify("wrong_password", hash)))

print("\n7. Timing-Safe Comparison:")
local token1 = "abc123xyz"
local token2 = "abc123xyz"
local token3 = "abc123abc"
print("   Token 1: " .. token1)
print("   Token 2: " .. token2)
print("   Token 3: " .. token3)
print("   token1 == token2: " .. tostring(crypto.timingSafeEqual(token1, token2)))
print("   token1 == token3: " .. tostring(crypto.timingSafeEqual(token1, token3)))

print("\n8. API Key Generation:")
local api_key_bytes = crypto.randomBytes(32)
local api_key_hex = {}
for i = 1, 32 do
    table.insert(api_key_hex, string.format("%02x", api_key_bytes[i]))
end
local api_key = table.concat(api_key_hex)
print("   Generated API Key (64 hex chars):")
print("   " .. api_key)

print("\n9. JWT-like Token (simplified):")
local header = '{"alg":"HS256","typ":"JWT"}'
local payload = '{"userId":123,"exp":1234567890}'
local header_b64 = crypto.base64Encode(header)
local payload_b64 = crypto.base64Encode(payload)
local unsigned_token = header_b64 .. "." .. payload_b64
local signature = crypto.hmac("sha256", "jwt-secret", unsigned_token)
local token = unsigned_token .. "." .. signature
print("   JWT Token (simplified):")
print("   " .. token:sub(1, 80) .. "...")

print("\n10. Data Integrity Check:")
local file_data = "This is important file content"
local checksum = crypto.hash("sha256", file_data)
print("   Data: " .. file_data)
print("   Checksum (SHA256): " .. checksum)
print("   Verification: " .. (crypto.hash("sha256", file_data) == checksum and "✓ Valid" or "✗ Invalid"))

print("\n11. Multiple Hash Algorithms Comparison:")
local input = "compare me"
print("   Input: " .. input)
print("   MD5    (32 chars):  " .. crypto.hash("md5", input))
print("   SHA1   (40 chars):  " .. crypto.hash("sha1", input))
print("   SHA256 (64 chars):  " .. crypto.hash("sha256", input))
print("   SHA512 (128 chars): " .. crypto.hash("sha512", input):sub(1, 64) .. "...")

print("\n12. Session ID Generation:")
for i = 1, 3 do
    local session_bytes = crypto.randomBytes(16)
    local session_hex = {}
    for j = 1, 16 do
        table.insert(session_hex, string.format("%02x", session_bytes[j]))
    end
    print("   Session " .. i .. ": " .. table.concat(session_hex))
end

print("\n=== Demo Complete ===")
