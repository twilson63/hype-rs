print("Testing basic HTTP module loading...")

local http = require('http')

assert(http ~= nil, "HTTP module should be loaded")
assert(type(http) == "table", "HTTP module should be a table")

assert(type(http.get) == "function", "http.get should be a function")
assert(type(http.post) == "function", "http.post should be a function")
assert(type(http.fetch) == "function", "http.fetch should be a function")
assert(type(http.postJson) == "function", "http.postJson should be a function")
assert(type(http.putJson) == "function", "http.putJson should be a function")

print("✓ HTTP module loaded successfully")
print("✓ All expected functions are present")
print("\nModule structure:")
for key, value in pairs(http) do
    print("  " .. key .. ": " .. type(value))
end
