# querystring - Query String Parsing and Formatting

> **Utilities for parsing and building URL query strings (application/x-www-form-urlencoded format).**

## Table of Contents
- [Import](#import)
- [Parsing & Building](#parsing--building)
- [Encoding & Decoding](#encoding--decoding)
- [Examples](#examples)

---

## Import

```lua
local querystring = require("querystring")
```

---

## Parsing & Building

### querystring.parse(queryString)

Parse query string into table of key-value pairs.

**Parameters:**
- `queryString: string` - Query string (without leading "?")

**Returns:** `table` - Key-value pairs (strings)

**Example:**
```lua
local querystring = require("querystring")

-- Basic parsing
local params = querystring.parse("name=John&age=30&city=NYC")
print(params.name)   -- "John"
print(params.age)    -- "30"
print(params.city)   -- "NYC"

-- URL encoded values (+ as space)
local encoded = querystring.parse("q=hello+world&name=John+Doe")
print(encoded.q)     -- "hello world"
print(encoded.name)  -- "John Doe"

-- Percent encoding
local special = querystring.parse("email=user%40example.com")
print(special.email)  -- "user@example.com"

-- Empty value
local empty = querystring.parse("key=")
print(empty.key)  -- ""

-- No value
local no_val = querystring.parse("flag")
print(no_val.flag)  -- ""

-- Empty string
local none = querystring.parse("")
-- {} (empty table)
```

---

### querystring.stringify(params)

Convert table of key-value pairs into query string.

**Parameters:**
- `params: table` - Key-value pairs (all values as strings)

**Returns:** `string` - Query string (without leading "?")

**Example:**
```lua
local querystring = require("querystring")

-- Basic stringify
local params = {
    name = "John Doe",
    age = "30",
    city = "New York"
}
local query = querystring.stringify(params)
print(query)
-- "age=30&city=New+York&name=John+Doe"
-- (alphabetically sorted)

-- Special characters
local search = {
    q = "hello world",
    filter = "status:active"
}
print(querystring.stringify(search))
-- "filter=status%3Aactive&q=hello+world"

-- Empty values
local with_empty = {
    key1 = "value",
    key2 = ""
}
print(querystring.stringify(with_empty))
-- "key1=value&key2="

-- Numbers (convert to strings)
local mixed = {
    page = tostring(1),
    limit = tostring(10)
}
print(querystring.stringify(mixed))
-- "limit=10&page=1"
```

---

## Encoding & Decoding

### querystring.escape(string)

URL-encode string for query strings (form-urlencoded).

**Parameters:**
- `string: string` - String to encode

**Returns:** `string` - Encoded string (spaces as `+`)

**Example:**
```lua
local querystring = require("querystring")

-- Space encoding
print(querystring.escape("hello world"))
-- "hello+world"

-- Special characters
print(querystring.escape("email@example.com"))
-- "email%40example.com"

print(querystring.escape("a=b&c=d"))
-- "a%3Db%26c%3Dd"

-- Unicode
print(querystring.escape("Hello 世界"))
-- "Hello+%E4%B8%96%E7%95%8C"

-- Use in query building
local query = "search=" .. querystring.escape("hello world")
-- "search=hello+world"
```

---

### querystring.unescape(string)

URL-decode query string component.

**Parameters:**
- `string: string` - Encoded string

**Returns:** `string` - Decoded string

**Example:**
```lua
local querystring = require("querystring")

-- Decode spaces (+ and %20)
print(querystring.unescape("hello+world"))
-- "hello world"

print(querystring.unescape("hello%20world"))
-- "hello world"

-- Decode special characters
print(querystring.unescape("email%40example.com"))
-- "email@example.com"

-- Roundtrip
local original = "hello world & friends"
local escaped = querystring.escape(original)
local result = querystring.unescape(escaped)
assert(result == original)

-- Decode from URL
local url = "https://example.com?q=hello+world"
local query_part = url:match("%?(.+)")
local decoded = querystring.unescape(query_part:match("q=(.+)"))
print(decoded)  -- "hello world"
```

---

## Examples

### Parse URL Query

```lua
local querystring = require("querystring")

-- Extract query from URL
function get_query_params(url)
    local query = url:match("%?(.+)")
    if not query then
        return {}
    end
    return querystring.parse(query)
end

local url = "https://example.com/search?q=lua&page=2&sort=recent"
local params = get_query_params(url)

print(params.q)      -- "lua"
print(params.page)   -- "2"
print(params.sort)   -- "recent"
```

### Build Query String

```lua
local querystring = require("querystring")

-- Build URL with query parameters
function build_url(base, params)
    if not params or next(params) == nil then
        return base
    end
    
    local query = querystring.stringify(params)
    return base .. "?" .. query
end

local url = build_url("https://api.example.com/users", {
    page = "1",
    limit = "10",
    sort = "name"
})
print(url)
-- "https://api.example.com/users?limit=10&page=1&sort=name"
```

### Form Data Encoding

```lua
local querystring = require("querystring")

-- Encode form data for POST body
function encode_form(data)
    return querystring.stringify(data)
end

local form_data = {
    username = "john_doe",
    password = "secret123",
    remember = "true"
}

local body = encode_form(form_data)
print(body)
-- "password=secret123&remember=true&username=john_doe"

-- Use with HTTP POST
local http = require("http")
http.post("https://example.com/login", {
    headers = {
        ["Content-Type"] = "application/x-www-form-urlencoded"
    },
    body = body
})
```

### Parse Form Response

```lua
local querystring = require("querystring")

-- Parse form-encoded response body
function parse_form_response(body)
    return querystring.parse(body)
end

local response_body = "status=success&user_id=12345&session=abc123"
local data = parse_form_response(response_body)

print(data.status)    -- "success"
print(data.user_id)   -- "12345"
print(data.session)   -- "abc123"
```

### Search Filter Builder

```lua
local querystring = require("querystring")

-- Build search URL with filters
function build_search(query, filters)
    local params = {q = query}
    
    -- Add filters
    for key, value in pairs(filters) do
        params[key] = value
    end
    
    return "https://example.com/search?" .. querystring.stringify(params)
end

local search_url = build_search("lua tutorial", {
    category = "programming",
    level = "beginner",
    sort = "popular"
})
print(search_url)
-- "https://example.com/search?category=programming&level=beginner&q=lua+tutorial&sort=popular"
```

### Modify Query Parameters

```lua
local querystring = require("querystring")

-- Add or update query parameter
function set_query_param(url, key, value)
    local base, query = url:match("([^?]+)%??(.*)")
    local params = querystring.parse(query or "")
    params[key] = value
    return base .. "?" .. querystring.stringify(params)
end

-- Remove query parameter
function remove_query_param(url, key)
    local base, query = url:match("([^?]+)%??(.*)")
    local params = querystring.parse(query or "")
    params[key] = nil
    
    if next(params) == nil then
        return base
    end
    
    return base .. "?" .. querystring.stringify(params)
end

local url = "https://example.com/page?a=1&b=2"
url = set_query_param(url, "c", "3")
-- "https://example.com/page?a=1&b=2&c=3"

url = remove_query_param(url, "b")
-- "https://example.com/page?a=1&c=3"
```

### API Request Helper

```lua
local querystring = require("querystring")
local http = require("http")

-- Helper for GET requests with query params
function api_get(endpoint, params)
    local url = "https://api.example.com" .. endpoint
    if params then
        url = url .. "?" .. querystring.stringify(params)
    end
    return http.get(url)
end

-- Usage
local response = api_get("/users", {
    page = "1",
    per_page = "20",
    sort = "created_at"
})
-- GET https://api.example.com/users?page=1&per_page=20&sort=created_at
```

---

## Performance Notes

- Parse: O(n) where n is query string length
- Stringify: O(n) where n is number of parameters
- Escape/Unescape: O(n) where n is string length
- Parameters are sorted alphabetically in stringify
- No array support (use multiple keys with same name for arrays)

---

## Format Details

**application/x-www-form-urlencoded** format:
- Spaces encoded as `+` (not `%20`)
- Reserved characters percent-encoded (`%XX`)
- Parameters separated by `&`
- Key-value pairs with `=`
- Empty values allowed: `key=`
- Multiple values: `key=val1&key=val2` (not directly supported, parsed as last value)

**Examples:**
```lua
local querystring = require("querystring")

-- Plus sign for spaces
print(querystring.escape("hello world"))
-- "hello+world"

-- Percent encoding for special chars
print(querystring.escape("a&b=c"))
-- "a%26b%3Dc"

-- Combine
local params = {
    search = "hello world",
    filter = "type:user"
}
print(querystring.stringify(params))
-- "filter=type%3Auser&search=hello+world"
```

---

## Comparison with URL Module

| Operation | querystring | url |
|-----------|-------------|-----|
| Parse query | `parse()` | `parseQuery()` |
| Build query | `stringify()` | `formatQuery()` |
| Encode | `escape()` (+ for space) | `encode()` (+ for space) |
| Encode component | N/A | `encodeComponent()` (%20) |
| Decode | `unescape()` | `decode()` |

**When to use:**
- **querystring**: Working with query strings only
- **url**: Full URL parsing and manipulation

```lua
-- Same functionality
local querystring = require("querystring")
local url = require("url")

local query = "name=John&age=30"

-- Equivalent
local p1 = querystring.parse(query)
local p2 = url.parseQuery(query)
-- Both produce same result

local params = {name = "John", age = "30"}

-- Equivalent
local q1 = querystring.stringify(params)
local q2 = url.formatQuery(params)
-- Both produce same result
```

---

## Error Handling

```lua
-- Malformed encoding
local ok, err = pcall(function()
    return querystring.unescape("%ZZ")  -- Invalid hex
end)
if not ok then
    print("Decode error:", err)
end

-- Empty values are fine
local params = querystring.parse("key=")
-- {key = ""}

-- No value is fine
local no_val = querystring.parse("flag")
-- {flag = ""}
```

---

## Limitations

- **No array support**: Multiple values with same key not directly supported
- **No nested objects**: Flat key-value pairs only
- **String values only**: Numbers must be converted to strings
- **No type preservation**: All values parsed as strings

**Workarounds:**

```lua
local querystring = require("querystring")
local json = require("json")

-- Arrays: use JSON encoding
local array_params = {
    filters = json.encode({"active", "verified"})
}
local query = querystring.stringify(array_params)
-- "filters=%5B%22active%22%2C%22verified%22%5D"

-- Parse back
local parsed = querystring.parse(query)
local filters = json.decode(parsed.filters)
-- {"active", "verified"}

-- Numbers: convert to/from strings
local numeric = {
    page = tostring(1),
    limit = tostring(10)
}
local query2 = querystring.stringify(numeric)

local parsed2 = querystring.parse(query2)
local page_num = tonumber(parsed2.page)
-- 1
```

---

## See Also

- [URL Module](./url.md) - Full URL parsing and manipulation
- [Examples](../../examples/querystring-demo.lua) - More examples
- [Tests](../../tests/querystring_module_test.rs) - Test suite
- [RFC 3986](https://tools.ietf.org/html/rfc3986) - URL specification

---

**Module**: querystring  
**Functions**: 4  
**Status**: ✅ Production Ready  
**Last Updated**: October 27, 2025
