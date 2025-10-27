local crypto = require("crypto")

print("=== Crypto Security Tests ===\n")

print("1. Hash Collision Resistance:")
local inputs = {"test", "Test", "test ", " test", "tset"}
local hashes = {}
for _, input in ipairs(inputs) do
    local hash = crypto.hash("sha256", input)
    for _, existing in ipairs(hashes) do
        assert(hash ~= existing, "Hash collision detected!")
    end
    table.insert(hashes, hash)
end
print("   ✓ No collisions in similar inputs")

print("\n2. HMAC Key Sensitivity:")
local message = "important data"
local hmac1 = crypto.hmac("sha256", "key1", message)
local hmac2 = crypto.hmac("sha256", "key2", message)
local hmac3 = crypto.hmac("sha256", "Key1", message)
assert(hmac1 ~= hmac2, "Different keys should produce different HMACs")
assert(hmac1 ~= hmac3, "Case-sensitive keys should produce different HMACs")
print("   ✓ HMAC is key-sensitive")

print("\n3. Bcrypt Salt Uniqueness:")
local password = "samepassword"
local hash1 = crypto.bcrypt(password, 4)
local hash2 = crypto.bcrypt(password, 4)
assert(hash1 ~= hash2, "Same password should have different salts")
assert(crypto.bcryptVerify(password, hash1), "Should verify with hash1")
assert(crypto.bcryptVerify(password, hash2), "Should verify with hash2")
print("   ✓ Bcrypt generates unique salts")

print("\n4. Bcrypt Case Sensitivity:")
local pass1 = "Password123"
local pass2 = "password123"
local hash = crypto.bcrypt(pass1, 4)
assert(crypto.bcryptVerify(pass1, hash), "Should verify correct case")
assert(not crypto.bcryptVerify(pass2, hash), "Should reject wrong case")
print("   ✓ Bcrypt is case-sensitive")

print("\n5. Random UUID Uniqueness:")
local uuids = {}
for i = 1, 100 do
    local uuid = crypto.randomUUID()
    for _, existing in ipairs(uuids) do
        assert(uuid ~= existing, "UUID collision at iteration " .. i)
    end
    table.insert(uuids, uuid)
end
print("   ✓ 100 unique UUIDs generated")

print("\n6. Random Bytes Non-Repetitive:")
local bytes1 = crypto.randomBytes(32)
local bytes2 = crypto.randomBytes(32)
local same_count = 0
for i = 1, 32 do
    if bytes1[i] == bytes2[i] then
        same_count = same_count + 1
    end
end
assert(same_count < 32, "Random bytes should not be identical")
print("   ✓ Random bytes are unique (" .. same_count .. "/32 matches)")

print("\n7. Timing Safe Equal - Length Independence:")
assert(crypto.timingSafeEqual("abc", "abc") == true, "Equal strings should match")
assert(crypto.timingSafeEqual("abc", "def") == false, "Different strings should not match")
assert(crypto.timingSafeEqual("short", "verylongstring") == false, "Different lengths should not match")
print("   ✓ Timing-safe comparison works correctly")

print("\n8. Base64 Reversibility:")
local test_data = {
    "Hello",
    "Hello World",
    "Special chars: !@#$%^&*()",
    "Unicode: 世界 🌍",
    "Numbers: 123456789",
    ""
}
for _, data in ipairs(test_data) do
    local encoded = crypto.base64Encode(data)
    local decoded = crypto.base64Decode(encoded)
    assert(decoded == data, "Base64 roundtrip failed for: " .. data)
end
print("   ✓ Base64 encoding is reversible")

print("\n9. Hex Encoding Reversibility:")
for _, data in ipairs(test_data) do
    local encoded = crypto.hexEncode(data)
    local decoded = crypto.hexDecode(encoded)
    assert(decoded == data, "Hex roundtrip failed for: " .. data)
end
print("   ✓ Hex encoding is reversible")

print("\n10. Hash Determinism:")
local data = "deterministic test"
local hash1 = crypto.hash("sha256", data)
local hash2 = crypto.hash("sha256", data)
local hash3 = crypto.hash("sha256", data)
assert(hash1 == hash2 and hash2 == hash3, "Hash should be deterministic")
print("   ✓ Hash function is deterministic")

print("\n11. HMAC Determinism:")
local key = "secret"
local msg = "message"
local hmac1 = crypto.hmac("sha256", key, msg)
local hmac2 = crypto.hmac("sha256", key, msg)
assert(hmac1 == hmac2, "HMAC should be deterministic")
print("   ✓ HMAC is deterministic")

print("\n12. Empty Input Handling:")
local empty_hash = crypto.hash("sha256", "")
assert(type(empty_hash) == "string" and #empty_hash == 64, "Should hash empty string")

local empty_b64 = crypto.base64Encode("")
assert(empty_b64 == "", "Empty base64 encode")

local empty_hex = crypto.hexEncode("")
assert(empty_hex == "", "Empty hex encode")
print("   ✓ Empty inputs handled correctly")

print("\n13. Special Characters in Passwords:")
local special_pass = "P@ssw0rd!#$%^&*()_+-=[]{}|;:,.<>?/"
local special_hash = crypto.bcrypt(special_pass, 4)
assert(crypto.bcryptVerify(special_pass, special_hash), "Should handle special chars")
assert(not crypto.bcryptVerify("Password", special_hash), "Should not match different password")
print("   ✓ Special characters in passwords work")

print("\n14. Unicode Password Handling:")
local unicode_pass = "密码🔐пароль"
local unicode_hash = crypto.bcrypt(unicode_pass, 4)
assert(crypto.bcryptVerify(unicode_pass, unicode_hash), "Should handle unicode")
print("   ✓ Unicode passwords work")

print("\n15. Random Int Range Verification:")
local min, max = 10, 20
local out_of_range = false
for i = 1, 100 do
    local num = crypto.randomInt(min, max)
    if num < min or num >= max then
        out_of_range = true
        break
    end
end
assert(not out_of_range, "Random int out of range")
print("   ✓ Random ints stay within range")

print("\n16. Multiple Hash Algorithms:")
local data_to_hash = "compare algorithms"
local sha256 = crypto.hash("sha256", data_to_hash)
local sha512 = crypto.hash("sha512", data_to_hash)
local md5 = crypto.hash("md5", data_to_hash)
local sha1 = crypto.hash("sha1", data_to_hash)

assert(#sha256 == 64, "SHA256 should be 64 hex chars")
assert(#sha512 == 128, "SHA512 should be 128 hex chars")
assert(#md5 == 32, "MD5 should be 32 hex chars")
assert(#sha1 == 40, "SHA1 should be 40 hex chars")

assert(sha256 ~= sha512, "Different algorithms should produce different hashes")
assert(sha256 ~= md5, "Different algorithms should produce different hashes")
print("   ✓ All hash algorithms produce correct lengths")

print("\n17. Bcrypt Cost Parameter:")
local pass = "testpass"
local hash_cost4 = crypto.bcrypt(pass, 4)
local hash_cost6 = crypto.bcrypt(pass, 6)
assert(hash_cost4 ~= hash_cost6, "Different costs should produce different hashes")
assert(string.match(hash_cost4, "^%$2[aby]%$04%$"), "Cost 4 should be in hash")
assert(string.match(hash_cost6, "^%$2[aby]%$06%$"), "Cost 6 should be in hash")
print("   ✓ Bcrypt cost parameter works")

print("\n18. Hash Avalanche Effect:")
local text1 = "password"
local text2 = "Password"
local hash_1 = crypto.hash("sha256", text1)
local hash_2 = crypto.hash("sha256", text2)
local different_chars = 0
for i = 1, #hash_1 do
    if hash_1:sub(i,i) ~= hash_2:sub(i,i) then
        different_chars = different_chars + 1
    end
end
local percent_diff = (different_chars / #hash_1) * 100
assert(percent_diff > 40, "Avalanche effect: at least 40% different")
print("   ✓ Avalanche effect: " .. string.format("%.1f", percent_diff) .. "% different")

print("\n✅ All security tests passed!")
