# fs - File System Operations

> **Synchronous file system operations for reading, writing, and managing files and directories.**

## Table of Contents
- [Import](#import)
- [Reading Files](#reading-files)
- [Writing Files](#writing-files)
- [File Information](#file-information)
- [Directory Operations](#directory-operations)
- [File Operations](#file-operations)
- [Examples](#examples)

---

## Import

```lua
local fs = require("fs")
```

**Note:** All operations are synchronous (blocking). Async support planned for future release.

---

## Reading Files

### fs.readFileSync(path)

Read entire file as UTF-8 text.

**Parameters:**
- `path: string` - Path to file

**Returns:** `string` - File contents as UTF-8 string

**Example:**
```lua
local fs = require("fs")

-- Read text file
local content = fs.readFileSync("config.json")
print(content)

-- Read and parse JSON
local json = require("json")
local config_text = fs.readFileSync("config.json")
local config = json.decode(config_text)

-- Handle errors
local ok, content = pcall(function()
    return fs.readFileSync("missing.txt")
end)
if not ok then
    print("Error reading file:", content)
end
```

---

## Writing Files

### fs.writeFileSync(path, data)

Write UTF-8 text to file (creates or overwrites).

**Parameters:**
- `path: string` - Path to file
- `data: string` - UTF-8 text content to write

**Returns:** `nil` (throws error if fails)

**Example:**
```lua
local fs = require("fs")

-- Write text file
fs.writeFileSync("output.txt", "Hello, World!")

-- Write JSON
local json = require("json")
local data = {name = "John", age = 30}
fs.writeFileSync("data.json", json.encode(data, true))

-- Overwrite existing file
fs.writeFileSync("log.txt", "New log entry\n")

-- Multi-line content
local content = [[
Line 1
Line 2
Line 3
]]
fs.writeFileSync("multiline.txt", content)
```

---

## File Information

### fs.existsSync(path)

Check if file or directory exists.

**Parameters:**
- `path: string` - Path to check

**Returns:** `boolean` - `true` if exists, `false` otherwise

**Example:**
```lua
local fs = require("fs")

-- Check file
if fs.existsSync("config.json") then
    print("Config found")
else
    print("Config missing")
end

-- Check directory
if fs.existsSync("/tmp") then
    print("Temp directory exists")
end

-- Conditional read
if fs.existsSync("optional.txt") then
    local content = fs.readFileSync("optional.txt")
end

-- Create if not exists
if not fs.existsSync("data.json") then
    fs.writeFileSync("data.json", "{}")
end
```

---

### fs.statSync(path)

Get file or directory metadata.

**Parameters:**
- `path: string` - Path to file or directory

**Returns:** `table` - Metadata:
  - `size: number` - Size in bytes
  - `isFile: boolean` - `true` if regular file
  - `isDirectory: boolean` - `true` if directory
  - `modified: number` - Last modified timestamp (seconds since epoch)

**Example:**
```lua
local fs = require("fs")

-- Get file info
local stat = fs.statSync("data.json")
print("Size:", stat.size .. " bytes")
print("Modified:", stat.modified)

-- Check type
if stat.isFile then
    print("Regular file")
elseif stat.isDirectory then
    print("Directory")
end

-- File size formatting
local function format_size(bytes)
    if bytes < 1024 then
        return bytes .. " B"
    elseif bytes < 1024 * 1024 then
        return string.format("%.2f KB", bytes / 1024)
    else
        return string.format("%.2f MB", bytes / (1024 * 1024))
    end
end

local stat = fs.statSync("large-file.bin")
print("Size:", format_size(stat.size))
```

---

## Directory Operations

### fs.readdirSync(path)

List directory contents (sorted alphabetically).

**Parameters:**
- `path: string` - Directory path

**Returns:** `table` - Array of filenames (strings, 1-indexed)

**Example:**
```lua
local fs = require("fs")

-- List files
local files = fs.readdirSync(".")
for _, file in ipairs(files) do
    print(file)
end

-- Filter by extension
local lua_files = {}
for _, file in ipairs(fs.readdirSync(".")) do
    if file:match("%.lua$") then
        table.insert(lua_files, file)
    end
end

-- Count files
local count = #fs.readdirSync("/tmp")
print("Files in /tmp:", count)

-- Check for specific file
local files = fs.readdirSync(".")
local has_config = false
for _, file in ipairs(files) do
    if file == "config.json" then
        has_config = true
        break
    end
end
```

---

### fs.mkdirSync(path)

Create directory (recursive by default).

**Parameters:**
- `path: string` - Directory path to create

**Returns:** `nil` (throws error if fails)

**Example:**
```lua
local fs = require("fs")

-- Create single directory
fs.mkdirSync("output")

-- Create nested directories (recursive)
fs.mkdirSync("data/backups/2025")

-- Safe create (idempotent)
if not fs.existsSync("logs") then
    fs.mkdirSync("logs")
end

-- Create with error handling
local ok, err = pcall(function()
    fs.mkdirSync("/root/protected")
end)
if not ok then
    print("Could not create directory:", err)
end
```

---

### fs.rmdirSync(path)

Remove empty directory.

**Parameters:**
- `path: string` - Directory path to remove

**Returns:** `nil` (throws error if fails or directory not empty)

**Example:**
```lua
local fs = require("fs")

-- Remove empty directory
fs.rmdirSync("empty-dir")

-- Must be empty (will error if not)
local ok, err = pcall(function()
    fs.rmdirSync("non-empty-dir")
end)
-- Error: Directory not empty

-- Safe remove
if fs.existsSync("temp-dir") then
    local ok, err = pcall(function()
        fs.rmdirSync("temp-dir")
    end)
    if not ok then
        print("Could not remove directory:", err)
    end
end
```

---

## File Operations

### fs.unlinkSync(path)

Delete file.

**Parameters:**
- `path: string` - File path to delete

**Returns:** `nil` (throws error if fails)

**Example:**
```lua
local fs = require("fs")

-- Delete file
fs.unlinkSync("temp.txt")

-- Safe delete
if fs.existsSync("old-file.txt") then
    fs.unlinkSync("old-file.txt")
end

-- Cleanup pattern
local temp_files = {"temp1.txt", "temp2.txt", "temp3.txt"}
for _, file in ipairs(temp_files) do
    if fs.existsSync(file) then
        fs.unlinkSync(file)
    end
end

-- Error handling
local ok, err = pcall(function()
    fs.unlinkSync("nonexistent.txt")
end)
-- Error: File not found
```

---

## Examples

### Read and Process File

```lua
local fs = require("fs")
local string = require("string")

-- Read and process text file
local function process_log(path)
    local content = fs.readFileSync(path)
    local lines = string.split(content, "\n")
    
    local errors = 0
    for _, line in ipairs(lines) do
        if string.contains(line, "ERROR") then
            errors = errors + 1
        end
    end
    
    return {
        total_lines = #lines,
        error_count = errors
    }
end

local stats = process_log("app.log")
print("Errors:", stats.error_count)
```

### Configuration Management

```lua
local fs = require("fs")
local json = require("json")

-- Load config with defaults
function load_config(path, defaults)
    if fs.existsSync(path) then
        local content = fs.readFileSync(path)
        return json.decode(content)
    else
        return defaults
    end
end

-- Save config
function save_config(path, config)
    local content = json.encode(config, true)
    fs.writeFileSync(path, content)
end

-- Usage
local config = load_config("config.json", {
    port = 3000,
    debug = false
})

config.port = 8080
save_config("config.json", config)
```

### Directory Tree Walker

```lua
local fs = require("fs")

function walk_directory(path, callback)
    local files = fs.readdirSync(path)
    
    for _, file in ipairs(files) do
        local full_path = path .. "/" .. file
        local stat = fs.statSync(full_path)
        
        callback(full_path, stat)
        
        if stat.isDirectory then
            walk_directory(full_path, callback)
        end
    end
end

-- Find all Lua files
walk_directory(".", function(path, stat)
    if stat.isFile and path:match("%.lua$") then
        print("Lua file:", path)
    end
end)
```

### File Backup

```lua
local fs = require("fs")
local time = require("time")

function backup_file(path)
    if not fs.existsSync(path) then
        error("File not found: " .. path)
    end
    
    local timestamp = time.format(time.now(), "%Y%m%d_%H%M%S")
    local backup_path = path .. "." .. timestamp .. ".bak"
    
    local content = fs.readFileSync(path)
    fs.writeFileSync(backup_path, content)
    
    return backup_path
end

-- Usage
local backup = backup_file("important.txt")
print("Backup created:", backup)
-- important.txt.20251027_143045.bak
```

### Cleanup Old Files

```lua
local fs = require("fs")
local time = require("time")

function cleanup_old_files(dir, max_age_seconds)
    local now = time.nowSeconds()
    local removed = 0
    
    for _, file in ipairs(fs.readdirSync(dir)) do
        local path = dir .. "/" .. file
        local stat = fs.statSync(path)
        
        if stat.isFile then
            local age = now - stat.modified
            if age > max_age_seconds then
                fs.unlinkSync(path)
                removed = removed + 1
            end
        end
    end
    
    return removed
end

-- Remove files older than 7 days
local count = cleanup_old_files("./logs", 7 * 24 * 60 * 60)
print("Removed " .. count .. " old files")
```

### Ensure Directory Structure

```lua
local fs = require("fs")

function ensure_directories(paths)
    for _, path in ipairs(paths) do
        if not fs.existsSync(path) then
            fs.mkdirSync(path)
            print("Created:", path)
        end
    end
end

-- Setup application directories
ensure_directories({
    "data",
    "data/cache",
    "data/logs",
    "data/backups",
    "config"
})
```

### File Size Reporter

```lua
local fs = require("fs")

function get_directory_size(path)
    local total = 0
    
    for _, file in ipairs(fs.readdirSync(path)) do
        local full_path = path .. "/" .. file
        local stat = fs.statSync(full_path)
        
        if stat.isFile then
            total = total + stat.size
        elseif stat.isDirectory then
            total = total + get_directory_size(full_path)
        end
    end
    
    return total
end

function format_bytes(bytes)
    if bytes < 1024 then
        return bytes .. " B"
    elseif bytes < 1024 * 1024 then
        return string.format("%.2f KB", bytes / 1024)
    elseif bytes < 1024 * 1024 * 1024 then
        return string.format("%.2f MB", bytes / (1024 * 1024))
    else
        return string.format("%.2f GB", bytes / (1024 * 1024 * 1024))
    end
end

local size = get_directory_size(".")
print("Total size:", format_bytes(size))
```

---

## Performance Notes

- All operations are synchronous (blocking)
- File I/O is buffered by the OS
- Large files may take time to read entirely
- Directory operations are fast for small directories
- Use streaming for large files (planned feature)

---

## Limitations

- **UTF-8 text only**: Binary files not supported (planned for Phase 2)
- **Synchronous only**: No async/await support yet
- **No streaming**: Entire file read into memory
- **No symlink support**: Symbolic links not handled
- **No permissions API**: Cannot set file permissions
- **No file watching**: File system events not supported yet
- **Recursive delete**: Must manually delete directory contents first

---

## Error Handling

```lua
local fs = require("fs")

-- File not found
local ok, err = pcall(function()
    return fs.readFileSync("missing.txt")
end)
-- Error: File not found

-- Permission denied
local ok, err = pcall(function()
    fs.writeFileSync("/root/protected.txt", "data")
end)
-- Error: Permission denied

-- Directory not empty
local ok, err = pcall(function()
    fs.rmdirSync("non-empty-dir")
end)
-- Error: Directory not empty

-- Invalid path
local ok, err = pcall(function()
    fs.readFileSync("")
end)
-- Error: Invalid path
```

---

## Cross-Platform Path Handling

```lua
local fs = require("fs")
local process = require("process")

-- Use forward slashes (works on all platforms)
fs.readFileSync("data/config.json")  -- ✅ Works on Windows, macOS, Linux

-- Platform-specific separator
local sep = process.platform == "windows" and "\\" or "/"
local path = "data" .. sep .. "config.json"

-- Build paths safely
function join_path(...)
    local sep = process.platform == "windows" and "\\" or "/"
    local parts = {...}
    return table.concat(parts, sep)
end

local config_path = join_path("data", "config", "app.json")
```

---

## Common Patterns

**File exists or create:**
```lua
if not fs.existsSync("data.json") then
    fs.writeFileSync("data.json", "{}")
end
```

**Read, modify, write:**
```lua
local content = fs.readFileSync("data.txt")
content = content:gsub("old", "new")
fs.writeFileSync("data.txt", content)
```

**Atomic write (backup first):**
```lua
if fs.existsSync("important.txt") then
    local backup = fs.readFileSync("important.txt")
    fs.writeFileSync("important.txt.bak", backup)
end
fs.writeFileSync("important.txt", new_content)
```

**Directory iteration:**
```lua
for _, file in ipairs(fs.readdirSync(".")) do
    print(file)
end
```

---

## Comparison with Node.js

| Node.js | Hype-RS | Notes |
|---------|---------|-------|
| `fs.readFileSync()` | `fs.readFileSync()` | UTF-8 only |
| `fs.writeFileSync()` | `fs.writeFileSync()` | UTF-8 only |
| `fs.existsSync()` | `fs.existsSync()` | ✅ Same |
| `fs.statSync()` | `fs.statSync()` | Subset of fields |
| `fs.readdirSync()` | `fs.readdirSync()` | Sorted output |
| `fs.unlinkSync()` | `fs.unlinkSync()` | ✅ Same |
| `fs.mkdirSync()` | `fs.mkdirSync()` | Recursive by default |
| `fs.rmdirSync()` | `fs.rmdirSync()` | Empty only |
| `fs.copyFileSync()` | ❌ | Not yet |
| `fs.renameSync()` | ❌ | Not yet |

---

## See Also

- [Examples](../../examples/file-operations-new.lua) - More examples
- [Tests](../../tests/fs_operations_test.rs) - Test suite
- [Module System](../modules/getting-started.md) - Module loading

---

**Module**: fs  
**Functions**: 8  
**Status**: ✅ Production Ready (UTF-8 only)  
**Last Updated**: October 27, 2025
