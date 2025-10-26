print("Testing HTTP GET request...")

local http = require('http')

print("\nMaking GET request to httpbin.org...")
local response = http.get("https://httpbin.org/get")

print("Response received!")
print("Status: " .. response.status .. " " .. response.statusText)
print("Body length: " .. #response.body)

print("\nTesting response methods...")
local is_ok = response:ok()
print("Response ok(): " .. tostring(is_ok))

local text = response:text()
print("Response text() length: " .. #text)

local json = response:json()
print("Response json() url field: " .. json.url)

print("\n✓ All tests passed!")
