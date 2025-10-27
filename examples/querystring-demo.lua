local qs = require("querystring")

print("=== Query String Module Demo ===\n")

print("1. Parse query string:")
local query = "name=John+Doe&email=test%40example.com&age=30&city=New%20York"
print("   Input: " .. query)
local parsed = qs.parse(query)
print("   Parsed:")
for k, v in pairs(parsed) do
    print("     " .. k .. " = " .. v)
end

print("\n2. Stringify table to query string:")
local params = {
    product = "laptop",
    price = "999.99",
    currency = "USD",
    category = "electronics"
}
print("   Input table:")
for k, v in pairs(params) do
    print("     " .. k .. " = " .. v)
end
local stringified = qs.stringify(params)
print("   Query string: " .. stringified)

print("\n3. Escape special characters:")
local text = "Hello World! How are you? @example.com"
print("   Input: " .. text)
local escaped = qs.escape(text)
print("   Escaped: " .. escaped)

print("\n4. Unescape URL-encoded string:")
local encoded = "Hello+World%21+How+are+you%3F+%40example.com"
print("   Input: " .. encoded)
local unescaped = qs.unescape(encoded)
print("   Unescaped: " .. unescaped)

print("\n5. Build a search URL:")
local search_params = {
    q = "lua programming",
    page = "1",
    sort = "relevance",
    filter = "recent"
}
local search_query = qs.stringify(search_params)
local search_url = "https://example.com/search?" .. search_query
print("   Search URL: " .. search_url)

print("\n6. Parse URL from web request:")
local web_query = "user=admin&token=abc123&action=login&remember=true"
print("   Query: " .. web_query)
local web_params = qs.parse(web_query)
print("   Parsed parameters:")
print("     User: " .. (web_params.user or "N/A"))
print("     Token: " .. (web_params.token or "N/A"))
print("     Action: " .. (web_params.action or "N/A"))
print("     Remember: " .. (web_params.remember or "N/A"))

print("\n7. Handle complex characters:")
local complex = {
    text = "Hello & goodbye!",
    emoji = "🚀",
    math = "a + b = c",
    symbols = "@#$%^&*()"
}
print("   Input:")
for k, v in pairs(complex) do
    print("     " .. k .. " = " .. v)
end
local complex_query = qs.stringify(complex)
print("   Encoded: " .. complex_query)
local decoded = qs.parse(complex_query)
print("   Decoded back:")
for k, v in pairs(decoded) do
    print("     " .. k .. " = " .. v)
end

print("\n8. API parameter building:")
local api_params = {
    api_key = "sk-1234567890abcdef",
    endpoint = "/api/v1/users",
    limit = "100",
    offset = "0",
    fields = "id,name,email"
}
local api_query = qs.stringify(api_params)
print("   API Query String: " .. api_query)

print("\n=== Demo Complete ===")
