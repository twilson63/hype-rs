# process - Process and Environment Management

> **Access and control process information, working directory, environment variables, and command-line arguments.**

## Table of Contents
- [Import](#import)
- [Working Directory](#working-directory)
- [Environment Variables](#environment-variables)
- [Process Information](#process-information)
- [Process Control](#process-control)
- [Examples](#examples)

---

## Import

```lua
local process = require("process")
```

---

## Working Directory

### process.cwd()

Get current working directory.

**Returns:** `string` - Absolute path to current working directory

**Example:**
```lua
local process = require("process")

local cwd = process.cwd()
print("Current directory:", cwd)  -- "/Users/john/projects"

-- Resolve relative paths
local config_path = cwd .. "/config.json"
```

---

### process.chdir(path)

Change current working directory.

**Parameters:**
- `path: string` - New working directory path

**Returns:** `nil` (throws error if fails)

**Example:**
```lua
local process = require("process")

print("Before:", process.cwd())  -- "/Users/john"

-- Change directory
process.chdir("/tmp")
print("After:", process.cwd())   -- "/tmp"

-- Relative paths work
process.chdir("../")
print("Parent:", process.cwd())  -- "/"

-- Error handling
local ok, err = pcall(function()
    process.chdir("/nonexistent")
end)
if not ok then
    print("Error:", err)
end
```

---

## Environment Variables

### process.env

Table of environment variables (readable and writable).

**Type:** `table` - Key-value pairs with metatable for dynamic access

**Example:**
```lua
local process = require("process")

-- Read environment variables
print(process.env.HOME)      -- "/Users/john"
print(process.env.PATH)      -- "/usr/bin:/bin:..."
print(process.env.USER)      -- "john"

-- Set environment variable
process.env.MY_VAR = "my_value"
print(process.env.MY_VAR)    -- "my_value"

-- Delete environment variable
process.env.MY_VAR = nil
print(process.env.MY_VAR)    -- nil

-- Iterate all variables
for key, value in pairs(process.env) do
    print(key .. " = " .. value)
end
```

---

### process.getenv(key)

Get environment variable value.

**Parameters:**
- `key: string` - Environment variable name

**Returns:** `string|nil` - Value or `nil` if not set

**Example:**
```lua
local process = require("process")

-- Get variable
local home = process.getenv("HOME")
print(home)  -- "/Users/john"

-- Check if set
local debug = process.getenv("DEBUG")
if debug then
    print("Debug mode enabled")
end

-- Default value pattern
local port = process.getenv("PORT") or "3000"
local log_level = process.getenv("LOG_LEVEL") or "info"
```

---

### process.setenv(key, value)

Set environment variable.

**Parameters:**
- `key: string` - Environment variable name
- `value: string` - Value to set

**Returns:** `nil`

**Example:**
```lua
local process = require("process")

-- Set variable
process.setenv("API_KEY", "secret123")
print(process.getenv("API_KEY"))  -- "secret123"

-- Override existing
process.setenv("PATH", "/custom/bin:" .. process.getenv("PATH"))

-- Configuration from code
process.setenv("NODE_ENV", "production")
process.setenv("LOG_LEVEL", "debug")
```

---

## Process Information

### process.pid

Current process ID (read-only).

**Type:** `number` - Process ID

**Example:**
```lua
local process = require("process")

print("Process ID:", process.pid)  -- 12345

-- Use in logging
function log(message)
    print("[PID:" .. process.pid .. "] " .. message)
end

-- Create PID file
local fs = require("fs")
fs.writeFileSync("/var/run/myapp.pid", tostring(process.pid))
```

---

### process.platform

Operating system platform (read-only).

**Type:** `string` - Platform: `"linux"`, `"macos"`, `"windows"`

**Example:**
```lua
local process = require("process")

print("Platform:", process.platform)  -- "macos"

-- Platform-specific code
if process.platform == "windows" then
    print("Running on Windows")
    -- Windows-specific logic
elseif process.platform == "macos" then
    print("Running on macOS")
    -- macOS-specific logic
else
    print("Running on Linux")
    -- Linux-specific logic
end
```

---

### process.arch

CPU architecture (read-only).

**Type:** `string` - Architecture: `"x86_64"`, `"aarch64"`, `"arm"`, `"x86"`

**Example:**
```lua
local process = require("process")

print("Architecture:", process.arch)  -- "aarch64"

-- Architecture detection
if process.arch == "aarch64" then
    print("ARM64 (Apple Silicon or AWS Graviton)")
elseif process.arch == "x86_64" then
    print("x86-64 (Intel/AMD)")
end
```

---

### process.argv

Command-line arguments (read-only).

**Type:** `table` - Array of arguments (1-indexed)

**Example:**
```lua
local process = require("process")

-- Script: hype script.lua arg1 arg2 arg3
print(process.argv[0])  -- "hype" (executable)
print(process.argv[1])  -- "script.lua" (script path)
print(process.argv[2])  -- "arg1"
print(process.argv[3])  -- "arg2"

-- Parse arguments
for i, arg in ipairs(process.argv) do
    print("arg[" .. i .. "] = " .. arg)
end

-- Simple argument parsing
local function parse_args(argv)
    local args = {}
    for i = 2, #argv do  -- Skip executable and script name
        local arg = argv[i]
        if arg:sub(1, 2) == "--" then
            local key, value = arg:match("--([^=]+)=?(.*)")
            args[key] = value ~= "" and value or true
        end
    end
    return args
end

local args = parse_args(process.argv)
-- hype script.lua --port=3000 --debug
-- args = {port = "3000", debug = true}
```

---

## Process Control

### process.exit(code?)

Exit the process with optional exit code.

**Parameters:**
- `code?: number` - Exit code 0-255 (default: 0)

**Returns:** Does not return (exits process)

**Example:**
```lua
local process = require("process")

-- Normal exit
process.exit()      -- Exit with code 0

-- Error exit
process.exit(1)     -- Exit with code 1

-- Custom exit codes
if error_occurred then
    process.exit(1)   -- General error
elseif invalid_config then
    process.exit(2)   -- Config error
end

-- Exit with message
local function fatal(message, code)
    print("FATAL: " .. message)
    process.exit(code or 1)
end

fatal("Could not connect to database", 3)
```

---

## Examples

### Configuration from Environment

```lua
local process = require("process")

-- Load config from environment
local config = {
    port = tonumber(process.getenv("PORT")) or 3000,
    host = process.getenv("HOST") or "0.0.0.0",
    debug = process.getenv("DEBUG") == "true",
    api_key = process.getenv("API_KEY"),
    database_url = process.getenv("DATABASE_URL")
}

-- Validate required variables
if not config.api_key then
    print("Error: API_KEY environment variable required")
    process.exit(1)
end

print("Starting server on " .. config.host .. ":" .. config.port)
```

### Command-Line Argument Parser

```lua
local process = require("process")

function parse_cli_args()
    local args = {
        flags = {},
        options = {},
        positional = {}
    }
    
    for i = 2, #process.argv do  -- Skip executable and script
        local arg = process.argv[i]
        
        if arg:sub(1, 2) == "--" then
            -- Long option: --key=value or --flag
            local key, value = arg:match("^--([^=]+)=?(.*)$")
            if value and value ~= "" then
                args.options[key] = value
            else
                args.flags[key] = true
            end
        elseif arg:sub(1, 1) == "-" then
            -- Short flag: -f
            local flag = arg:sub(2)
            args.flags[flag] = true
        else
            -- Positional argument
            table.insert(args.positional, arg)
        end
    end
    
    return args
end

-- Usage: hype script.lua file.txt --verbose --output=result.txt
local args = parse_cli_args()
print("Flags:", args.flags.verbose)           -- true
print("Options:", args.options.output)        -- "result.txt"
print("Files:", args.positional[1])           -- "file.txt"
```

### Environment-Based Logging

```lua
local process = require("process")

local LOG_LEVELS = {
    debug = 1,
    info = 2,
    warn = 3,
    error = 4
}

local current_level = process.getenv("LOG_LEVEL") or "info"
local current_level_num = LOG_LEVELS[current_level] or LOG_LEVELS.info

function log(level, message)
    if LOG_LEVELS[level] >= current_level_num then
        local time = require("time")
        local timestamp = time.format(time.now(), "%Y-%m-%d %H:%M:%S")
        print(string.format("[%s] [%s] %s", 
            timestamp, level:upper(), message))
    end
end

-- Only logs if LOG_LEVEL=debug
log("debug", "Starting initialization")
log("info", "Server started")
log("error", "Connection failed")
```

### Working Directory Context

```lua
local process = require("process")

function with_directory(path, fn)
    local original = process.cwd()
    
    local ok, result = pcall(function()
        process.chdir(path)
        return fn()
    end)
    
    -- Restore original directory
    process.chdir(original)
    
    if not ok then
        error(result)
    end
    
    return result
end

-- Use temporary directory for operation
with_directory("/tmp", function()
    print("Working in:", process.cwd())
    -- Do work in /tmp
end)

print("Back to:", process.cwd())  -- Original directory
```

### Process Information Reporter

```lua
local process = require("process")
local json = require("json")

function get_process_info()
    return {
        pid = process.pid,
        platform = process.platform,
        arch = process.arch,
        cwd = process.cwd(),
        argv = process.argv,
        env = {
            user = process.getenv("USER"),
            home = process.getenv("HOME"),
            path = process.getenv("PATH")
        }
    }
end

print(json.encode(get_process_info(), true))
```

### Environment Validator

```lua
local process = require("process")

function validate_env(required)
    local missing = {}
    
    for _, key in ipairs(required) do
        if not process.getenv(key) then
            table.insert(missing, key)
        end
    end
    
    if #missing > 0 then
        print("Error: Missing required environment variables:")
        for _, key in ipairs(missing) do
            print("  - " .. key)
        end
        process.exit(1)
    end
end

-- Validate required variables before starting
validate_env({
    "DATABASE_URL",
    "API_KEY",
    "SECRET_KEY"
})

print("All required environment variables present")
```

---

## Performance Notes

- `process.cwd()` - Fast (< 1ms)
- `process.chdir()` - Fast (< 1ms)
- `process.env` - Direct access to environment (fast)
- `process.getenv/setenv` - Direct syscalls (very fast)
- `process.exit()` - Immediate (does not return)

---

## Platform Differences

| Property/Function | Linux | macOS | Windows |
|-------------------|-------|-------|---------|
| `process.cwd()` | ✅ | ✅ | ✅ |
| `process.chdir()` | ✅ | ✅ | ✅ |
| `process.env` | ✅ | ✅ | ✅ |
| `process.getenv()` | ✅ | ✅ | ✅ |
| `process.setenv()` | ✅ | ✅ | ✅ |
| `process.pid` | ✅ | ✅ | ✅ |
| `process.platform` | "linux" | "macos" | "windows" |
| `process.arch` | ✅ | ✅ | ✅ |
| `process.argv` | ✅ | ✅ | ✅ |
| `process.exit()` | ✅ | ✅ | ✅ |

---

## Common Environment Variables

**Unix/Linux/macOS:**
- `HOME` - User home directory
- `USER` - Current username
- `PATH` - Executable search paths
- `SHELL` - User's shell
- `PWD` - Current directory
- `TERM` - Terminal type

**Windows:**
- `USERPROFILE` - User home directory
- `USERNAME` - Current username
- `PATH` - Executable search paths
- `TEMP` - Temporary directory
- `APPDATA` - Application data directory

**Common (All platforms):**
- `NODE_ENV` - Node.js environment (development/production)
- `DEBUG` - Debug mode flag
- `PORT` - Server port
- `LOG_LEVEL` - Logging level

---

## Error Handling

```lua
local process = require("process")

-- chdir errors
local ok, err = pcall(function()
    process.chdir("/nonexistent")
end)
if not ok then
    print("Directory error:", err)
end

-- Exit code validation
local ok, err = pcall(function()
    process.exit(256)  -- Out of range
end)
-- Error: Exit code must be between 0 and 255

-- Missing environment variables return nil
local missing = process.getenv("DOES_NOT_EXIST")
-- missing == nil
```

---

## See Also

- [OS Module](./os.md) - System information and hardware stats
- [Examples](../../examples/process-demo.lua) - More examples
- [Tests](../../tests/process_module_test.rs) - Test suite

---

**Module**: process  
**Functions**: 5 + 4 properties  
**Status**: ✅ Production Ready  
**Last Updated**: October 27, 2025
