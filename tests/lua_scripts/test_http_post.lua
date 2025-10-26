print("Testing HTTP POST with JSON...")

local http = require('http')

print("\nTesting postJson()...")
local response = http.postJson("https://httpbin.org/post", {
    name = "test",
    value = 42,
    nested = { key = "value" }
})

print("Status: " .. response.status .. " " .. response.statusText)
print("Is OK: " .. tostring(response:ok()))

local data = response:json()
print("\nVerifying posted data:")
print("  name: " .. data.json.name)
print("  value: " .. tostring(data.json.value))
print("  nested.key: " .. data.json.nested.key)

assert(data.json.name == "test", "Name should match")
assert(data.json.value == 42, "Value should match")
assert(data.json.nested.key == "value", "Nested key should match")

print("\n✓ All tests passed!")
