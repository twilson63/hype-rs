# url - URL Parsing and Manipulation

> **RFC 3986 compliant URL parsing, building, and encoding utilities.**

## Table of Contents
- [Import](#import)
- [Parsing & Formatting](#parsing--formatting)
- [URL Resolution](#url-resolution)
- [Encoding & Decoding](#encoding--decoding)
- [Query Strings](#query-strings)
- [Examples](#examples)

---

## Import

```lua
local url = require("url")
```

---

## Parsing & Formatting

### url.parse(urlString)

Parse URL string into components.

**Parameters:**
- `urlString: string` - URL to parse

**Returns:** `table` - URL components:
  - `protocol: string` - Protocol (e.g., "http", "https")
  - `host: string` - Host with port (e.g., "example.com:8080")
  - `hostname: string` - Hostname only (e.g., "example.com")
  - `port: number|nil` - Port number (e.g., 8080) or nil for default
  - `path: string` - Path (e.g., "/api/users")
  - `query: string|nil` - Query string without "?" (e.g., "id=1&sort=asc")
  - `hash: string|nil` - Fragment without "#" (e.g., "section")
  - `username: string|nil` - Username for authentication
  - `password: string|nil` - Password for authentication

**Example:**
```lua
local url = require("url")

-- Parse full URL
local parsed = url.parse("https://user:pass@example.com:8080/api/users?id=1#top")

print(parsed.protocol)   -- "https"
print(parsed.hostname)   -- "example.com"
print(parsed.port)       -- 8080
print(parsed.path)       -- "/api/users"
print(parsed.query)      -- "id=1"
print(parsed.hash)       -- "top"
print(parsed.username)   -- "user"
print(parsed.password)   -- "pass"

-- Simple URL
local simple = url.parse("http://example.com/path")
print(simple.protocol)   -- "http"
print(simple.hostname)   -- "example.com"
print(simple.port)       -- nil (default: 80)
print(simple.path)       -- "/path"

-- HTTPS default port
local https = url.parse("https://example.com")
print(https.port)        -- nil (default: 443)
```

---

### url.format(components)

Build URL from components.

**Parameters:**
- `components: table` - URL components (same structure as parse result)

**Returns:** `string` - Formatted URL

**Example:**
```lua
local url = require("url")

-- Build URL
local components = {
    protocol = "https",
    hostname = "example.com",
    port = 8080,
    path = "/api/users",
    query = "id=1",
    hash = "top"
}
local result = url.format(components)
print(result)  -- "https://example.com:8080/api/users?id=1#top"

-- Minimal URL
local simple = url.format({
    protocol = "http",
    hostname = "localhost",
    path = "/"
})
print(simple)  -- "http://localhost/"

-- With authentication
local auth_url = url.format({
    protocol = "https",
    username = "user",
    password = "pass",
    hostname = "api.example.com",
    path = "/data"
})
print(auth_url)  -- "https://user:pass@api.example.com/data"
```

---

## URL Resolution

### url.resolve(base, relative)

Resolve relative URL against base URL.

**Parameters:**
- `base: string` - Base URL
- `relative: string` - Relative URL

**Returns:** `string` - Resolved absolute URL

**Example:**
```lua
local url = require("url")

local base = "https://example.com/api/v1/"

-- Relative paths
print(url.resolve(base, "users"))
-- "https://example.com/api/v1/users"

print(url.resolve(base, "./users"))
-- "https://example.com/api/v1/users"

print(url.resolve(base, "../v2/users"))
-- "https://example.com/api/v2/users"

-- Absolute path
print(url.resolve(base, "/other"))
-- "https://example.com/other"

-- Absolute URL (replaces base)
print(url.resolve(base, "https://other.com/path"))
-- "https://other.com/path"

-- Query and hash
print(url.resolve(base, "users?id=1#top"))
-- "https://example.com/api/v1/users?id=1#top"
```

---

## Encoding & Decoding

### url.encode(string)

URL encode string (form-urlencoded format).

**Parameters:**
- `string: string` - String to encode

**Returns:** `string` - Encoded string (spaces as `+`)

**Example:**
```lua
local url = require("url")

-- Space encoding
print(url.encode("hello world"))
-- "hello+world"

-- Special characters
print(url.encode("name=John Doe&age=30"))
-- "name%3DJohn+Doe%26age%3D30"

-- Unicode
print(url.encode("Hello 世界"))
-- "Hello+%E4%B8%96%E7%95%8C"

-- Use case: form data
local query = "search=" .. url.encode(user_input)
```

---

### url.decode(string)

URL decode string (form-urlencoded format).

**Parameters:**
- `string: string` - Encoded string

**Returns:** `string` - Decoded string

**Example:**
```lua
local url = require("url")

-- Decode spaces
print(url.decode("hello+world"))
-- "hello world"

-- Decode special characters
print(url.decode("name%3DJohn+Doe"))
-- "name=John Doe"

-- Roundtrip
local original = "hello world & friends"
local encoded = url.encode(original)
local decoded = url.decode(encoded)
assert(decoded == original)
```

---

### url.encodeComponent(string)

Encode URL component (percent-encoding).

**Parameters:**
- `string: string` - String to encode

**Returns:** `string` - Encoded string (spaces as `%20`)

**Example:**
```lua
local url = require("url")

-- Path encoding (spaces as %20)
print(url.encodeComponent("hello world"))
-- "hello%20world"

-- Reserved characters
print(url.encodeComponent("a/b?c=d&e#f"))
-- "a%2Fb%3Fc%3Dd%26e%23f"

-- Use case: build path
local user_id = "john/doe"
local path = "/users/" .. url.encodeComponent(user_id)
-- "/users/john%2Fdoe"

-- Query parameters (use for values)
local search = url.encodeComponent("hello world")
local query_url = "https://example.com/search?q=" .. search
-- "https://example.com/search?q=hello%20world"
```

---

### url.decodeComponent(string)

Decode URL component (percent-encoding).

**Parameters:**
- `string: string` - Encoded string

**Returns:** `string` - Decoded string

**Example:**
```lua
local url = require("url")

-- Decode percent-encoded
print(url.decodeComponent("hello%20world"))
-- "hello world"

print(url.decodeComponent("a%2Fb%3Fc%3Dd"))
-- "a/b?c=d"

-- Plus signs NOT decoded (use url.decode for that)
print(url.decodeComponent("hello+world"))
-- "hello+world" (not "hello world")
```

---

## Query Strings

### url.parseQuery(queryString)

Parse query string into table.

**Parameters:**
- `queryString: string` - Query string (without leading "?")

**Returns:** `table` - Key-value pairs

**Example:**
```lua
local url = require("url")

-- Parse query string
local params = url.parseQuery("name=John&age=30&city=NYC")
print(params.name)   -- "John"
print(params.age)    -- "30"
print(params.city)   -- "NYC"

-- From URL
local parsed = url.parse("https://example.com/search?q=lua&page=2")
local query_params = url.parseQuery(parsed.query)
print(query_params.q)     -- "lua"
print(query_params.page)  -- "2"

-- URL encoded values
local encoded = url.parseQuery("q=hello+world&name=John+Doe")
print(encoded.q)     -- "hello world" (automatically decoded)
print(encoded.name)  -- "John Doe"

-- Empty query
local empty = url.parseQuery("")
-- {} (empty table)
```

---

### url.formatQuery(params)

Format table as query string.

**Parameters:**
- `params: table` - Key-value pairs

**Returns:** `string` - Query string (without leading "?")

**Example:**
```lua
local url = require("url")

-- Build query string
local params = {
    name = "John Doe",
    age = "30",
    city = "New York"
}
local query = url.formatQuery(params)
print(query)
-- "age=30&city=New+York&name=John+Doe" (alphabetical)

-- Add to URL
local base = "https://example.com/api/users"
local full_url = base .. "?" .. query

-- Complex values
local search_params = {
    q = "hello world",
    filter = "type:user,status:active"
}
local search_query = url.formatQuery(search_params)
-- "filter=type%3Auser%2Cstatus%3Aactive&q=hello+world"

-- Roundtrip
local original = {name = "test", value = "123"}
local query_str = url.formatQuery(original)
local parsed_back = url.parseQuery(query_str)
-- parsed_back == original
```

---

## Examples

### Building API URLs

```lua
local url = require("url")

-- Build API endpoint
function build_api_url(endpoint, params)
    local base = "https://api.example.com/v1"
    local path = base .. endpoint
    
    if params then
        path = path .. "?" .. url.formatQuery(params)
    end
    
    return path
end

local user_url = build_api_url("/users", {
    page = "1",
    limit = "10",
    sort = "name"
})
print(user_url)
-- "https://api.example.com/v1/users?limit=10&page=1&sort=name"
```

### Parsing and Modifying URLs

```lua
local url = require("url")

-- Change query parameters
function set_query_param(url_string, key, value)
    local parsed = url.parse(url_string)
    local params = url.parseQuery(parsed.query or "")
    params[key] = value
    parsed.query = url.formatQuery(params)
    return url.format(parsed)
end

local original = "https://example.com/search?q=lua"
local modified = set_query_param(original, "page", "2")
print(modified)
-- "https://example.com/search?page=2&q=lua"
```

### URL Sanitization

```lua
local url = require("url")

-- Remove sensitive data
function sanitize_url(url_string)
    local parsed = url.parse(url_string)
    parsed.username = nil
    parsed.password = nil
    parsed.query = nil
    parsed.hash = nil
    return url.format(parsed)
end

local sensitive = "https://user:pass@example.com/api?token=secret#debug"
local safe = sanitize_url(sensitive)
print(safe)
-- "https://example.com/api"
```

### Relative URL Helper

```lua
local url = require("url")

-- Get relative URLs from base
function make_relative(base_url)
    return function(path)
        return url.resolve(base_url, path)
    end
end

local api = make_relative("https://api.example.com/v1/")
print(api("users"))        -- "https://api.example.com/v1/users"
print(api("posts/123"))    -- "https://api.example.com/v1/posts/123"
print(api("../v2/data"))   -- "https://api.example.com/v2/data"
```

### Search Query Builder

```lua
local url = require("url")

-- Build search URL
function build_search_url(query, filters)
    local params = {q = query}
    
    for key, value in pairs(filters or {}) do
        params[key] = value
    end
    
    return "https://example.com/search?" .. url.formatQuery(params)
end

local search_url = build_search_url("lua programming", {
    category = "tutorials",
    sort = "recent",
    lang = "en"
})
print(search_url)
-- "https://example.com/search?category=tutorials&lang=en&q=lua+programming&sort=recent"
```

### URL Validator

```lua
local url = require("url")

-- Validate URL structure
function is_valid_url(url_string)
    local ok, parsed = pcall(function()
        return url.parse(url_string)
    end)
    
    if not ok then
        return false
    end
    
    -- Check required components
    return parsed.protocol ~= nil and 
           parsed.hostname ~= nil
end

print(is_valid_url("https://example.com"))      -- true
print(is_valid_url("not-a-url"))                -- false
print(is_valid_url("ftp://files.example.com"))  -- true
```

---

## Performance Notes

- Parsing: O(n) where n is URL length
- Formatting: O(n) where n is number of components
- Resolution: O(n) where n is combined path length
- Encoding/decoding: O(n) where n is string length
- All operations use RFC 3986 compliant parser

---

## Encoding Differences

| Method | Space | Reserved | Use Case |
|--------|-------|----------|----------|
| `encode` | `+` | Encoded | Form data (body) |
| `encodeComponent` | `%20` | Encoded | URL paths, query values |

**Examples:**
```lua
local url = require("url")

local text = "hello world"
print(url.encode(text))           -- "hello+world"
print(url.encodeComponent(text))  -- "hello%20world"

-- Use encode for form bodies
local form_body = url.encode("search query")

-- Use encodeComponent for URL parts
local query_url = "?q=" .. url.encodeComponent("search query")
```

---

## Error Handling

```lua
-- Invalid URL
local ok, err = pcall(function()
    return url.parse("not a url")
end)
if not ok then
    print("Parse error:", err)
end

-- Invalid base URL for resolution
local ok, err = pcall(function()
    return url.resolve("invalid", "path")
end)

-- Malformed percent encoding
local ok, err = pcall(function()
    return url.decode("%ZZ")  -- Invalid hex
end)
```

---

## Comparison with Node.js

| Node.js | Hype-RS | Notes |
|---------|---------|-------|
| `new URL()` | `url.parse()` | Parse URL |
| `url.format()` | `url.format()` | Build URL |
| `url.resolve()` | `url.resolve()` | Resolve relative |
| `encodeURIComponent()` | `url.encodeComponent()` | Percent encode |
| `querystring.parse()` | `url.parseQuery()` | Parse query |
| `querystring.stringify()` | `url.formatQuery()` | Build query |

---

## See Also

- [QueryString Module](./querystring.md) - Dedicated query string utilities
- [Examples](../../examples/url-demo.lua) - More examples
- [Tests](../../tests/url_module_test.rs) - Test suite
- [RFC 3986](https://tools.ietf.org/html/rfc3986) - URL specification

---

**Module**: url  
**Functions**: 9  
**Status**: ✅ Production Ready  
**Last Updated**: October 27, 2025
