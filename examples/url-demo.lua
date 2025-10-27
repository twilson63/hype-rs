local url = require("url")

print("=== URL Module Demo ===\n")

print("1. Parsing URLs:")
local example = "https://user:pass@api.example.com:8080/v1/users?active=true&role=admin#results"
print("  URL: " .. example)
local parsed = url.parse(example)
print("  Protocol: " .. parsed.protocol)
print("  Username: " .. (parsed.username or "none"))
print("  Password: " .. (parsed.password or "none"))
print("  Hostname: " .. parsed.hostname)
print("  Port: " .. (parsed.port or "default"))
print("  Path: " .. parsed.path)
print("  Query: " .. (parsed.query or "none"))
print("  Hash: " .. (parsed.hash or "none"))

print("\n2. Building URLs:")
local components = {
    protocol = "https",
    hostname = "api.github.com",
    path = "/repos/owner/repo/issues",
    query = "state=open&labels=bug"
}
local built = url.format(components)
print("  Built URL: " .. built)

print("\n3. Resolving Relative URLs:")
local base = "https://example.com/docs/guide/intro.html"
print("  Base: " .. base)

local relative_urls = {
    "./advanced.html",
    "../api/reference.html",
    "/home",
    "https://other.com/page"
}

for _, rel in ipairs(relative_urls) do
    local resolved = url.resolve(base, rel)
    print("  " .. rel .. " → " .. resolved)
end

print("\n4. URL Encoding:")
local to_encode = {
    "hello world",
    "user@example.com",
    "price=$99.99",
    "search: Lua & programming"
}

for _, str in ipairs(to_encode) do
    local encoded = url.encodeComponent(str)
    print("  '" .. str .. "' → '" .. encoded .. "'")
end

print("\n5. URL Decoding:")
local to_decode = {
    "hello%20world",
    "user%40example.com",
    "price%3D%2499.99"
}

for _, str in ipairs(to_decode) do
    local decoded = url.decodeComponent(str)
    print("  '" .. str .. "' → '" .. decoded .. "'")
end

print("\n6. Query String Parsing:")
local query = "search=laptop&category=electronics&price_max=1000&sort=price&order=asc"
print("  Query: " .. query)
local params = url.parseQuery(query)
print("  Parsed parameters:")
for key, value in pairs(params) do
    print("    " .. key .. " = " .. value)
end

print("\n7. Query String Building:")
local search_params = {
    q = "Lua programming",
    lang = "en",
    limit = "10",
    offset = "0"
}
local query_string = url.formatQuery(search_params)
print("  Parameters:")
for k, v in pairs(search_params) do
    print("    " .. k .. " = " .. v)
end
print("  Query string: " .. query_string)

print("\n8. API URL Builder:")
local function build_api_url(endpoint, params)
    return url.format({
        protocol = "https",
        hostname = "api.example.com",
        path = "/v1/" .. endpoint,
        query = url.formatQuery(params)
    })
end

local users_url = build_api_url("users", {active = "true", role = "admin"})
print("  Users API: " .. users_url)

local posts_url = build_api_url("posts", {author = "123", published = "true"})
print("  Posts API: " .. posts_url)

print("\n9. URL Manipulation:")
local original_url = "https://shop.example.com/products?category=books&page=1"
print("  Original: " .. original_url)

local parts = url.parse(original_url)
local current_params = url.parseQuery(parts.query)

current_params.page = "2"
current_params.sort = "price"
current_params.order = "asc"

local updated_url = url.format({
    protocol = parts.protocol,
    hostname = parts.hostname,
    path = parts.path,
    query = url.formatQuery(current_params)
})
print("  Updated: " .. updated_url)

print("\n10. Working with Fragments:")
local doc_url = url.format({
    protocol = "https",
    hostname = "docs.example.com",
    path = "/guide/installation",
    hash = "prerequisites"
})
print("  Documentation URL: " .. doc_url)

print("\n11. URL Validation Pattern:")
local function is_valid_url(url_string)
    local success, result = pcall(url.parse, url_string)
    return success and result.protocol ~= nil
end

local test_urls = {
    "https://example.com",
    "not a url",
    "ftp://files.example.com/data.zip",
    "mailto:test@example.com"
}

print("  URL Validation:")
for _, test_url in ipairs(test_urls) do
    local valid = is_valid_url(test_url)
    print("    " .. test_url .. " → " .. (valid and "✓ Valid" or "✗ Invalid"))
end

print("\n12. Building Search URLs:")
local function create_search_url(query_text, filters)
    filters = filters or {}
    filters.q = query_text
    
    return url.format({
        protocol = "https",
        hostname = "search.example.com",
        path = "/results",
        query = url.formatQuery(filters)
    })
end

local search_url = create_search_url("Lua tutorials", {
    difficulty = "beginner",
    duration = "short",
    free = "true"
})
print("  Search URL: " .. search_url)

print("\n13. CDN URL Builder:")
local function cdn_url(resource, version)
    return url.format({
        protocol = "https",
        hostname = "cdn.example.com",
        path = "/" .. resource .. "/" .. version .. "/bundle.js"
    })
end

print("  CDN URLs:")
print("    React: " .. cdn_url("react", "18.2.0"))
print("    Vue: " .. cdn_url("vue", "3.3.4"))

print("\n=== Demo Complete ===")
