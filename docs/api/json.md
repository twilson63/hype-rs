# json - JSON Encoding and Decoding

> **Fast JSON encoding and decoding with full Unicode support.**

## Table of Contents
- [Import](#import)
- [Encoding](#encoding)
- [Decoding](#decoding)
- [Type Mapping](#type-mapping)
- [Examples](#examples)

---

## Import

```lua
local json = require("json")
```

---

## Encoding

### json.encode(value, pretty?)

Encode Lua value to JSON string.

**Parameters:**
- `value: any` - Lua value to encode
- `pretty?: boolean` - Enable pretty-printing (default: `false`)

**Returns:** `string` - JSON string

**Example:**
```lua
local json = require("json")

-- Basic encoding
local data = {name = "John", age = 30}
local str = json.encode(data)
print(str)  -- {"name":"John","age":30}

-- Pretty printing
local pretty = json.encode(data, true)
print(pretty)
-- {
--   "name": "John",
--   "age": 30
-- }

-- Arrays
local arr = {"a", "b", "c"}
print(json.encode(arr))  -- ["a","b","c"]

-- Nested objects
local nested = {
    user = {
        name = "John",
        contacts = {"email", "phone"}
    }
}
print(json.encode(nested, true))
```

---

### json.stringify(value, pretty?)

Alias for `json.encode()`.

**Parameters:**
- `value: any` - Lua value to encode
- `pretty?: boolean` - Enable pretty-printing (default: `false`)

**Returns:** `string` - JSON string

**Example:**
```lua
local json = require("json")

-- Same as encode()
local str = json.stringify({key = "value"})
print(str)  -- {"key":"value"}

-- With pretty printing
local pretty = json.stringify({a = 1, b = 2}, true)
```

---

## Decoding

### json.decode(jsonString)

Decode JSON string to Lua value.

**Parameters:**
- `jsonString: string` - JSON string to decode

**Returns:** `any` - Lua value (table, string, number, boolean, or nil)

**Example:**
```lua
local json = require("json")

-- Decode object
local obj = json.decode('{"name":"John","age":30}')
print(obj.name)  -- "John"
print(obj.age)   -- 30

-- Decode array
local arr = json.decode('["a","b","c"]')
print(arr[1])  -- "a"
print(arr[2])  -- "b"

-- Decode primitives
print(json.decode('"hello"'))  -- "hello"
print(json.decode('42'))       -- 42
print(json.decode('true'))     -- true
print(json.decode('null'))     -- nil

-- Error handling
local ok, result = pcall(function()
    return json.decode('invalid json')
end)
if not ok then
    print("Parse error:", result)
end
```

---

### json.parse(jsonString)

Alias for `json.decode()`.

**Parameters:**
- `jsonString: string` - JSON string to decode

**Returns:** `any` - Lua value

**Example:**
```lua
local json = require("json")

-- Same as decode()
local data = json.parse('{"key":"value"}')
print(data.key)  -- "value"
```

---

## Type Mapping

### Lua to JSON

| Lua Type | JSON Type | Example |
|----------|-----------|---------|
| `nil` | `null` | `nil` → `null` |
| `boolean` | `boolean` | `true` → `true` |
| `number` | `number` | `42` → `42` |
| `string` | `string` | `"hello"` → `"hello"` |
| `table` (array) | `array` | `{1,2,3}` → `[1,2,3]` |
| `table` (object) | `object` | `{a=1}` → `{"a":1}` |

**Notes:**
- Tables with sequential numeric keys (1, 2, 3...) encode as JSON arrays
- Tables with string/mixed keys encode as JSON objects
- Functions, userdata, threads cannot be encoded (throw error)

**Example:**
```lua
local json = require("json")

-- Array (sequential keys)
local arr = {1, 2, 3}
print(json.encode(arr))  -- [1,2,3]

-- Object (string keys)
local obj = {a = 1, b = 2}
print(json.encode(obj))  -- {"a":1,"b":2}

-- Mixed (treated as object)
local mixed = {1, 2, key = "value"}
print(json.encode(mixed))  -- {"1":1,"2":2,"key":"value"}

-- Nested
local data = {
    users = {
        {name = "John", age = 30},
        {name = "Jane", age = 25}
    }
}
print(json.encode(data, true))
```

---

### JSON to Lua

| JSON Type | Lua Type | Example |
|-----------|----------|---------|
| `null` | `nil` | `null` → `nil` |
| `boolean` | `boolean` | `true` → `true` |
| `number` | `number` | `42` → `42` |
| `string` | `string` | `"hello"` → `"hello"` |
| `array` | `table` (1-indexed) | `[1,2,3]` → `{1,2,3}` |
| `object` | `table` | `{"a":1}` → `{a=1}` |

**Important:** JSON arrays are decoded to 1-indexed Lua tables.

**Example:**
```lua
local json = require("json")

-- Array indices
local arr = json.decode('[10,20,30]')
print(arr[1])  -- 10 (NOT arr[0])
print(arr[2])  -- 20
print(arr[3])  -- 30

-- Object access
local obj = json.decode('{"name":"John"}')
print(obj.name)  -- "John"
print(obj["name"])  -- "John"
```

---

## Examples

### Configuration Files

```lua
local json = require("json")
local fs = require("fs")

-- Load config
function load_config(path)
    local content = fs.readFileSync(path)
    return json.decode(content)
end

-- Save config
function save_config(path, config)
    local content = json.encode(config, true)
    fs.writeFileSync(path, content)
end

-- Usage
local config = load_config("config.json")
config.port = 8080
save_config("config.json", config)
```

### API Request/Response

```lua
local json = require("json")
local http = require("http")

-- POST JSON data
function api_post(url, data)
    local body = json.encode(data)
    return http.post(url, {
        headers = {
            ["Content-Type"] = "application/json"
        },
        body = body
    })
end

-- Parse JSON response
function api_get(url)
    local response = http.get(url)
    return json.decode(response.body)
end

-- Usage
local user = api_get("https://api.example.com/user/123")
print("Name:", user.name)

api_post("https://api.example.com/users", {
    name = "John",
    email = "john@example.com"
})
```

### Data Transformation

```lua
local json = require("json")

-- Convert CSV to JSON
function csv_to_json(csv_text)
    local lines = {}
    for line in csv_text:gmatch("[^\n]+") do
        table.insert(lines, line)
    end
    
    local headers = {}
    for header in lines[1]:gmatch("[^,]+") do
        table.insert(headers, header)
    end
    
    local result = {}
    for i = 2, #lines do
        local row = {}
        local col = 1
        for value in lines[i]:gmatch("[^,]+") do
            row[headers[col]] = value
            col = col + 1
        end
        table.insert(result, row)
    end
    
    return json.encode(result, true)
end

local csv = [[
name,age,city
John,30,NYC
Jane,25,LA
]]

print(csv_to_json(csv))
-- [
--   {"name":"John","age":"30","city":"NYC"},
--   {"name":"Jane","age":"25","city":"LA"}
-- ]
```

### Deep Clone

```lua
local json = require("json")

-- Clone table via JSON roundtrip
function deep_clone(obj)
    return json.decode(json.encode(obj))
end

local original = {
    user = {
        name = "John",
        tags = {"admin", "user"}
    }
}

local copy = deep_clone(original)
copy.user.name = "Jane"

print(original.user.name)  -- "John" (unchanged)
print(copy.user.name)      -- "Jane"
```

### Pretty Print

```lua
local json = require("json")

-- Pretty print any Lua table
function pp(value)
    print(json.encode(value, true))
end

-- Usage
local data = {
    users = {
        {id = 1, name = "John"},
        {id = 2, name = "Jane"}
    },
    count = 2
}

pp(data)
-- {
--   "users": [
--     {"id": 1, "name": "John"},
--     {"id": 2, "name": "Jane"}
--   ],
--   "count": 2
-- }
```

### Validation

```lua
local json = require("json")

-- Validate JSON string
function is_valid_json(str)
    local ok, _ = pcall(function()
        return json.decode(str)
    end)
    return ok
end

print(is_valid_json('{"valid": true}'))   -- true
print(is_valid_json('{invalid json}'))    -- false

-- Validate and parse
function safe_parse(str, default)
    local ok, result = pcall(function()
        return json.decode(str)
    end)
    return ok and result or default
end

local data = safe_parse('invalid', {})
-- data = {} (default)
```

### Logging

```lua
local json = require("json")
local time = require("time")

function log_json(level, message, data)
    local entry = {
        timestamp = time.toISO(time.now()),
        level = level,
        message = message,
        data = data or {}
    }
    print(json.encode(entry))
end

-- Structured logging
log_json("info", "User login", {
    user_id = 123,
    ip = "192.168.1.1"
})
-- {"timestamp":"2025-10-27T12:30:45Z","level":"info","message":"User login","data":{"user_id":123,"ip":"192.168.1.1"}}
```

---

## Performance Notes

- Encoding: Very fast, O(n) where n is data size
- Decoding: Very fast, O(n) where n is JSON string length
- Pretty printing: Slightly slower due to formatting
- Unicode: Full UTF-8 support including emojis
- Large objects: Entire JSON processed in memory

---

## Unicode Support

```lua
local json = require("json")

-- Full Unicode support
local data = {
    english = "Hello",
    chinese = "你好",
    emoji = "🌍🚀"
}

local str = json.encode(data, true)
print(str)

local parsed = json.decode(str)
print(parsed.emoji)  -- "🌍🚀"
```

---

## Error Handling

```lua
local json = require("json")

-- Invalid JSON
local ok, err = pcall(function()
    return json.decode('{invalid}')
end)
if not ok then
    print("Parse error:", err)
    -- Parse error: expected value at line 1 column 2
end

-- Unencodable values
local ok, err = pcall(function()
    return json.encode({func = function() end})
end)
-- Error: Cannot encode function

-- Empty string
local ok, err = pcall(function()
    return json.decode('')
end)
-- Error: EOF while parsing
```

---

## Limitations

- **No circular references**: Tables with circular references cannot be encoded
- **No sparse arrays**: Sparse arrays may not encode as expected
- **No metatable preservation**: Metatables lost during encode/decode
- **No function encoding**: Functions cannot be serialized
- **Memory usage**: Entire JSON loaded into memory

**Circular reference example:**
```lua
local json = require("json")

local t = {}
t.self = t  -- Circular reference

local ok, err = pcall(function()
    return json.encode(t)
end)
-- Error: Maximum recursion depth exceeded
```

---

## Common Patterns

**Config with defaults:**
```lua
local default = {port = 3000, debug = false}
local config = json.decode(fs.readFileSync("config.json"))
for k, v in pairs(default) do
    config[k] = config[k] or v
end
```

**Roundtrip test:**
```lua
local original = {a = 1, b = "test"}
local encoded = json.encode(original)
local decoded = json.decode(encoded)
assert(decoded.a == original.a)
assert(decoded.b == original.b)
```

**Safe decode with default:**
```lua
local data = pcall(json.decode, input) and json.decode(input) or {}
```

---

## Comparison with Alternatives

| Library | Speed | Unicode | Pretty | Size |
|---------|-------|---------|--------|------|
| Hype JSON | ✅ Fast | ✅ Full | ✅ Yes | Small |
| cjson | ✅ Fast | ⚠️ Limited | ✅ Yes | Medium |
| dkjson | ⚠️ Slow | ✅ Full | ✅ Yes | Small |

---

## See Also

- [Examples](../../examples/json-demo.lua) - More examples
- [Tests](../../tests/json_module_test.rs) - Test suite
- [serde_json](https://docs.rs/serde_json/) - Underlying library

---

**Module**: json  
**Functions**: 4 (encode, decode, stringify, parse)  
**Status**: ✅ Production Ready  
**Last Updated**: October 27, 2025
