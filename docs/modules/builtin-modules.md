# Built-in Modules Reference

Hype-RS provides 6 built-in modules for common operations. These are always available and require no installation.

## Table of Contents

- [fs Module](#fs-module)
- [path Module](#path-module)
- [events Module](#events-module)
- [util Module](#util-module)
- [table Module](#table-module)
- [http Module](#http-module) 🆕

---

## fs Module

File system operations for reading, writing, and manipulating files.

### Overview

The `fs` module provides synchronous file I/O operations. All functions are synchronous and block until complete.

### Load

```lua
local fs = require("fs")
```

### Functions

#### readFileSync(path: string) → string

Read entire file contents as a string.

**Parameters:**
- `path` (string): Path to file to read

**Returns:**
- (string): File contents

**Errors:**
- `FILE_NOT_FOUND`: File doesn't exist
- `PERMISSION_DENIED`: No read permission
- `IS_DIRECTORY`: Path is a directory, not a file

**Example:**
```lua
local fs = require("fs")

local content = fs.readFileSync("config.json")
print("File size:", #content, "bytes")
```

**Error handling:**
```lua
local ok, content = pcall(function()
    return fs.readFileSync("data.txt")
end)

if ok then
    print("Content:", content)
else
    print("Error reading file:", content)
end
```

#### writeFileSync(path: string, data: string) → nil

Write string data to file. Creates file if it doesn't exist, overwrites if it does.

**Parameters:**
- `path` (string): Path to file to write
- `data` (string): Content to write

**Returns:**
- None (nil)

**Errors:**
- `PERMISSION_DENIED`: No write permission
- `PARENT_NOT_FOUND`: Parent directory doesn't exist

**Example:**
```lua
local fs = require("fs")

fs.writeFileSync("output.txt", "Hello, World!")
print("File written successfully")
```

**Multiple writes:**
```lua
local fs = require("fs")

fs.writeFileSync("log.txt", "")  -- Clear file
fs.writeFileSync("log.txt", "Line 1\n")
fs.writeFileSync("log.txt", "Line 2\n")
```

#### existsSync(path: string) → boolean

Check if file or directory exists.

**Parameters:**
- `path` (string): Path to check

**Returns:**
- (boolean): `true` if path exists, `false` otherwise

**Example:**
```lua
local fs = require("fs")

if fs.existsSync("data.txt") then
    print("File exists")
else
    print("File not found")
end
```

**Conditional file operations:**
```lua
local fs = require("fs")

local file = "backup.dat"
if not fs.existsSync(file) then
    fs.writeFileSync(file, "default data")
end

local content = fs.readFileSync(file)
```

#### statSync(path: string) → table

Get file metadata (size, modification time, type, etc.).

**Parameters:**
- `path` (string): Path to file

**Returns:**
- (table): File metadata with fields:
  - `size` (number): File size in bytes
  - `modified` (number): Last modification time (Unix timestamp)
  - `is_file` (boolean): Is regular file
  - `is_directory` (boolean): Is directory
  - `is_symlink` (boolean): Is symbolic link

**Errors:**
- `FILE_NOT_FOUND`: Path doesn't exist

**Example:**
```lua
local fs = require("fs")

local stat = fs.statSync("data.txt")
print("Size:", stat.size, "bytes")
print("Type:", stat.is_file and "file" or "directory")
```

**Get modification time:**
```lua
local fs = require("fs")

local stat = fs.statSync("config.json")
print("Last modified:", stat.modified)
```

#### readdirSync(path: string) → table

List contents of directory.

**Parameters:**
- `path` (string): Directory path

**Returns:**
- (table): Array of filenames (1-indexed Lua table)

**Errors:**
- `FILE_NOT_FOUND`: Directory doesn't exist
- `NOT_DIRECTORY`: Path is not a directory

**Example:**
```lua
local fs = require("fs")

local files = fs.readdirSync(".")
for i, filename in ipairs(files) do
    print(i, filename)
end
```

**Process directory:**
```lua
local fs = require("fs")

local entries = fs.readdirSync("data")
for _, name in ipairs(entries) do
    if name:sub(-4) == ".txt" then
        print("Text file:", name)
    end
end
```

#### unlinkSync(path: string) → nil

Delete a file.

**Parameters:**
- `path` (string): File path to delete

**Returns:**
- None (nil)

**Errors:**
- `FILE_NOT_FOUND`: File doesn't exist
- `PERMISSION_DENIED`: No permission
- `IS_DIRECTORY`: Path is directory, not file

**Example:**
```lua
local fs = require("fs")

if fs.existsSync("temp.txt") then
    fs.unlinkSync("temp.txt")
    print("Temporary file deleted")
end
```

#### mkdirSync(path: string) → nil

Create a directory.

**Parameters:**
- `path` (string): Directory path to create

**Returns:**
- None (nil)

**Errors:**
- `ALREADY_EXISTS`: Directory already exists
- `PARENT_NOT_FOUND`: Parent directory doesn't exist
- `PERMISSION_DENIED`: No permission

**Example:**
```lua
local fs = require("fs")

fs.mkdirSync("output")
fs.writeFileSync("output/data.txt", "content")
```

#### rmdirSync(path: string) → nil

Remove an empty directory.

**Parameters:**
- `path` (string): Directory path to remove

**Returns:**
- None (nil)

**Errors:**
- `NOT_FOUND`: Directory doesn't exist
- `NOT_EMPTY`: Directory contains files
- `PERMISSION_DENIED`: No permission

**Example:**
```lua
local fs = require("fs")

-- Make sure directory is empty first
fs.rmdirSync("empty_dir")
```

### Complete Example

```lua
local fs = require("fs")
local path = require("path")

-- Create working directory
if not fs.existsSync("work") then
    fs.mkdirSync("work")
end

-- Write file
local output_path = path.join("work", "data.txt")
fs.writeFileSync(output_path, "Hello, World!")
print("✓ File written")

-- Read file back
local content = fs.readFileSync(output_path)
print("✓ File contents:", content)

-- Get file info
local stat = fs.statSync(output_path)
print("✓ File size:", stat.size, "bytes")

-- List directory
local files = fs.readdirSync("work")
print("✓ Directory contents:", table.concat(files, ", "))

-- Cleanup
fs.unlinkSync(output_path)
fs.rmdirSync("work")
print("✓ Cleaned up")
```

---

## path Module

Path manipulation utilities for working with file and directory paths.

### Overview

The `path` module provides utilities for path operations. It handles path separators across platforms automatically.

### Load

```lua
local path = require("path")
```

### Functions

#### join(...: string) → string

Join path segments into a single path.

**Parameters:**
- `...` (string, variable): Path segments to join

**Returns:**
- (string): Joined path

**Example:**
```lua
local path = require("path")

local p = path.join("home", "user", "documents", "file.txt")
print(p)  -- "home/user/documents/file.txt" (or \ on Windows)
```

**With current directory:**
```lua
local path = require("path")

local datafile = path.join(".", "data", "input.json")
print(datafile)  -- "./data/input.json"
```

#### dirname(path: string) → string

Get the directory portion of a path.

**Parameters:**
- `path` (string): File path

**Returns:**
- (string): Directory path

**Example:**
```lua
local path = require("path")

print(path.dirname("/home/user/file.txt"))  -- "/home/user"
print(path.dirname("file.txt"))             -- "."
```

#### basename(path: string, extension?: string) → string

Get the filename portion of a path, optionally removing extension.

**Parameters:**
- `path` (string): File path
- `extension` (string, optional): Extension to remove

**Returns:**
- (string): Filename

**Example:**
```lua
local path = require("path")

print(path.basename("/home/user/file.txt"))           -- "file.txt"
print(path.basename("/home/user/file.txt", ".txt"))   -- "file"
```

#### extname(path: string) → string

Get the file extension.

**Parameters:**
- `path` (string): File path

**Returns:**
- (string): Extension including dot, or empty string if no extension

**Example:**
```lua
local path = require("path")

print(path.extname("file.txt"))      -- ".txt"
print(path.extname("archive.tar.gz") -- ".gz"
print(path.extname("README"))        -- ""
```

#### resolve(...: string) → string

Resolve a path to an absolute path.

**Parameters:**
- `...` (string, variable): Path segments to resolve

**Returns:**
- (string): Absolute path

**Example:**
```lua
local path = require("path")

print(path.resolve(".", "data"))  -- "/current/working/directory/data"
print(path.resolve("..", "parent"))  -- "/parent/directory"
```

### Complete Example

```lua
local path = require("path")

-- Parse a file path
local filepath = "/home/user/documents/report.pdf"

print("Full path:", filepath)
print("Directory:", path.dirname(filepath))      -- "/home/user/documents"
print("Filename:", path.basename(filepath))      -- "report.pdf"
print("Base name:", path.basename(filepath, ".pdf"))  -- "report"
print("Extension:", path.extname(filepath))      -- ".pdf"

-- Build paths safely (cross-platform)
local project_dir = "."
local config_file = path.join(project_dir, "config", "app.json")
print("Config path:", config_file)

-- Resolve to absolute path
local abs_config = path.resolve(config_file)
print("Absolute path:", abs_config)
```

---

## events Module

Event emitter for publishing and subscribing to events.

### Overview

The `events` module provides an event emitter pattern for decoupling code components. Useful for callbacks, notifications, and async-like patterns.

### Load

```lua
local events = require("events")
```

### Classes

#### EventEmitter

Event emitter class for managing events.

### Creating an Emitter

```lua
local events = require("events")

local emitter = events.EventEmitter:new()
```

### Methods

#### emitter:on(event_name: string, listener: function) → self

Register an event listener.

**Parameters:**
- `event_name` (string): Name of event
- `listener` (function): Callback function

**Returns:**
- (self): Returns emitter for chaining

**Example:**
```lua
local events = require("events")
local emitter = events.EventEmitter:new()

emitter:on("message", function(data)
    print("Received:", data)
end)
```

#### emitter:emit(event_name: string, ...: any) → nil

Emit an event, calling all registered listeners.

**Parameters:**
- `event_name` (string): Name of event
- `...` (any): Arguments to pass to listeners

**Returns:**
- None (nil)

**Example:**
```lua
local events = require("events")
local emitter = events.EventEmitter:new()

emitter:on("ready", function()
    print("System ready!")
end)

emitter:emit("ready")  -- Calls listener
```

**With data:**
```lua
local emitter = events.EventEmitter:new()

emitter:on("user_login", function(username)
    print("User logged in:", username)
end)

emitter:emit("user_login", "alice")  -- Calls listener with "alice"
```

#### emitter:off(event_name: string, listener?: function) → self

Unregister an event listener.

**Parameters:**
- `event_name` (string): Name of event
- `listener` (function, optional): Specific listener to remove

**Returns:**
- (self): Returns emitter for chaining

**Example:**
```lua
local emitter = events.EventEmitter:new()

local function on_data(data)
    print("Data:", data)
end

emitter:on("data", on_data)
emitter:emit("data", "hello")

emitter:off("data", on_data)
emitter:emit("data", "goodbye")  -- Listener not called
```

### Complete Example

```lua
local events = require("events")

-- Create event bus
local bus = events.EventEmitter:new()

-- Register handlers
bus:on("start", function()
    print("Application starting...")
end)

bus:on("error", function(message)
    print("Error occurred:", message)
end)

bus:on("complete", function(result)
    print("Completed with result:", result)
end)

-- Emit events
bus:emit("start")
bus:emit("error", "Something went wrong")
bus:emit("complete", "Success")
```

---

## util Module

General utility functions for common operations.

### Overview

The `util` module provides miscellaneous utility functions for string manipulation, type checking, and debugging.

### Load

```lua
local util = require("util")
```

### Functions

#### util.inspect(value: any) → string

Convert a value to a readable string representation (for debugging).

**Parameters:**
- `value` (any): Value to inspect

**Returns:**
- (string): String representation

**Example:**
```lua
local util = require("util")

print(util.inspect({a = 1, b = 2}))
print(util.inspect(function() end))
```

#### util.type(value: any) → string

Get the type of a value (enhanced version of Lua's type()).

**Parameters:**
- `value` (any): Value to check

**Returns:**
- (string): Type name

**Example:**
```lua
local util = require("util")

print(util.type(42))           -- "number"
print(util.type("hello"))      -- "string"
print(util.type({}))           -- "table"
print(util.type(function() end) -- "function"
```

### Example

```lua
local util = require("util")

local function debug_value(name, value)
    print(name .. " (" .. util.type(value) .. "):", util.inspect(value))
end

debug_value("count", 42)
debug_value("data", {x = 1, y = 2})
```

---

## table Module

Table operations for manipulating Lua tables.

### Overview

The `table` module extends Lua's built-in table functionality with additional utility functions.

### Load

```lua
local table_utils = require("table")
```

### Functions

#### table_utils.keys(tbl: table) → table

Get all keys from a table.

**Parameters:**
- `tbl` (table): Table to extract keys from

**Returns:**
- (table): Array of keys

**Example:**
```lua
local table_utils = require("table")

local data = {a = 1, b = 2, c = 3}
local keys = table_utils.keys(data)

for _, key in ipairs(keys) do
    print(key)
end
```

#### table_utils.values(tbl: table) → table

Get all values from a table.

**Parameters:**
- `tbl` (table): Table to extract values from

**Returns:**
- (table): Array of values

**Example:**
```lua
local table_utils = require("table")

local data = {a = 1, b = 2, c = 3}
local values = table_utils.values(data)

for _, value in ipairs(values) do
    print(value)
end
```

#### table_utils.length(tbl: table) → number

Get the number of key-value pairs in a table.

**Parameters:**
- `tbl` (table): Table to measure

**Returns:**
- (number): Number of entries

**Example:**
```lua
local table_utils = require("table")

local data = {a = 1, b = 2, c = 3}
print("Entries:", table_utils.length(data))  -- 3
```

#### table_utils.merge(tbl1: table, tbl2: table) → table

Merge two tables into a new table.

**Parameters:**
- `tbl1` (table): First table
- `tbl2` (table): Second table

**Returns:**
- (table): New merged table

**Example:**
```lua
local table_utils = require("table")

local config = {debug = true, timeout = 5}
local defaults = {retries = 3, timeout = 10}

local merged = table_utils.merge(defaults, config)
-- Result: {debug = true, timeout = 5, retries = 3}
```

### Complete Example

```lua
local table_utils = require("table")

local users = {
    alice = {age = 30, city = "NYC"},
    bob = {age = 25, city = "LA"},
    carol = {age = 28, city = "SF"},
}

print("User list:", table.concat(table_utils.keys(users), ", "))
print("Total users:", table_utils.length(users))

local metadata = {version = 1, updated = 2024}
local full_data = table_utils.merge(users, metadata)
```

---

## Error Handling

All built-in modules raise errors on failure. Use `pcall()` for error handling:

```lua
local fs = require("fs")

local ok, result = pcall(function()
    return fs.readFileSync("missing.txt")
end)

if not ok then
    print("Error:", result)
else
    print("Content:", result)
end
```

---

## Performance

All built-in modules are optimized for performance:

- **fs**: Efficient system calls, no buffering overhead
- **path**: String operations only, no filesystem access
- **events**: O(1) emitter dispatch
- **util**: Minimal overhead for type checking
- **table**: O(n) operations with efficient Lua implementation

---

## http Module

> 🆕 **New in v0.1.0** | Requires `http` feature flag

HTTP client for making web requests.

### Overview

The `http` module provides a comprehensive HTTP client for making web requests. It supports all standard HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS) and provides a modern fetch-style API.

### Load

```lua
local http = require("http")
```

### Requirements

Compile with the `http` feature flag:

```bash
cargo build --features http
```

Or use default features (HTTP included):

```bash
cargo build
```

### Quick Start

```lua
local http = require("http")

-- Simple GET request
local response = http.get("https://api.github.com/users/torvalds")

if response.ok() then
    local user = response.json()
    print("Name:", user.name)
    print("Location:", user.location)
end
```

### Main Functions

#### http.get(url)
Perform HTTP GET request.

#### http.post(url, options?)
Perform HTTP POST request with optional body and headers.

#### http.put(url, options?)
Perform HTTP PUT request.

#### http.delete(url, options?)
Perform HTTP DELETE request.

#### http.fetch(url, options?)
Universal fetch API (similar to JavaScript fetch).

#### http.postJson(url, data)
POST with automatic JSON serialization.

#### http.putJson(url, data)
PUT with automatic JSON serialization.

### Response Object

All HTTP methods return a Response object:

- `status` (number): HTTP status code
- `statusText` (string): Status text
- `headers` (table): Response headers
- `body` (string): Raw response body
- `ok()` → boolean: Check if status is 2xx
- `text()` → string: Get body as string
- `json()` → table: Parse body as JSON

### Examples

**POST JSON:**
```lua
local data = {name = "Alice", age = 30}
local response = http.postJson("https://api.example.com/users", data)
print("Created:", response.json().id)
```

**Custom headers:**
```lua
local response = http.fetch("https://api.example.com/protected", {
    method = "GET",
    headers = {
        ["Authorization"] = "Bearer token123",
        ["Accept"] = "application/json"
    }
})
```

**Timeout:**
```lua
local response = http.fetch("https://api.example.com/slow", {
    timeout = 5000  -- 5 seconds
})
```

### Complete Documentation

See [HTTP Module API Reference](./http-api.md) for complete documentation including:
- All HTTP methods
- Response handling
- Error handling
- Advanced examples
- Implementation details

---

**Last Updated**: October 2025  
**Built-in Modules**: 6  

See also: [require() API Reference](./require-api.md) | [Getting Started Guide](./getting-started.md) | [HTTP API Reference](./http-api.md)
