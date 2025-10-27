local json = require("json")

print("=== JSON Module Demo ===\n")

print("1. Simple object encoding:")
local user = {
    name = "Alice",
    age = 30,
    email = "alice@example.com",
    active = true
}
local user_json = json.encode(user)
print("   " .. user_json)

print("\n2. Pretty printing:")
local config = {
    database = {
        host = "localhost",
        port = 5432,
        name = "myapp"
    },
    cache = {
        enabled = true,
        ttl = 3600
    }
}
print(json.encode(config, true))

print("\n3. Array encoding:")
local numbers = {1, 2, 3, 4, 5}
print("   Numbers: " .. json.encode(numbers))

local fruits = {"apple", "banana", "cherry"}
print("   Fruits: " .. json.encode(fruits))

print("\n4. Parsing JSON:")
local json_data = '{"status":"success","data":{"id":123,"score":95.5}}'
local parsed = json.parse(json_data)
print("   Status: " .. parsed.status)
print("   ID: " .. parsed.data.id)
print("   Score: " .. parsed.data.score)

print("\n5. Complex nested structure:")
local api_response = {
    meta = {
        version = "1.0",
        timestamp = 1234567890
    },
    users = {
        {id = 1, name = "Alice", roles = {"admin", "user"}},
        {id = 2, name = "Bob", roles = {"user"}},
        {id = 3, name = "Carol", roles = {"moderator", "user"}}
    },
    total = 3
}

local json_response = json.stringify(api_response, true)
print(json_response)

print("\n6. Roundtrip conversion:")
local original = {x = 10, y = 20, label = "Point A"}
local encoded = json.encode(original)
local decoded = json.decode(encoded)
print("   Original x: " .. original.x)
print("   Decoded x: " .. decoded.x)
print("   Match: " .. tostring(original.x == decoded.x))

print("\n7. Unicode support:")
local multilingual = {
    en = "Hello World",
    zh = "你好世界",
    ja = "こんにちは世界",
    emoji = "🌍 🚀 ⭐"
}
local encoded_ml = json.encode(multilingual, true)
print(encoded_ml)

print("\n=== Demo Complete ===")
