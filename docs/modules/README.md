# Module System Documentation

Welcome to the Hype-RS module system! This documentation covers how to use and work with modules in your Hype-RS projects.

## Table of Contents

- [Overview](#overview)
- [Why Modules Matter](#why-modules-matter)
- [Architecture](#architecture)
- [Key Concepts](#key-concepts)
- [Feature Highlights](#feature-highlights)
- [Quick Start Example](#quick-start-example)
- [Further Reading](#further-reading)

---

## Overview

The **Hype-RS module system** provides a structured way to organize, reuse, and share code. It enables:

- **Code Reusability**: Package functionality into modules and reuse across projects
- **Dependency Management**: Declare and manage module dependencies with `hype.json`
- **Built-in Utilities**: Access 5 core modules for common tasks (fs, path, events, util, table)
- **Module Caching**: Automatic caching prevents redundant loading
- **Clear Interfaces**: Explicit module exports and well-defined APIs

Hype-RS uses a **require-based module system** similar to Node.js, making it familiar to JavaScript developers while remaining idiomatic to Lua.

---

## Why Modules Matter

### Without Modules
```lua
-- Monolithic script
local function readFile(path) ... end
local function processData(data) ... end
local function writeOutput(data) ... end

readFile("input.txt")
processData(result)
writeOutput(result)
```

**Problems:**
- Hard to reuse code across projects
- Difficult to manage dependencies
- No clear separation of concerns
- Naming conflicts between utilities

### With Modules
```lua
-- main.lua
local fileUtils = require("file-utils")
local dataProcessor = require("data-processor")

fileUtils.readFile("input.txt")
dataProcessor.process(result)
```

**Benefits:**
- Reusable components
- Clear interfaces
- Organized code structure
- Manageable dependencies

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                 Hype-RS Script                      │
│                                                     │
│   local fs = require("fs")                         │
└──────────────────┬──────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────┐
│            require() Function                       │
│  • Parses module ID                                │
│  • Checks cache (fast path)                        │
│  • Triggers resolution                             │
└──────────────────┬──────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────┐
│         Module Resolution Algorithm                 │
│                                                     │
│  1. Check require.cache                            │
│  2. Search hype_modules/ directories               │
│  3. Check ~/.hype/modules/                         │
│  4. Check built-in modules                         │
└──────────────────┬──────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────┐
│          Module Loader                              │
│                                                     │
│  • Reads hype.json or index.lua                    │
│  • Executes module code                            │
│  • Caches result                                   │
└──────────────────┬──────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────┐
│         Return module.exports to Script            │
└─────────────────────────────────────────────────────┘
```

---

## Key Concepts

### require()
The main function to load modules. Returns the module's exports.

```lua
local fs = require("fs")              -- Load built-in module
local math = require("math-lib")      -- Load from hype_modules
local util = require("./util")        -- Load relative module
```

### module.exports
How modules define what they expose to other modules.

```lua
-- In util.lua
module.exports = {
    add = function(a, b) return a + b end,
    multiply = function(a, b) return a * b end,
}

-- In main.lua
local util = require("util")
print(util.add(5, 3))  -- 8
```

### require.cache
A table tracking all loaded modules. Useful for introspection and debugging.

```lua
local fs = require("fs")

-- Inspect cache
for module_name, module_obj in pairs(require.cache) do
    print(module_name, module_obj.__id)
end
```

### require.resolve()
Resolves a module ID to its full file path without loading it.

```lua
local path = require.resolve("fs")
print(path)  -- "/path/to/builtin/fs"
```

### __dirname and __filename
Global variables available in every module for file path operations.

```lua
-- In myapp/lib/utils.lua
print(__dirname)   -- "/path/to/myapp/lib"
print(__filename)  -- "/path/to/myapp/lib/utils.lua"
```

### hype.json
Metadata file for modules and projects. Declares module information, dependencies, and executable commands.

```json
{
  "name": "my-app",
  "version": "1.0.0",
  "main": "app.lua",
  "description": "My application",
  "dependencies": {
    "utils-lib": "^1.0.0"
  },
  "bin": {
    "mycommand": "bin/cli.lua"
  }
}
```

The optional `bin` field maps command names to executable scripts. When installed globally with `hype install`, these commands become available system-wide. See [Global Installation](../features/global-install.md) for details.

---

## Feature Highlights

✅ **5 Built-in Modules**
- `fs` - File system operations
- `path` - Path manipulation utilities
- `events` - Event emitter pattern
- `util` - General utilities
- `table` - Table operations

✅ **Module Caching**
- First load: full execution
- Subsequent loads: instant return from cache
- Performance benefit: ~50x faster for cached modules

✅ **Circular Dependency Handling**
- Automatic detection of circular requires
- Clear error messages with dependency chain
- Prevents infinite loops

✅ **Node.js-Compatible Resolution**
- Familiar to JavaScript developers
- Standard hype_modules directory structure
- Compatible with ecosystem patterns

✅ **Cross-Platform Support**
- Works on Windows, macOS, Linux
- Automatic path normalization
- Native path handling

---

## Quick Start Example

### Step 1: Create Your First Module

Create `math-lib.lua`:
```lua
module.exports = {
    add = function(a, b)
        return a + b
    end,
    
    multiply = function(a, b)
        return a * b
    end,
}
```

### Step 2: Use the Module

Create `main.lua`:
```lua
local math = require("math-lib")

print("3 + 4 =", math.add(3, 4))      -- 3 + 4 = 7
print("3 * 4 =", math.multiply(3, 4)) -- 3 * 4 = 12
```

### Step 3: Run It

```bash
$ hype main.lua
3 + 4 = 7
3 * 4 = 12
```

### Step 4: Use Built-in Modules

```lua
-- Use the fs module
local fs = require("fs")

-- Use the path module
local path = require("path")

-- Use events
local events = require("events")

-- Use utilities
local util = require("util")

-- Use table operations
local table_utils = require("table")
```

---

## Global Packages and Executables

Modules can be packaged and installed globally to create system-wide CLI tools.

### Creating Executable Packages

Add a `bin` field to your `hype.json`:

```json
{
  "name": "my-cli-tool",
  "version": "1.0.0",
  "main": "index.lua",
  "bin": {
    "mytool": "bin/cli.lua"
  }
}
```

### Installing Globally

```bash
cd my-cli-tool
hype install
```

This creates `~/.hype/bin/mytool` which can be run from anywhere (after adding to PATH).

### Relationship Between Modules and Packages

- **Modules** are reusable code loaded with `require()`
- **Packages** are directories with `hype.json` containing modules and/or executables
- **Global packages** are installed to `~/.hype/packages/` and can expose CLI commands
- **Executables** (via `bin` field) are standalone scripts that can use modules

**Example Structure:**

```
my-package/
├── hype.json           # Declares name, version, bin commands
├── index.lua           # Main module (loaded with require)
├── bin/                # Executable CLI scripts
│   └── cli.lua
└── lib/                # Internal modules
    └── utils.lua
```

See **[Global Installation Guide](../features/global-install.md)** for complete documentation.

## Further Reading

Explore more about the module system:

1. **[require() API Reference](./require-api.md)**
   - Complete API documentation for require()
   - require.cache and require.resolve()
   - module.exports patterns
   - __dirname and __filename globals

2. **[Built-in Modules Reference](./builtin-modules.md)**
   - Detailed documentation for all 5 built-in modules
   - Function signatures and examples
   - Error handling

3. **[Getting Started Guide](./getting-started.md)**
   - Step-by-step tutorial
   - Creating custom modules
   - Common patterns and best practices
   - Troubleshooting guide

4. **[Global Installation Guide](../features/global-install.md)**
   - Install packages globally
   - Create CLI tools with the bin field
   - Manage installed packages
   - PATH setup for different shells

---

## Architecture Overview

For developers interested in how the module system works internally:

**Resolution Flow:**
1. User calls `require("module-name")`
2. Check `require.cache` for already-loaded module
3. If not cached, use resolver to find module path
4. Read module manifest (hype.json) or source file
5. Execute module code in isolated environment
6. Extract and return `module.exports`
7. Cache result for future loads

**Built-in Modules:**
Built-in modules are provided by the runtime and don't require separate files. They're available immediately without installation.

**Custom Modules:**
Custom modules can be organized in directories with `hype.json` files:

```
my-project/
├── hype.json
├── app.lua
└── hype_modules/
    └── math-lib/
        ├── hype.json
        ├── index.lua
        └── utils.lua
```

---

## Next Steps

Ready to get started? Here's what to do:

1. **Read the Quick Start**: Follow the "Quick Start Example" above
2. **Explore Built-ins**: Check [Built-in Modules Reference](./builtin-modules.md)
3. **Build Something**: Create your first module
4. **Share**: Package it for others to use

For detailed API documentation, see [require() API Reference](./require-api.md).

---

**Last Updated**: October 2025  
**Module System Version**: Phase 4  
**Documentation Status**: Complete and tested

For issues or questions, refer to the [Getting Started Guide](./getting-started.md) troubleshooting section.
