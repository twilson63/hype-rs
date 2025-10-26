local http = require('http')

print("Testing HTTP module Lua bindings...")

print("\n=== Test 1: Module loaded successfully ===")
assert(http ~= nil, "HTTP module should be loaded")
assert(type(http) == "table", "HTTP module should be a table")

print("\n=== Test 2: All HTTP methods exist ===")
assert(type(http.get) == "function", "http.get should be a function")
assert(type(http.post) == "function", "http.post should be a function")
assert(type(http.put) == "function", "http.put should be a function")
assert(type(http.delete) == "function", "http.delete should be a function")
assert(type(http.patch) == "function", "http.patch should be a function")
assert(type(http.head) == "function", "http.head should be a function")
assert(type(http.fetch) == "function", "http.fetch should be a function")

print("\n=== Test 3: Convenience methods exist ===")
assert(type(http.postJson) == "function", "http.postJson should be a function")
assert(type(http.putJson) == "function", "http.putJson should be a function")

print("\n=== Test 4: Basic GET request (httpbin.org) ===")
local response = http.get("https://httpbin.org/get")
assert(response ~= nil, "Response should not be nil")
assert(type(response) == "table", "Response should be a table")

print("\n=== Test 5: Response object structure ===")
assert(type(response.status) == "number", "Response should have numeric status")
assert(type(response.statusText) == "string", "Response should have statusText")
assert(type(response.headers) == "table", "Response should have headers table")
assert(type(response.body) == "string", "Response should have body string")
print("Status: " .. response.status .. " " .. response.statusText)

print("\n=== Test 6: Response methods exist ===")
assert(type(response.json) == "function", "Response should have json() method")
assert(type(response.text) == "function", "Response should have text() method")
assert(type(response.ok) == "function", "Response should have ok() method")

print("\n=== Test 7: Response.ok() method ===")
local is_ok = response:ok()
assert(type(is_ok) == "boolean", "ok() should return boolean")
assert(is_ok == true, "200 response should be ok")
print("Response is OK: " .. tostring(is_ok))

print("\n=== Test 8: Response.text() method ===")
local text = response:text()
assert(type(text) == "string", "text() should return string")
assert(#text > 0, "text() should return non-empty string")
print("Body length: " .. #text)

print("\n=== Test 9: Response.json() method ===")
local json_data = response:json()
assert(type(json_data) == "table", "json() should return table")
assert(json_data.url ~= nil, "JSON should have 'url' field")
print("URL from JSON: " .. json_data.url)

print("\n=== Test 10: GET with query parameters ===")
local response2 = http.get("https://httpbin.org/get?foo=bar&baz=qux")
assert(response2:ok(), "GET with query params should succeed")
local data = response2:json()
assert(data.args ~= nil, "Should have args in response")
assert(data.args.foo == "bar", "Query param foo should be 'bar'")
assert(data.args.baz == "qux", "Query param baz should be 'qux'")

print("\n=== Test 11: fetch() with method option ===")
local response3 = http.fetch("https://httpbin.org/get", { method = "GET" })
assert(response3:ok(), "fetch with GET method should succeed")

print("\n=== Test 12: fetch() with headers ===")
local response4 = http.fetch("https://httpbin.org/headers", {
    method = "GET",
    headers = {
        ["User-Agent"] = "hype-rs-test",
        ["X-Custom-Header"] = "test-value"
    }
})
assert(response4:ok(), "fetch with custom headers should succeed")
local headers_data = response4:json()
assert(headers_data.headers["User-Agent"] == "hype-rs-test", "Custom User-Agent should be set")
assert(headers_data.headers["X-Custom-Header"] == "test-value", "Custom header should be set")

print("\n=== Test 13: POST with JSON body ===")
local post_response = http.postJson("https://httpbin.org/post", {
    name = "test",
    value = 42,
    nested = { key = "value" }
})
assert(post_response:ok(), "POST JSON should succeed")
local post_data = post_response:json()
assert(post_data.json ~= nil, "Should have json field in response")
assert(post_data.json.name == "test", "Posted name should match")
assert(post_data.json.value == 42, "Posted value should match")
assert(post_data.json.nested.key == "value", "Nested value should match")

print("\n=== Test 14: PUT with JSON body ===")
local put_response = http.putJson("https://httpbin.org/put", {
    updated = true,
    count = 10
})
assert(put_response:ok(), "PUT JSON should succeed")
local put_data = put_response:json()
assert(put_data.json.updated == true, "Updated field should be true")
assert(put_data.json.count == 10, "Count field should be 10")

print("\n=== Test 15: POST with string body ===")
local post_str_response = http.post("https://httpbin.org/post", {
    body = "plain text body",
    headers = {
        ["Content-Type"] = "text/plain"
    }
})
assert(post_str_response:ok(), "POST with string body should succeed")

print("\n=== Test 16: DELETE request ===")
local delete_response = http.delete("https://httpbin.org/delete")
assert(delete_response:ok(), "DELETE should succeed")

print("\n=== Test 17: PATCH request ===")
local patch_response = http.patch("https://httpbin.org/patch", {
    body = '{"patched": true}',
    headers = {
        ["Content-Type"] = "application/json"
    }
})
assert(patch_response:ok(), "PATCH should succeed")

print("\n=== Test 18: HEAD request ===")
local head_response = http.head("https://httpbin.org/get")
assert(head_response:ok(), "HEAD should succeed")
assert(head_response.body == "", "HEAD response body should be empty")

print("\n=== Test 19: Error handling - invalid URL ===")
local success, err = pcall(function()
    http.get("not-a-valid-url")
end)
assert(not success, "Invalid URL should throw error")
assert(type(err) == "string", "Error should be a string")
print("Error caught: " .. tostring(err))

print("\n=== Test 20: Error handling - 404 response ===")
local not_found_response = http.get("https://httpbin.org/status/404")
assert(not_found_response ~= nil, "Should still get response object for 404")
assert(not_found_response.status == 404, "Status should be 404")
assert(not not_found_response:ok(), "404 should not be ok()")

print("\n=== Test 21: Timeout option ===")
local timeout_response = http.fetch("https://httpbin.org/delay/1", {
    timeout = 5000
})
assert(timeout_response:ok(), "Request with timeout should succeed")

print("\n=== All tests passed! ===")
