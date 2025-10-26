local http = require('http')

print("=== HTTP Module Demo ===\n")

print("1. Simple GET request:")
local response = http.get("https://httpbin.org/get?demo=true")
print("   Status: " .. response.status .. " " .. response.statusText)
print("   Success: " .. tostring(response:ok()))
print("   Body length: " .. #response.body .. " bytes\n")

print("2. Parse JSON response:")
local data = response:json()
print("   URL: " .. data.url)
print("   Query param 'demo': " .. data.args.demo .. "\n")

print("3. POST JSON data:")
local post_response = http.postJson("https://httpbin.org/post", {
    username = "demo-user",
    timestamp = 1234567890,
    settings = {
        theme = "dark",
        notifications = true
    }
})
print("   Status: " .. post_response.status)
local post_data = post_response:json()
print("   Posted username: " .. post_data.json.username)
print("   Posted theme: " .. post_data.json.settings.theme .. "\n")

print("4. Custom headers with fetch():")
local custom_response = http.fetch("https://httpbin.org/headers", {
    method = "GET",
    headers = {
        ["User-Agent"] = "hype-rs/1.0",
        ["X-Custom-Header"] = "demo-value"
    }
})
local headers_data = custom_response:json()
print("   User-Agent sent: " .. headers_data.headers["User-Agent"])
print("   Custom header sent: " .. headers_data.headers["X-Custom-Header"] .. "\n")

print("5. PUT request:")
local put_response = http.putJson("https://httpbin.org/put", {
    id = 123,
    updated = true
})
print("   Status: " .. put_response.status .. "\n")

print("6. DELETE request:")
local delete_response = http.delete("https://httpbin.org/delete")
print("   Status: " .. delete_response.status .. "\n")

print("7. Error handling:")
local success, err = pcall(function()
    http.get("not-a-valid-url")
end)
if not success then
    print("   Caught error: Invalid URL (expected)\n")
end

print("8. 404 handling:")
local not_found = http.get("https://httpbin.org/status/404")
print("   Status: " .. not_found.status)
print("   Is OK: " .. tostring(not_found:ok()) .. " (expected: false)\n")

print("✓ Demo complete!")
