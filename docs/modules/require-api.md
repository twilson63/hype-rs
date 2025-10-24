# require() API Reference

Complete API documentation for the `require()` function and related interfaces in the Hype-RS module system.

## Table of Contents

- [require(module_id)](#requiremodule_id)
- [require.cache](#requirecache)
- [require.resolve(module_id)](#requireresolvemodule_id)
- [module.exports](#moduleexports)
- [__dirname and __filename](#__dirname-and-__filename)
- [Error Handling](#error-handling)
- [Examples](#examples)

---

## require(module_id)

Load and return a module.

### Signature
```lua
exports = require(module_id: string)
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `module_id` | string | Name, path, or identifier of the module to load |

### Returns

| Return Type | Description |
|------------|-------------|
| any | The value of `module.exports` from the loaded module |

### Module ID Formats

The `module_id` parameter can take several forms:

```lua
-- Built-in module
local fs = require("fs")

-- Node modules directory
local math_lib = require("math-lib")

-- Relative path
local util = require("./lib/util")

-- Parent directory path
local helper = require("../shared/helper")

-- Absolute path (not recommended, breaks portability)
local config = require("/absolute/path/to/module")
```

### Module Resolution Order

When you call `require("module-name")`, the system searches in this order:

1. **Require Cache**: Check if already loaded
2. **Built-in Modules**: fs, path, events, util, table
3. **hype_modules**: Search directories walking up from current directory
   - `./hype_modules/module-name/`
   - `../hype_modules/module-name/`
   - `../../hype_modules/module-name/` (and so on)
4. **Home Modules**: `~/.hype/modules/module-name/`

### Behavior

- **First Load**: Module code is executed, result is cached
- **Subsequent Loads**: Cached version is returned instantly
- **Performance**: Cached loads complete in < 1ms
- **Isolation**: Each module has its own environment
- **Error**: Throws error if module not found

### Examples

**Load built-in module:**
```lua
local fs = require("fs")
print(type(fs))  -- "table"
```

**Load and use a module:**
```lua
local math_lib = require("math-lib")
local result = math_lib.add(5, 3)
print(result)  -- 8
```

**Load multiple modules:**
```lua
local fs = require("fs")
local path = require("path")
local events = require("events")
```

**Error handling:**
```lua
local ok, module = pcall(function()
    return require("my-module")
end)

if ok then
    print("Module loaded successfully")
else
    print("Error:", module)
end
```

### Error Cases

| Error | Description | Solution |
|-------|-------------|----------|
| `MODULE_NOT_FOUND` | Module doesn't exist in any search path | Check module name spelling, verify installation |
| `CIRCULAR_DEPENDENCY` | Module A requires Module B which requires Module A | Restructure modules to avoid cycle |
| `EXECUTION_ERROR` | Error occurred while executing module code | Check module code for syntax/runtime errors |

---

## require.cache

A table containing all currently loaded modules.

### Signature
```lua
cache_table = require.cache
```

### Type
```lua
type(require.cache) == "table"
```

### Description

The `require.cache` table stores all modules that have been loaded. Keys are module identifiers, values are the module exports. This is useful for:

- **Debugging**: Inspect loaded modules
- **Introspection**: Check module status
- **Testing**: Clear or modify cached modules
- **Performance**: Understand caching behavior

### Accessing Cache Entries

```lua
-- Iterate over all cached modules
for module_id, module_exports in pairs(require.cache) do
    print("Module:", module_id)
    print("Type:", type(module_exports))
end
```

### Cache Structure

Each cached module entry contains:

| Field | Type | Description |
|-------|------|-------------|
| `__id` | string | Module identifier/name |
| `__filename` | string | Full path to module file |
| `__dirname` | string | Directory containing module |
| (other fields) | any | Module-specific exports |

### Examples

**List all loaded modules:**
```lua
local fs = require("fs")
local path = require("path")

print("Loaded modules:")
for module_id in pairs(require.cache) do
    print("  -", module_id)
end

-- Output:
-- Loaded modules:
--   - fs
--   - path
```

**Check if module is cached:**
```lua
if require.cache["fs"] then
    print("fs module is already loaded")
else
    print("fs module not yet loaded")
end
```

**Inspect module metadata:**
```lua
local fs = require("fs")
local fs_module = require.cache["fs"]

if fs_module.__id then
    print("Module ID:", fs_module.__id)
end

if fs_module.__filename then
    print("File path:", fs_module.__filename)
end
```

**Get cache size:**
```lua
local count = 0
for _ in pairs(require.cache) do
    count = count + 1
end
print("Cached modules:", count)
```

### Performance Characteristics

- **Lookup**: O(1) constant time
- **Storage**: Minimal overhead per module
- **Memory**: Typically < 1KB per module entry
- **Persistence**: Cache persists for script lifetime

### Notes

[!NOTE]
Cache entries are read-only from Lua. Do not modify cache values directly; instead, re-require the module if it needs to be reloaded.

---

## require.resolve(module_id)

Resolve a module ID to its full file path without loading it.

### Signature
```lua
path_string = require.resolve(module_id: string)
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `module_id` | string | Name or identifier of module to resolve |

### Returns

| Return Type | Description |
|------------|-------------|
| string | Absolute file path to the module |

### Description

The `resolve()` function uses the same resolution algorithm as `require()` but returns the path instead of loading the module. It's useful for:

- **Path Operations**: Get module location for path manipulation
- **File Access**: Read module metadata without loading
- **Debugging**: Understand where modules are located

### Behavior

- Does NOT execute module code
- Does NOT cache anything
- Returns absolute paths
- Throws error if module not found

### Examples

**Resolve built-in module:**
```lua
local fs_path = require.resolve("fs")
print(fs_path)
-- Output: /usr/local/share/hype/modules/fs.lua
```

**Resolve module from hype_modules:**
```lua
local util_path = require.resolve("math-lib")
print(util_path)
-- Output: /home/user/project/hype_modules/math-lib/index.lua
```

**Use with path operations:**
```lua
local fs = require("fs")
local path = require("path")

-- Find where fs module is located
local fs_location = require.resolve("fs")

-- Get directory containing it
local dir = path.dirname(fs_location)
print("Module directory:", dir)
```

**Check module without loading:**
```lua
-- Check if module exists by trying to resolve
local ok, path = pcall(function()
    return require.resolve("optional-dependency")
end)

if ok then
    print("Module found at:", path)
    -- Now decide if you want to load it
    -- local mod = require("optional-dependency")
else
    print("Module not found")
end
```

**Error handling:**
```lua
-- Try to resolve non-existent module
local ok, result = pcall(function()
    return require.resolve("nonexistent-module")
end)

if not ok then
    print("Resolve error:", result)
end
```

### Resolution Order

`require.resolve()` uses the same search order as `require()`:

1. Built-in modules (fs, path, events, util, table)
2. hype_modules directories (walking up)
3. ~/.hype/modules/ directory
4. Error if not found

### Error Cases

| Error | Description |
|-------|-------------|
| `MODULE_NOT_FOUND` | Module cannot be resolved |

---

## module.exports

Define what a module exposes to other modules.

### Signature
```lua
module.exports = value
```

### Type
Any Lua value: table, function, string, number, etc.

### Description

`module.exports` is how modules define their public interface. Whatever you assign to `module.exports` becomes the value returned by `require()`.

### Defining Exports

**Table of functions:**
```lua
-- In math-lib.lua
module.exports = {
    add = function(a, b) return a + b end,
    subtract = function(a, b) return a - b end,
    multiply = function(a, b) return a * b end,
}

-- In main.lua
local math = require("math-lib")
print(math.add(5, 3))  -- 8
```

**Single function:**
```lua
-- In greeter.lua
module.exports = function(name)
    return "Hello, " .. name
end

-- In main.lua
local greet = require("greeter")
print(greet("World"))  -- Hello, World
```

**Table with mixed content:**
```lua
-- In config.lua
module.exports = {
    version = "1.0.0",
    debug = true,
    settings = {
        timeout = 5000,
        retries = 3,
    },
    initialize = function()
        print("Initialized")
    end,
}

-- In main.lua
local config = require("config")
print(config.version)  -- 1.0.0
config.initialize()    -- Initialized
```

**Empty exports:**
```lua
-- In side-effects.lua
print("Running setup")
module.exports = {}  -- Nothing to export

-- In main.lua
require("side-effects")  -- Runs print statement
```

### Patterns

**Library pattern:**
```lua
-- lib/utils.lua
local function private_helper()
    return "internal"
end

module.exports = {
    public_func = function()
        return private_helper()
    end,
}
```

**Class/Constructor pattern:**
```lua
-- models/user.lua
local User = {}

function User:new(name, email)
    local obj = {name = name, email = email}
    return setmetatable(obj, {__index = User})
end

function User:save()
    print("Saving user:", self.name)
end

module.exports = User

-- In main.lua
local User = require("models/user")
local user = User:new("Alice", "alice@example.com")
user:save()
```

**Singleton pattern:**
```lua
-- services/logger.lua
local Logger = {}

function Logger:log(msg)
    print("[LOG]", msg)
end

module.exports = Logger

-- In main.lua
local logger = require("services/logger")
logger:log("Hello")
```

### Important Notes

[!NOTE]
`module.exports` must be explicitly set. Lua doesn't automatically return the last statement in a file.

```lua
-- This does NOT work:
function myFunc() end
-- myFunc is not exported!

-- This works:
module.exports = myFunc
```

[!WARNING]
Do not modify `module.exports` after other modules have required this module. Changes won't be reflected in already-loaded modules due to caching.

### Best Practices

1. **Set exports once**: Set `module.exports` once at the end of the module
2. **Clear interface**: Explicitly define what's public
3. **Group related functions**: Organize exports logically
4. **Use tables for multiple exports**: Table is the standard for multiple values
5. **Document exports**: Add comments explaining what each export does

---

## __dirname and __filename

Global variables available in every module for path information.

### __dirname

The absolute path to the directory containing the current module file.

### Signature
```lua
dir_string = __dirname
```

### Type
```lua
type(__dirname) == "string"
```

### Example
```lua
-- In /home/user/project/lib/utils.lua
print(__dirname)
-- Output: /home/user/project/lib
```

### __filename

The absolute path to the current module file.

### Signature
```lua
file_string = __filename
```

### Type
```lua
type(__filename) == "string"
```

### Example
```lua
-- In /home/user/project/lib/utils.lua
print(__filename)
-- Output: /home/user/project/lib/utils.lua
```

### Use Cases

**Load sibling files:**
```lua
-- In lib/utils.lua
local path = require("path")
local fs = require("fs")

-- Load a sibling file
local config_path = path.join(__dirname, "config.lua")
local config_content = fs.readFileSync(config_path)
```

**Create log files in module directory:**
```lua
local fs = require("fs")

local log_file = __dirname .. "/debug.log"
fs.writeFileSync(log_file, "Debug output")
```

**Resolve relative paths from module:**
```lua
local path = require("path")

-- Convert relative path to absolute, relative to this module
local data_file = path.resolve(__dirname, "../data/input.txt")
```

**Module self-awareness:**
```lua
print("This module is:", __filename)
print("Located in:", __dirname)
```

### Platform Notes

**Path Separators:**
- **Linux/macOS**: `/` (forward slash)
- **Windows**: `\` (backslash)

Hype-RS automatically normalizes paths, but be aware of these differences if doing string manipulation.

**Absolute vs Relative:**
- `__dirname` and `__filename` are always absolute paths
- Use with `path.resolve()` for portable path operations
- Prefer `path.join(__dirname, "file")` over string concatenation

### Examples

**Create absolute path safely (cross-platform):**
```lua
local path = require("path")

-- Good: Works on all platforms
local config = path.join(__dirname, "config.json")

-- Avoid: Platform-specific
local config = __dirname .. "/config.json"  -- Fails on Windows
```

**Debug module location:**
```lua
function debug_info()
    print("Module file:", __filename)
    print("Module directory:", __dirname)
end
```

**Dynamic requires:**
```lua
local path = require("path")
local fs = require("fs")

-- Load a sibling module dynamically
local sibling_path = path.join(__dirname, "sibling.lua")
if fs.existsSync(sibling_path) then
    local sibling = require(sibling_path)
end
```

---

## Error Handling

### Module Loading Errors

Errors during `require()` are thrown as Lua exceptions. Use `pcall()` to handle them.

```lua
local ok, result = pcall(function()
    return require("some-module")
end)

if not ok then
    print("Failed to load module:", result)
    -- result contains error message
else
    print("Module loaded:", result)
    -- result contains module exports
end
```

### Common Errors

**Module not found:**
```
Failed to load module 'nonexistent': Unknown built-in module: nonexistent
```

**Circular dependency:**
```
Failed to load module 'a': Circular dependency detected: a -> b -> a
```

**Syntax error in module:**
```
Failed to load module 'broken': [string "broken"]:5: unexpected symbol
```

**Execution error in module:**
```
Failed to load module 'broken': runtime error: attempt to call nil
```

### Error Messages

Hype-RS provides detailed error messages including:
- Module name that failed to load
- Type of error (MODULE_NOT_FOUND, CIRCULAR_DEPENDENCY, etc.)
- Context information (dependency chain, line numbers)

### Debugging Failed Loads

1. Check module name spelling
2. Verify module file exists
3. Check for syntax errors in module file
4. Look for circular dependencies
5. Use `require.resolve()` to verify path resolution
6. Check permissions on module files

---

## Examples

### Basic Module Usage

```lua
-- math-lib.lua
module.exports = {
    add = function(a, b) return a + b end,
    sub = function(a, b) return a - b end,
}

-- main.lua
local math = require("math-lib")
print(math.add(5, 3))  -- 8
```

### Module Introspection

```lua
local fs = require("fs")
local cache = require.cache

print("Loaded modules:")
for name, _ in pairs(cache) do
    print(" -", name)
end

-- Resolve a module
local fs_path = require.resolve("fs")
print("fs is located at:", fs_path)
```

### Conditional Loading

```lua
local ok, debug = pcall(function()
    return require("debug-lib")
end)

if ok then
    debug.enable()
else
    print("Debug library not available")
end
```

### Module with Dependencies

```lua
-- services/email.lua
local fs = require("fs")
local util = require("util")

module.exports = {
    send = function(to, subject)
        -- Implementation uses fs and util
    end,
}

-- main.lua
local email = require("services/email")
email.send("user@example.com", "Hello")
```

### Path Operations in Module

```lua
-- In lib/data-loader.lua
local fs = require("fs")
local path = require("path")

local function load_data()
    local data_file = path.join(__dirname, "../data/input.json")
    return fs.readFileSync(data_file)
end

module.exports = {
    load = load_data,
}
```

---

## Performance Tips

### Module Caching

Modules are cached automatically. Subsequent calls to `require()` return the cached result:

```lua
local fs1 = require("fs")  -- ~50ms (first load)
local fs2 = require("fs")  -- <1ms (from cache)
assert(fs1 == fs2)         -- Same object
```

### Avoid Circular Dependencies

Circular dependencies are detected but hurt performance:

```lua
-- bad.lua
local bad_b = require("bad_b")

-- bad_b.lua
local bad_a = require("bad_a")  -- Detected and errors
```

Instead, restructure your modules:

```lua
-- shared.lua (no dependencies on other custom modules)
module.exports = { /* shared code */ }

-- a.lua
local shared = require("shared")

-- b.lua
local shared = require("shared")
```

### Check with resolve() First

If loading a module is expensive and might not exist:

```lua
local ok, path = pcall(function()
    return require.resolve("optional-dep")
end)

if ok then
    local optional = require("optional-dep")
end
```

---

**Last Updated**: October 2025  
**API Version**: 1.0  

See also: [Getting Started Guide](./getting-started.md) | [Built-in Modules](./builtin-modules.md)
