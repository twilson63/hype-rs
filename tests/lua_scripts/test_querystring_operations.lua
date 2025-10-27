local qs = require("querystring")

print("Testing querystring module...")

print("\n1. Testing parse:")
local parsed = qs.parse("foo=bar&baz=qux")
assert(parsed.foo == "bar", "Expected foo=bar")
assert(parsed.baz == "qux", "Expected baz=qux")
print("   ✓ Basic parse works")

print("\n2. Testing parse with encoding:")
local parsed_encoded = qs.parse("name=John+Doe&email=test%40example.com")
assert(parsed_encoded.name == "John Doe", "Expected name='John Doe'")
assert(parsed_encoded.email == "test@example.com", "Expected email='test@example.com'")
print("   ✓ Parse with URL encoding works")

print("\n3. Testing stringify:")
local result = qs.stringify({foo = "bar", baz = "qux"})
assert(string.find(result, "foo=bar", 1, true) ~= nil, "Expected foo=bar in result")
assert(string.find(result, "baz=qux", 1, true) ~= nil, "Expected baz=qux in result")
print("   ✓ Basic stringify works")

print("\n4. Testing stringify with spaces:")
local result_spaces = qs.stringify({name = "John Doe"})
assert(string.find(result_spaces, "name=John+Doe", 1, true) ~= nil, "Expected name=John+Doe in result")
print("   ✓ Stringify with spaces works")

print("\n5. Testing escape:")
assert(qs.escape("hello world") == "hello+world", "Expected 'hello+world'")
assert(qs.escape("foo@bar.com") == "foo%40bar.com", "Expected 'foo%40bar.com'")
assert(qs.escape("a&b=c") == "a%26b%3Dc", "Expected 'a%26b%3Dc'")
print("   ✓ Escape works")

print("\n6. Testing unescape:")
assert(qs.unescape("hello+world") == "hello world", "Expected 'hello world'")
assert(qs.unescape("foo%40bar.com") == "foo@bar.com", "Expected 'foo@bar.com'")
assert(qs.unescape("a%26b%3Dc") == "a&b=c", "Expected 'a&b=c'")
print("   ✓ Unescape works")

print("\n7. Testing escape/unescape roundtrip:")
local original = "hello world & foo=bar @#$"
local escaped = qs.escape(original)
local unescaped = qs.unescape(escaped)
assert(unescaped == original, "Expected roundtrip to preserve original")
print("   ✓ Escape/unescape roundtrip works")

print("\n8. Testing parse with empty string:")
local empty_parsed = qs.parse("")
local count = 0
for _ in pairs(empty_parsed) do count = count + 1 end
assert(count == 0, "Expected empty table")
print("   ✓ Parse empty string works")

print("\n9. Testing stringify with empty table:")
local empty_result = qs.stringify({})
assert(empty_result == "", "Expected empty string")
print("   ✓ Stringify empty table works")

print("\n10. Testing parse/stringify roundtrip:")
local original_table = {name = "John Doe", email = "test@example.com", age = "30"}
local stringified = qs.stringify(original_table)
local parsed_back = qs.parse(stringified)
assert(parsed_back.name == original_table.name, "Expected name to match")
assert(parsed_back.email == original_table.email, "Expected email to match")
assert(parsed_back.age == original_table.age, "Expected age to match")
print("   ✓ Parse/stringify roundtrip works")

print("\n✅ All querystring tests passed!")
