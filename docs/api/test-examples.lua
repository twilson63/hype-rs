-- Test all API documentation examples

print("=== Testing API Documentation Examples ===\n")

local tests_passed = 0
local tests_failed = 0

local function test(name, fn)
    local ok, err = pcall(fn)
    if ok then
        print("✅ " .. name)
        tests_passed = tests_passed + 1
    else
        print("❌ " .. name)
        print("   Error: " .. tostring(err))
        tests_failed = tests_failed + 1
    end
end

-- ============================================================================
-- CRYPTO MODULE TESTS
-- ============================================================================

print("\n📚 Testing crypto module examples...\n")

test("crypto.hash - SHA256", function()
    local crypto = require("crypto")
    local hash = crypto.hash("sha256", "hello")
    assert(hash == "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")
end)

test("crypto.hash - Multiple algorithms", function()
    local crypto = require("crypto")
    local sha512 = crypto.hash("sha512", "hello")
    local sha1 = crypto.hash("sha1", "hello")
    local md5 = crypto.hash("md5", "hello")
    assert(#sha512 == 128)
    assert(#sha1 == 40)
    assert(#md5 == 32)
end)

-- test("crypto.hashFile", function()
--     local crypto = require("crypto")
--     local fs = require("fs")
--     fs.write("test_doc.txt", "file contents")
--     local hash = crypto.hashFile("sha256", "test_doc.txt")
--     assert(type(hash) == "string")
--     assert(#hash == 64)
--     fs.remove("test_doc.txt")
-- end)

test("crypto.hmac", function()
    local crypto = require("crypto")
    local key = "my-secret-key"
    local message = "important data"
    local signature = crypto.hmac("sha256", key, message)
    local received_sig = crypto.hmac("sha256", key, message)
    local is_valid = crypto.timingSafeEqual(signature, received_sig)
    assert(is_valid == true)
end)

test("crypto.randomBytes", function()
    local crypto = require("crypto")
    local bytes = crypto.randomBytes(16)
    local hex = {}
    for i = 1, 16 do
        table.insert(hex, string.format("%02x", bytes[i]))
    end
    local hex_string = table.concat(hex)
    assert(#hex_string == 32)
end)

test("crypto.randomInt", function()
    local crypto = require("crypto")
    local num = crypto.randomInt(1, 101)
    assert(num >= 1 and num < 101)
    
    local roll = crypto.randomInt(1, 7)
    assert(roll >= 1 and roll < 7)
    
    local temp = crypto.randomInt(-10, 11)
    assert(temp >= -10 and temp < 11)
end)

test("crypto.randomUUID", function()
    local crypto = require("crypto")
    local id1 = crypto.randomUUID()
    local id2 = crypto.randomUUID()
    assert(#id1 == 36)
    assert(#id2 == 36)
    assert(id1 ~= id2)
end)

test("crypto.base64Encode", function()
    local crypto = require("crypto")
    local encoded = crypto.base64Encode("Hello, World!")
    assert(encoded == "SGVsbG8sIFdvcmxkIQ==")
    
    local unicode = crypto.base64Encode("Hello 世界 🌍")
    assert(type(unicode) == "string")
end)

test("crypto.base64Decode", function()
    local crypto = require("crypto")
    local decoded = crypto.base64Decode("SGVsbG8sIFdvcmxkIQ==")
    assert(decoded == "Hello, World!")
    
    local original = "Test data 123"
    local encoded = crypto.base64Encode(original)
    local result = crypto.base64Decode(encoded)
    assert(result == original)
end)

test("crypto.hexEncode", function()
    local crypto = require("crypto")
    local hex = crypto.hexEncode("hello")
    assert(hex == "68656c6c6f")
end)

test("crypto.hexDecode", function()
    local crypto = require("crypto")
    local decoded = crypto.hexDecode("68656c6c6f")
    assert(decoded == "hello")
    
    local data = "Secret123"
    local hex = crypto.hexEncode(data)
    local result = crypto.hexDecode(hex)
    assert(result == data)
end)

test("crypto.bcrypt", function()
    local crypto = require("crypto")
    local hash = crypto.bcrypt("user_password_123", 4)
    assert(type(hash) == "string")
    assert(#hash > 50)
    
    local hash1 = crypto.bcrypt("same_password", 4)
    local hash2 = crypto.bcrypt("same_password", 4)
    assert(hash1 ~= hash2)
end)

test("crypto.bcryptVerify", function()
    local crypto = require("crypto")
    local hash = crypto.bcrypt("correct_password", 4)
    local is_correct = crypto.bcryptVerify("correct_password", hash)
    assert(is_correct == true)
    
    local is_wrong = crypto.bcryptVerify("wrong_password", hash)
    assert(is_wrong == false)
end)

test("crypto.timingSafeEqual", function()
    local crypto = require("crypto")
    local token1 = "secret_token_abc123"
    local token2 = "secret_token_abc123"
    local is_valid = crypto.timingSafeEqual(token1, token2)
    assert(is_valid == true)
    
    local not_equal = crypto.timingSafeEqual(token1, "different")
    assert(not_equal == false)
end)

test("crypto - API key generation pattern", function()
    local crypto = require("crypto")
    local bytes = crypto.randomBytes(32)
    local hex = {}
    for i = 1, 32 do
        table.insert(hex, string.format("%02x", bytes[i]))
    end
    local api_key = table.concat(hex)
    assert(#api_key == 64)
end)

test("crypto - Message signing pattern", function()
    local crypto = require("crypto")
    local api_secret = "secret"
    local request_body = "data"
    local signature = crypto.hmac("sha256", api_secret, request_body)
    assert(type(signature) == "string")
    assert(#signature == 64)
end)

test("crypto - Data integrity pattern", function()
    local crypto = require("crypto")
    local file_data = "important file"
    local checksum = crypto.hash("sha256", file_data)
    local verify = crypto.hash("sha256", file_data)
    assert(checksum == verify)
end)

-- ============================================================================
-- SUMMARY
-- ============================================================================

print("\n" .. string.rep("=", 60))
print("RESULTS:")
print("  ✅ Passed: " .. tests_passed)
if tests_failed > 0 then
    print("  ❌ Failed: " .. tests_failed)
else
    print("  🎉 All documentation examples working!")
end
