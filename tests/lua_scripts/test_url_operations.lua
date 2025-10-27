local url = require("url")

print("=== URL Module Integration Tests ===\n")

print("1. Testing url.parse()...")
local parsed = url.parse("https://user:pass@example.com:8080/path/to/page?query=1&foo=bar#section")
assert(parsed.protocol == "https", "protocol should be https")
assert(parsed.username == "user", "username should be user")
assert(parsed.password == "pass", "password should be pass")
assert(parsed.hostname == "example.com", "hostname should be example.com")
assert(parsed.port == 8080, "port should be 8080")
assert(parsed.path == "/path/to/page", "path mismatch")
assert(parsed.query == "query=1&foo=bar", "query mismatch")
assert(parsed.hash == "section", "hash should be section")
print("   ✓ Full URL parsing works")

print("\n2. Testing url.format()...")
local formatted = url.format({
    protocol = "https",
    hostname = "api.example.com",
    port = 443,
    path = "/v1/users",
    query = "active=true",
    hash = "top"
})
assert(string.find(formatted, "https://"), "Should contain https://")
assert(string.find(formatted, "api.example.com"), "Should contain hostname")
assert(string.find(formatted, "/v1/users"), "Should contain path")
print("   ✓ Formatted URL: " .. formatted)

print("\n3. Testing url.resolve()...")
local resolved1 = url.resolve("https://example.com/foo/bar", "../baz")
assert(resolved1 == "https://example.com/baz", "Relative path resolution failed")
print("   ✓ Relative: ../baz → " .. resolved1)

local resolved2 = url.resolve("https://example.com/foo", "/bar")
assert(resolved2 == "https://example.com/bar", "Absolute path resolution failed")
print("   ✓ Absolute: /bar → " .. resolved2)

local resolved3 = url.resolve("https://example.com/", "page.html")
assert(resolved3 == "https://example.com/page.html", "Simple resolution failed")
print("   ✓ Simple: page.html → " .. resolved3)

print("\n4. Testing url.encodeComponent()...")
local encoded1 = url.encodeComponent("hello world")
assert(encoded1 == "hello%20world", "Space encoding failed")
print("   ✓ 'hello world' → '" .. encoded1 .. "'")

local encoded2 = url.encodeComponent("foo@bar.com")
assert(encoded2 == "foo%40bar.com", "@ encoding failed")
print("   ✓ 'foo@bar.com' → '" .. encoded2 .. "'")

local encoded3 = url.encodeComponent("a&b=c")
print("   ✓ 'a&b=c' → '" .. encoded3 .. "'")

print("\n5. Testing url.decodeComponent()...")
local decoded1 = url.decodeComponent("hello%20world")
assert(decoded1 == "hello world", "Space decoding failed")
print("   ✓ 'hello%20world' → '" .. decoded1 .. "'")

local decoded2 = url.decodeComponent("foo%40bar.com")
assert(decoded2 == "foo@bar.com", "@ decoding failed")
print("   ✓ 'foo%40bar.com' → '" .. decoded2 .. "'")

print("\n6. Testing encode/decode roundtrip...")
local original = "Special chars: @#$%^&*()"
local encoded = url.encodeComponent(original)
local decoded = url.decodeComponent(encoded)
assert(decoded == original, "Roundtrip failed")
print("   ✓ Roundtrip successful for: " .. original)

print("\n7. Testing url.parseQuery()...")
local params = url.parseQuery("foo=bar&baz=qux&page=1&limit=10")
assert(params.foo == "bar", "foo param failed")
assert(params.baz == "qux", "baz param failed")
assert(params.page == "1", "page param failed")
assert(params.limit == "10", "limit param failed")
print("   ✓ Query parsing works")
print("     foo=" .. params.foo)
print("     baz=" .. params.baz)
print("     page=" .. params.page)
print("     limit=" .. params.limit)

print("\n8. Testing url.formatQuery()...")
local query_params = {
    search = "test",
    category = "books",
    sort = "price"
}
local query_string = url.formatQuery(query_params)
assert(type(query_string) == "string", "Query string should be string")
assert(string.find(query_string, "search=test"), "Should contain search param")
assert(string.find(query_string, "category=books"), "Should contain category param")
print("   ✓ Query string: " .. query_string)

print("\n9. Testing query with encoded values...")
local encoded_params = url.parseQuery("message=hello+world&email=user%40example.com")
assert(encoded_params.message == "hello world", "Space decoding failed")
assert(encoded_params.email == "user@example.com", "@ decoding failed")
print("   ✓ Encoded values decoded correctly")

print("\n10. Testing practical URL manipulation...")
local api_url = "https://api.example.com/v1/search?q=test"
local components = url.parse(api_url)
print("   Original: " .. api_url)
print("   Protocol: " .. components.protocol)
print("   Host: " .. components.hostname)
print("   Path: " .. components.path)

local query_data = url.parseQuery(components.query)
query_data.page = "2"
query_data.limit = "20"

local new_url = url.format({
    protocol = components.protocol,
    hostname = components.hostname,
    path = components.path,
    query = url.formatQuery(query_data)
})
print("   Updated: " .. new_url)
assert(string.find(new_url, "page=2"), "Should contain page param")
assert(string.find(new_url, "limit=20"), "Should contain limit param")
print("   ✓ URL manipulation successful")

print("\n11. Testing URL without port...")
local simple = url.parse("https://example.com/path")
assert(simple.protocol == "https", "Protocol should be https")
assert(simple.hostname == "example.com", "Hostname should be example.com")
assert(simple.port == nil, "Port should be nil")
print("   ✓ URL without explicit port works")

print("\n12. Testing URL with query but no hash...")
local no_hash = url.parse("https://example.com/search?q=test")
assert(no_hash.query == "q=test", "Query should be present")
assert(no_hash.hash == nil, "Hash should be nil")
print("   ✓ URL with query but no hash works")

print("\n13. Testing URL construction from scratch...")
local built = url.format({
    protocol = "http",
    hostname = "localhost",
    port = 3000,
    path = "/api/data"
})
print("   Built URL: " .. built)
assert(string.find(built, "http://localhost:3000/api/data"), "Built URL incorrect")
print("   ✓ URL construction works")

print("\n14. Testing path resolution edge cases...")
local edge1 = url.resolve("https://example.com/a/b/c", "../../d")
print("   ✓ Complex relative: " .. edge1)

local edge2 = url.resolve("https://example.com/", "./test")
print("   ✓ Current dir: " .. edge2)

print("\n=== All URL Module Tests Passed! ===")
