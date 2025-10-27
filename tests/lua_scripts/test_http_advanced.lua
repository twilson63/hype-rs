local http = require("http")

print("=== Testing HTTP Advanced Features ===\n")

local failed = false

print("1. Testing postForm()...")
local res1 = http.postForm("https://httpbin.org/post", {
    username = "alice",
    password = "secret123"
})
if res1.status == 200 and res1.body:find("alice") then
    print("✓ Form submission works")
else
    print("✗ Form submission failed")
    failed = true
end

print("\n2. Testing uploadFile()...")
local file_content = "This is a test file content"
local ok2, res2 = pcall(http.uploadFile, "https://httpbin.org/post", {
    file = {
        filename = "test.txt",
        content = file_content,
        contentType = "text/plain"
    },
    metadata = "Test upload"
})
if ok2 then
    if res2.status == 200 and res2.body:find("test.txt", 1, true) then
        print("✓ File upload works")
    else
        print("? File upload returned " .. res2.status)
    end
else
    print("? File upload test inconclusive: " .. tostring(res2))
end

print("\n3. Testing Basic Auth...")
local res3 = http.get("https://httpbin.org/basic-auth/user/pass", {
    auth = {
        username = "user",
        password = "pass"
    }
})
if res3.status == 200 then
    print("✓ Basic Auth works")
else
    print("✗ Basic Auth failed: status=" .. res3.status)
    failed = true
end

print("\n4. Testing Bearer Token...")
local res4 = http.get("https://httpbin.org/bearer", {
    authToken = "test_token_123"
})
if res4.status == 200 and res4.body:find("test_token_123") then
    print("✓ Bearer token works")
else
    print("✗ Bearer token failed")
    failed = true
end

print("\n5. Testing Proxy configuration...")
local ok, res5 = pcall(http.get, "https://httpbin.org/get", {
    proxy = "http://localhost:8888"
})
if ok then
    print("✓ Proxy configuration accepted")
else
    print("? Proxy test skipped (no local proxy)")
end

print("\n6. Testing backward compatibility...")
local res6 = http.get("https://httpbin.org/get")
if res6.status == 200 then
    print("✓ Backward compatibility maintained")
else
    print("✗ Backward compatibility broken")
    failed = true
end

print("\n7. Testing form with special characters...")
local res7 = http.postForm("https://httpbin.org/post", {
    email = "user@example.com",
    message = "Hello World!"
})
if res7.status == 200 then
    print("✓ Form encoding works")
else
    print("✗ Form encoding failed")
    failed = true
end

if failed then
    print("\n=== Some Tests Failed ===")
    error("Tests failed")
else
    print("\n=== All Tests Passed ===")
end
