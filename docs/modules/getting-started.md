# Getting Started with Modules

A practical guide to using the Hype-RS module system. This tutorial walks you through creating and using modules.

## Table of Contents

- [What You'll Learn](#what-youll-learn)
- [Prerequisites](#prerequisites)
- [Your First Module](#your-first-module)
- [Creating Custom Modules](#creating-custom-modules)
- [Using Built-in Modules](#using-built-in-modules)
- [Common Patterns](#common-patterns)
- [Troubleshooting](#troubleshooting)
- [Next Steps](#next-steps)

---

## What You'll Learn

In this guide, you'll:

✅ Create and load your first module  
✅ Understand `module.exports` and `require()`  
✅ Use built-in modules effectively  
✅ Organize code with modules  
✅ Handle common errors  

**Time**: ~15 minutes  
**Difficulty**: Beginner  
**Prerequisites**: Basic Lua knowledge

---

## Prerequisites

You should have:

- Hype-RS installed
- A text editor
- Basic Lua knowledge
- Terminal/command line access

Check your installation:

```bash
hype --version
```

---

## Your First Module

### Step 1: Create a Simple Module

Create a file called `hello.lua`:

```lua
module.exports = function(name)
    return "Hello, " .. name .. "!"
end
```

### Step 2: Use the Module

Create another file called `main.lua` in the same directory:

```lua
local hello = require("hello")
print(hello("World"))
```

### Step 3: Run It

```bash
$ hype main.lua
Hello, World!
```

**Congratulations!** You've created and used your first module.

---

## Understanding require()

### How require() Works

When you call `require("module-name")`:

1. Hype-RS checks if module is already loaded (cached)
2. If not cached, it searches for the module file
3. It executes the module code
4. It returns `module.exports`

### Module Resolution

`require()` searches in this order:

```
1. require.cache (already loaded?)
2. Built-in modules (fs, path, events, util, table)
3. ./hype_modules/module-name/
4. ../hype_modules/module-name/
5. ~/.hype/modules/module-name/
```

### Caching Behavior

Once a module is loaded, it's cached:

```lua
local fs1 = require("fs")  -- Load and cache
local fs2 = require("fs")  -- Return cached version

print(fs1 == fs2)  -- true (same object!)
```

This is why module initialization happens only once.

---

## Creating Custom Modules

### Module 1: Math Library

Create `math_lib.lua`:

```lua
module.exports = {
    add = function(a, b)
        return a + b
    end,
    
    subtract = function(a, b)
        return a - b
    end,
    
    multiply = function(a, b)
        return a * b
    end,
    
    divide = function(a, b)
        if b == 0 then
            error("Division by zero")
        end
        return a / b
    end,
}
```

Use it in `calculator.lua`:

```lua
local math_lib = require("math_lib")

print("5 + 3 =", math_lib.add(5, 3))
print("5 - 3 =", math_lib.subtract(5, 3))
print("5 * 3 =", math_lib.multiply(5, 3))
print("5 / 3 =", math_lib.divide(5, 3))
```

Run it:

```bash
$ hype calculator.lua
5 + 3 = 8
5 - 3 = 2
5 * 3 = 15
5 / 3 = 1.6666666666667
```

### Module 2: String Utilities

Create `string_utils.lua`:

```lua
module.exports = {
    capitalize = function(str)
        return str:sub(1, 1):upper() .. str:sub(2):lower()
    end,
    
    reverse = function(str)
        return str:reverse()
    end,
    
    repeat = function(str, count)
        local result = ""
        for i = 1, count do
            result = result .. str
        end
        return result
    end,
    
    contains = function(str, substring)
        return str:find(substring) ~= nil
    end,
}
```

Use it:

```lua
local str_utils = require("string_utils")

print(str_utils.capitalize("hello"))        -- "Hello"
print(str_utils.reverse("hello"))           -- "olleh"
print(str_utils.repeat("ha", 3))            -- "hahaha"
print(str_utils.contains("hello", "ell"))   -- true
```

### Module 3: Configuration

Create `config.lua`:

```lua
module.exports = {
    app_name = "MyApp",
    version = "1.0.0",
    debug = false,
    
    database = {
        host = "localhost",
        port = 5432,
        name = "myapp_db",
    },
    
    server = {
        port = 8080,
        host = "0.0.0.0",
        timeout = 30,
    },
}
```

Use it:

```lua
local config = require("config")

print("App:", config.app_name, config.version)
print("Debug mode:", config.debug)
print("Database:", config.database.host .. ":" .. config.database.port)
```

---

## Using Built-in Modules

### File System Module

The `fs` module provides file operations:

```lua
local fs = require("fs")

-- Write a file
fs.writeFileSync("data.txt", "Hello, World!")

-- Read it back
local content = fs.readFileSync("data.txt")
print("File contents:", content)

-- Check if file exists
if fs.existsSync("data.txt") then
    print("File exists!")
end

-- Get file info
local stat = fs.statSync("data.txt")
print("File size:", stat.size, "bytes")

-- List directory
local files = fs.readdirSync(".")
print("Files in current directory:")
for _, name in ipairs(files) do
    print("  -", name)
end
```

### Path Module

The `path` module helps with file paths:

```lua
local path = require("path")

-- Join paths safely (cross-platform)
local filepath = path.join("data", "users", "alice.json")
print("Path:", filepath)

-- Extract parts
print("Directory:", path.dirname(filepath))
print("Filename:", path.basename(filepath))
print("Extension:", path.extname(filepath))

-- Resolve to absolute path
local absolute = path.resolve(filepath)
print("Absolute:", absolute)
```

### Combining fs and path

A practical example combining both modules:

```lua
local fs = require("fs")
local path = require("path")

-- Create a directory structure
local dirs = {"data", "output", "logs"}
for _, dir in ipairs(dirs) do
    if not fs.existsSync(dir) then
        fs.mkdirSync(dir)
        print("Created directory:", dir)
    end
end

-- Write files to data directory
for i = 1, 3 do
    local filepath = path.join("data", "file_" .. i .. ".txt")
    fs.writeFileSync(filepath, "Content " .. i)
    print("Created:", filepath)
end

-- List all files
print("\nDirectory contents:")
local files = fs.readdirSync("data")
for _, name in ipairs(files) do
    local full_path = path.join("data", name)
    local stat = fs.statSync(full_path)
    print(string.format("  %s (%d bytes)", name, stat.size))
end
```

### Events Module

Create an event system:

```lua
local events = require("events")

-- Create event bus
local bus = events.EventEmitter:new()

-- Register event handlers
bus:on("user_login", function(username)
    print("User logged in:", username)
end)

bus:on("user_logout", function(username)
    print("User logged out:", username)
end)

bus:on("error", function(message)
    print("ERROR:", message)
end)

-- Emit events
bus:emit("user_login", "alice")
bus:emit("user_login", "bob")
bus:emit("error", "Something went wrong!")
bus:emit("user_logout", "alice")
```

---

## Common Patterns

### Pattern 1: Module Initialization

```lua
-- logger.lua
local initialized = false

local function init()
    print("Logger initialized")
    initialized = true
end

module.exports = {
    init = init,
    
    log = function(msg)
        if not initialized then
            init()
        end
        print("[LOG]", msg)
    end,
}
```

### Pattern 2: Private Functions

```lua
-- user_service.lua
local function hash_password(password)
    -- Private function
    return password .. "_hashed"
end

module.exports = {
    register = function(username, password)
        -- Use private function
        local hashed = hash_password(password)
        print("User registered:", username)
        return true
    end,
    
    -- Public function
    authenticate = function(username, password)
        local hashed = hash_password(password)
        print("Authenticating:", username)
        return true
    end,
}
```

### Pattern 3: Dependency Injection

```lua
-- Create a service that depends on logger
-- service.lua
module.exports = function(logger)
    return {
        do_work = function()
            logger:log("Starting work")
            -- Do something
            logger:log("Work complete")
        end,
    }
end

-- main.lua
local logger = require("logger")
local service = require("service")(logger)
service.do_work()
```

### Pattern 4: Singleton

```lua
-- database.lua
local db = nil

module.exports = {
    get_connection = function()
        if db == nil then
            print("Connecting to database...")
            db = {connected = true}
        end
        return db
    end,
}

-- Usage (connection is created once)
local db1 = require("database").get_connection()
local db2 = require("database").get_connection()
print(db1 == db2)  -- true (same connection)
```

---

## Working with __dirname and __filename

These globals let modules know their own location:

```lua
-- lib/utils.lua
print("This file:", __filename)
print("This directory:", __dirname)

-- Load a sibling file
local path = require("path")
local fs = require("fs")

local sibling = path.join(__dirname, "helpers.lua")
if fs.existsSync(sibling) then
    -- Could load it here
end
```

### Practical Example: Configuration Loading

```lua
-- app/services/database.lua
local fs = require("fs")
local path = require("path")

-- Load config from same directory
local config_path = path.join(__dirname, "config.json")

local function load_config()
    if fs.existsSync(config_path) then
        local content = fs.readFileSync(config_path)
        -- Parse JSON (for now just return content)
        return content
    else
        error("Config not found at " .. config_path)
    end
end

module.exports = {
    connect = function()
        local config = load_config()
        print("Connecting with config:", config)
    end,
}
```

---

## Troubleshooting

### Error: "Unknown built-in module"

**Problem**: Module name is misspelled or doesn't exist.

```lua
local math = require("math")  -- Error! No "math" module
```

**Solution**: Use available built-in modules (fs, path, events, util, table) or load custom modules by correct path.

```lua
local fs = require("fs")  -- Correct
local math_lib = require("math_lib")  -- Custom module
```

### Error: "Circular dependency detected"

**Problem**: Module A requires Module B, and Module B requires Module A.

```lua
-- a.lua
local b = require("b")

-- b.lua
local a = require("a")  -- Error!
```

**Solution**: Restructure to avoid cycle. Use a third module:

```lua
-- shared.lua
module.exports = {shared_data = "value"}

-- a.lua
local shared = require("shared")

-- b.lua
local shared = require("shared")
```

### Error: "Failed to read file"

**Problem**: File doesn't exist or no read permission.

```lua
local fs = require("fs")
local content = fs.readFileSync("missing.txt")  -- Error!
```

**Solution**: Check file exists first.

```lua
local fs = require("fs")

if fs.existsSync("file.txt") then
    local content = fs.readFileSync("file.txt")
else
    print("File not found")
end
```

### Error: "Cannot write to file"

**Problem**: No write permission or parent directory doesn't exist.

**Solution**: Create directories first.

```lua
local fs = require("fs")
local path = require("path")

local filepath = path.join("output", "data.txt")
local dir = path.dirname(filepath)

if not fs.existsSync(dir) then
    fs.mkdirSync(dir)
end

fs.writeFileSync(filepath, "content")
```

### Module Not Found Error

**Problem**: `require()` can't find your module.

**Checklist**:
1. ✓ Module file exists in current directory
2. ✓ File extension is `.lua`
3. ✓ Module name matches file name (case-sensitive)
4. ✓ No `.lua` extension in require() call
5. ✓ Path separators correct for your OS

**Debug**: Use `require.resolve()` to check where it's looking:

```lua
local path = require.resolve("my_module")
print("Found at:", path)
```

### Module.exports Not Working

**Problem**: Module doesn't return expected value.

```lua
-- broken.lua
local function my_func()
    return "hello"
end
-- Forgot: module.exports = my_func

-- main.lua
local m = require("broken")
print(m)  -- nil (not the function!)
```

**Solution**: Always explicitly set `module.exports`:

```lua
-- correct.lua
local function my_func()
    return "hello"
end

module.exports = my_func

-- main.lua
local m = require("correct")
print(m(""))  -- "hello"
```

---

## Best Practices

### 1. One Module Per File

✓ Good: `user_service.lua` contains user operations  
✗ Bad: Multiple unrelated modules in one file

### 2. Clear Exports

```lua
-- Clear what module provides
module.exports = {
    get_user = function(id) ... end,
    save_user = function(user) ... end,
    delete_user = function(id) ... end,
}
```

### 3. Use Meaningful Names

✓ Good: `require("database_connection")`  
✗ Bad: `require("d_conn")`

### 4. Document Your Modules

```lua
-- A module for user management
-- Usage: local users = require("users")
--        users.create({name = "Alice"})

module.exports = {
    create = function(user_data) ... end,
    delete = function(user_id) ... end,
}
```

### 5. Handle Errors

```lua
local function safe_require(module_name)
    local ok, module = pcall(function()
        return require(module_name)
    end)
    
    if not ok then
        print("Failed to load " .. module_name .. ":", module)
        return nil
    end
    
    return module
end
```

---

## Next Steps

Congratulations! You now understand modules. Here's what to explore next:

1. **Read the API Reference**: [require() API Reference](./require-api.md)
   - Detailed function signatures
   - Error handling patterns
   - Advanced examples

2. **Explore Built-in Modules**: [Built-in Modules Reference](./builtin-modules.md)
   - Complete documentation for each module
   - Real-world examples
   - Best practices

3. **Build Something**: Create a project using modules
   - Organize code with modules
   - Use built-in modules
   - Share your module with others

---

## Common Questions

**Q: Can I have module.exports be different types?**

A: Yes! module.exports can be a function, table, string, number, or any Lua value.

```lua
module.exports = function() end  -- Function
module.exports = {a=1, b=2}      -- Table
module.exports = "string value"  -- String
```

**Q: Are modules cached forever?**

A: Yes, for the lifetime of the script. Cache is cleared when the script exits.

**Q: Can I modify require.cache?**

A: It's not recommended. Cache is managed by the runtime. Use require() to load modules.

**Q: What's the performance impact?**

A: Cached modules are instant (< 1ms). First load is ~50ms. Circular dependencies add <1ms detection overhead.

**Q: How do I reload a module?**

A: Modules are cached for the script's lifetime. To reload, restart the script. In tests, you might clear cache between tests.

---

## Getting Help

- **Documentation**: See [Module System README](./README.md)
- **API Reference**: [require() API](./require-api.md)
- **Built-in Modules**: [Module Reference](./builtin-modules.md)
- **Issues**: Check troubleshooting section above

---

**Last Updated**: October 2025  
**Difficulty**: Beginner  
**Time to Complete**: ~15 minutes

Happy coding! 🚀
