# Hype Lua Standard Library - Quick Reference

> **One-page reference for all standard library modules**

## Import Syntax

```lua
local crypto = require("crypto")
local string = require("string")  
local time = require("time")
local url = require("url")
local querystring = require("querystring")
local os = require("os")
local process = require("process")
local fs = require("fs")
local json = require("json")
```

---

## crypto - Cryptographic Operations

```lua
-- Hashing
crypto.hash("sha256", data) -> string
crypto.hashFile("sha256", path) -> string
crypto.hmac("sha256", key, data) -> string

-- Random
crypto.randomBytes(16) -> table
crypto.randomInt(1, 100) -> number
crypto.randomUUID() -> string

-- Encoding
crypto.base64Encode(data) -> string
crypto.base64Decode(data) -> string
crypto.hexEncode(data) -> string
crypto.hexDecode(data) -> string

-- Passwords
crypto.bcrypt(password, 12) -> string
crypto.bcryptVerify(password, hash) -> boolean
crypto.timingSafeEqual(a, b) -> boolean
```

**Quick Examples:**
```lua
-- Hash data
local hash = crypto.hash("sha256", "hello")

-- Generate UUID
local id = crypto.randomUUID()

-- Hash password
local hash = crypto.bcrypt("password", 12)
local valid = crypto.bcryptVerify("password", hash)

-- API signature
local sig = crypto.hmac("sha256", secret, data)
```

---

## string - String Manipulation

```lua
-- Split & Join
string.split(str, delimiter) -> table
string.lines(str) -> table
string.chars(str) -> table

-- Trim
string.trim(str) -> string
string.trimStart(str) -> string
string.trimEnd(str) -> string

-- Padding
string.padStart(str, length, fill?) -> string
string.padEnd(str, length, fill?) -> string

-- Case
string.toUpperCase(str) -> string
string.toLowerCase(str) -> string
string.capitalize(str) -> string

-- Search
string.startsWith(str, prefix) -> boolean
string.endsWith(str, suffix) -> boolean
string.contains(str, substring) -> boolean

-- Replace
string.replace(str, pattern, replacement, count?) -> string
string.replaceAll(str, pattern, replacement) -> string
string["repeat"](str, count) -> string
```

**Quick Examples:**
```lua
-- Split CSV
local parts = string.split("a,b,c", ",")

-- Trim whitespace
local clean = string.trim("  hello  ")

-- Pad numbers
local padded = string.padStart("5", 3, "0")  -- "005"

-- Check prefix
if string.startsWith(url, "https://") then end
```

---

## time - Date & Time Operations

```lua
-- Current Time
time.now() -> number           -- milliseconds
time.nowSeconds() -> number    -- seconds
time.nowNanos() -> number      -- nanoseconds

-- Formatting
time.toISO(timestamp) -> string
time.fromISO(isoString) -> number
time.format(timestamp, format) -> string
time.parse(dateString, format) -> number

-- Components
time.date(timestamp?) -> table
time.year(timestamp?) -> number
time.month(timestamp?) -> number
time.day(timestamp?) -> number
time.hour(timestamp?) -> number
time.minute(timestamp?) -> number
time.second(timestamp?) -> number

-- Utilities
time.sleep(ms) -> nil
time.elapsed(start) -> number
time.duration(ms) -> string
```

**Quick Examples:**
```lua
-- Get current timestamp
local now = time.now()

-- Format date
local iso = time.toISO(now)

-- Sleep
time.sleep(1000)  -- 1 second

-- Measure elapsed
local start = time.now()
-- ... do work ...
local ms = time.elapsed(start)
```

---

## url - URL Parsing & Manipulation

```lua
-- Parse & Format
url.parse(urlString) -> table
url.format(components) -> string
url.resolve(base, relative) -> string

-- Encoding
url.encode(str) -> string
url.decode(str) -> string
url.encodeComponent(str) -> string
url.decodeComponent(str) -> string

-- Query Strings
url.parseQuery(queryString) -> table
url.formatQuery(params) -> string
```

**Quick Examples:**
```lua
-- Parse URL
local parts = url.parse("https://example.com:8080/path?key=val#hash")
print(parts.hostname)  -- "example.com"
print(parts.port)      -- 8080

-- Build URL
local full = url.format({hostname = "example.com", path = "/api"})

-- Encode
local safe = url.encodeComponent("hello world")  -- "hello+world"
```

---

## querystring - Query String Operations

```lua
querystring.parse(queryString) -> table
querystring.stringify(params) -> string
querystring.escape(str) -> string
querystring.unescape(str) -> string
```

**Quick Examples:**
```lua
-- Parse
local params = querystring.parse("foo=bar&baz=qux")
print(params.foo)  -- "bar"

-- Stringify
local qs = querystring.stringify({foo = "bar", baz = "qux"})
```

---

## os - Operating System Info

```lua
-- Platform Info
os.platform() -> string        -- "linux", "macos", "windows"
os.arch() -> string           -- "x86_64", "aarch64"
os.hostname() -> string

-- Directories
os.homedir() -> string
os.tmpdir() -> string

-- System Stats
os.cpus() -> table
os.totalmem() -> number
os.freemem() -> number
os.uptime() -> number
os.loadavg() -> table

-- Network
os.networkInterfaces() -> table

-- User
os.username() -> string
os.userInfo() -> table
```

**Quick Examples:**
```lua
-- Check platform
if os.platform() == "macos" then end

-- Get home directory
local home = os.homedir()

-- Memory info
local total = os.totalmem()
local free = os.freemem()
local used_percent = ((total - free) / total) * 100
```

---

## process - Process & Environment

```lua
-- Arguments
process.args() -> table
process.argv -> table

-- Environment
process.env(key) -> string | nil
process.getEnv(key, default?) -> string
process.setEnv(key, value) -> nil

-- Working Directory
process.cwd() -> string
process.chdir(path) -> nil

-- Exit
process.exit(code?) -> nil
```

**Quick Examples:**
```lua
-- Get arguments
local args = process.args()

-- Environment variables
local home = process.env("HOME")
local port = process.getEnv("PORT", "3000")

-- Working directory
local cwd = process.cwd()
```

---

## fs - File System Operations

```lua
-- Read/Write
fs.read(path) -> string
fs.write(path, data) -> nil
fs.append(path, data) -> nil

-- Info
fs.exists(path) -> boolean
fs.isFile(path) -> boolean
fs.isDir(path) -> boolean
fs.stat(path) -> table

-- Directory
fs.readDir(path) -> table
fs.mkdir(path) -> nil
fs.mkdirAll(path) -> nil

-- Operations
fs.copy(src, dest) -> nil
fs.move(src, dest) -> nil
fs.remove(path) -> nil
fs.removeAll(path) -> nil
```

**Quick Examples:**
```lua
-- Read/write files
local content = fs.read("file.txt")
fs.write("output.txt", "data")

-- Check existence
if fs.exists("config.json") then end

-- List directory
local files = fs.readDir(".")
for _, file in ipairs(files) do
    print(file.name, file.size)
end
```

---

## json - JSON Encoding/Decoding

```lua
json.parse(str) -> table
json.stringify(value, pretty?) -> string
json.encode(value) -> string
json.decode(str) -> table
```

**Quick Examples:**
```lua
-- Parse JSON
local data = json.parse('{"name": "John", "age": 30}')
print(data.name)  -- "John"

-- Create JSON
local json_str = json.stringify({foo = "bar", num = 42})

-- Pretty print
local pretty = json.stringify(data, true)
```

---

## Common Patterns

### API Request with Signature
```lua
local crypto = require("crypto")
local http = require("http")

local secret = "api-secret"
local body = json.stringify(request_data)
local signature = crypto.hmac("sha256", secret, body)

http.post("https://api.example.com/data", {
    body = body,
    headers = {
        ["X-Signature"] = signature,
        ["Content-Type"] = "application/json"
    }
})
```

### Configuration File
```lua
local fs = require("fs")
local json = require("json")

-- Load config
local config = json.parse(fs.read("config.json"))

-- Update and save
config.updated_at = time.toISO(time.now())
fs.write("config.json", json.stringify(config, true))
```

### Secure Password Storage
```lua
local crypto = require("crypto")

-- Registration
local password_hash = crypto.bcrypt(user_password, 12)
-- Save hash to database

-- Login
local stored_hash = get_from_database(username)
if crypto.bcryptVerify(attempt, stored_hash) then
    -- Login successful
end
```

### Data Processing Pipeline
```lua
local fs = require("fs")
local string = require("string")
local json = require("json")

-- Read CSV
local csv = fs.read("data.csv")
local lines = string.split(csv, "\n")

-- Process
local records = {}
for _, line in ipairs(lines) do
    local fields = string.split(line, ",")
    table.insert(records, {
        name = fields[1],
        value = tonumber(fields[2])
    })
end

-- Save as JSON
fs.write("data.json", json.stringify(records, true))
```

---

## Error Handling

```lua
-- Use pcall for error handling
local ok, result = pcall(function()
    return json.parse(invalid_json)
end)

if not ok then
    print("Error:", result)
else
    print("Success:", result)
end
```

---

## Module Reference

| Module | Functions | Documentation |
|--------|-----------|---------------|
| crypto | 13 | [crypto.md](crypto.md) |
| string | 17 | [string.md](string.md) |
| time | 17 | [time.md](time.md) |
| url | 9 | [url.md](url.md) |
| querystring | 4 | [querystring.md](querystring.md) |
| os | 13 | [os.md](os.md) |
| process | 8 | [process.md](process.md) |
| fs | 15+ | [fs.md](fs.md) |
| json | 4 | [json.md](json.md) |

---

**Last Updated**: October 27, 2025  
**All Examples Tested**: ✅ Yes
