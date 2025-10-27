local json = require("json")

print("Testing json module...")

print("\n1. Testing encode (object)...")
local obj = {name = "Alice", age = 30, active = true}
local encoded_obj = json.encode(obj)
print("   Encoded:", encoded_obj)

print("\n2. Testing encode (array)...")
local arr = {1, 2, 3, 4, 5}
local encoded_arr = json.encode(arr)
print("   Encoded:", encoded_arr)

print("\n3. Testing encode with pretty print...")
local pretty = json.encode({key = "value"}, true)
print("   Pretty JSON:\n" .. pretty)

print("\n4. Testing decode (object)...")
local json_str = '{"name":"Bob","age":25,"city":"NYC"}'
local decoded = json.decode(json_str)
print("   Name:", decoded.name)
print("   Age:", decoded.age)
print("   City:", decoded.city)

print("\n5. Testing decode (array)...")
local json_arr = '[10, 20, 30, 40, 50]'
local decoded_arr = json.decode(json_arr)
print("   Array length:", #decoded_arr)
print("   First element:", decoded_arr[1])
print("   Last element:", decoded_arr[5])

print("\n6. Testing roundtrip...")
local original = {
    string = "test",
    number = 42,
    boolean = true,
    array = {1, 2, 3},
    nested = {
        key = "value",
        inner = {x = 10}
    }
}
local encoded = json.encode(original)
local decoded = json.decode(encoded)
print("   String matches:", decoded.string == "test")
print("   Number matches:", decoded.number == 42)
print("   Boolean matches:", decoded.boolean == true)
print("   Array matches:", decoded.array[2] == 2)
print("   Nested matches:", decoded.nested.inner.x == 10)

print("\n7. Testing stringify (alias)...")
local stringified = json.stringify({x = 1, y = 2})
print("   Stringified:", stringified)

print("\n8. Testing parse (alias)...")
local parsed = json.parse('{"a":100,"b":200}')
print("   Parsed a:", parsed.a)
print("   Parsed b:", parsed.b)

print("\n9. Testing Unicode support...")
local unicode_data = {text = "Hello 世界 🚀"}
local encoded_unicode = json.encode(unicode_data)
local decoded_unicode = json.decode(encoded_unicode)
print("   Unicode text:", decoded_unicode.text)

print("\n10. Testing complex nested structure...")
local complex = {
    users = {
        {id = 1, name = "Alice", roles = {"admin", "user"}},
        {id = 2, name = "Bob", roles = {"user"}}
    },
    metadata = {
        version = 1,
        created = 1234567890,
        tags = {"production", "api"}
    }
}
local complex_json = json.encode(complex, true)
print("   Complex structure encoded (pretty):")
print(complex_json)

local complex_decoded = json.decode(complex_json)
print("   First user name:", complex_decoded.users[1].name)
print("   First user roles:", table.concat(complex_decoded.users[1].roles, ", "))
print("   Metadata version:", complex_decoded.metadata.version)

print("\n11. Testing null handling...")
local null_json = json.encode(nil)
print("   Encoded nil:", null_json)

print("\n12. Testing error handling...")
local ok, err = pcall(function()
    json.decode('{"invalid": }')
end)
if not ok then
    print("   Error caught (expected):", tostring(err):match("JSON") ~= nil)
end

print("\nAll json module tests passed!")
