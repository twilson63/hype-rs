-- Path Utilities Module
-- Demonstrates path module structure and expected functionality
--
-- Run with:
--   hype --module examples/module-path-operations.lua
-- or:
--   hype -m examples/module-path-operations.lua

print("═══════════════════════════════════════════════════════")
print("  Path Operations Example")
print("═══════════════════════════════════════════════════════\n")

-- Load the path module
local path = require("path")

print("✓ Successfully loaded path module")
print("✓ Path module available via require()\n")

-- Example 1: Available path functions
print("--- Example 1: Path module API ---")
print("Available functions in path module:")

local function inspect_module(module)
    local functions = {}
    local values = {}
    
    for key, item in pairs(module) do
        if type(item) == "table" then
            if item.__fn then
                table.insert(functions, key)
            elseif item.__value then
                table.insert(values, key)
            end
        end
    end
    
    for _, fname in ipairs(functions) do
        print("  • " .. fname .. "()")
    end
    for _, vname in ipairs(values) do
        print("  • " .. vname .. " = constant")
    end
end

inspect_module(path)
print()

-- Example 2: Path module design
print("--- Example 2: Path module design ---")
print([[
The path module provides utilities for working with
file and directory paths. It includes:

Functions:
  path.join(...)        - Combine path segments
  path.dirname(path)    - Get directory name
  path.basename(path)   - Get base filename
  path.extname(path)    - Get file extension
  path.resolve(...)     - Resolve absolute path
  path.relative(from, to) - Compute relative path
  path.normalize(path)  - Normalize path string

Constants:
  path.sep              - Path separator ('/' or '\')

Once implemented, you can use them like:
  local dir = path.dirname("src/main.lua")
  -- Returns: "src"
]])
print()

-- Example 3: Path manipulation examples (conceptual)
print("--- Example 3: Conceptual usage examples ---")
print([[
Example usages once functions are implemented:

Building project paths:
  local src = path.join(root, "src")
  local main = path.join(src, "main.lua")

Extracting path components:
  local dir = path.dirname("src/main.lua")      → "src"
  local base = path.basename("src/main.lua")    → "main.lua"
  local ext = path.extname("main.lua")          → ".lua"

Path normalization:
  path.normalize("a//b/./c")  → "a/b/c"
  path.normalize("a/b/../c")  → "a/c"

Relative paths:
  path.relative("a/b", "a/b/c")  → "c"
  path.relative("a/b", "a/c")    → "../c"
]])
print()

-- Example 4: Path separator awareness
print("--- Example 4: Cross-platform path handling ---")
if path.sep then
    print("Current system path separator:", path.sep)
else
    print("(path.sep not yet implemented)")
end

print([[
The path module is designed to work correctly across:
  ✓ Unix/Linux/macOS (uses '/' separator)
  ✓ Windows (uses '\\' separator)

This allows code to work on all platforms without
manual string manipulation.
]])
print()

-- Example 5: Module loading verification
print("--- Example 5: Verifying module loaded ---")
print("Path module status:")
print("  Type:", type(path))
print("  Has __id:", path.__id ~= nil)
print("  Number of items:", (function()
    local count = 0
    for _ in pairs(path) do count = count + 1 end
    return count
end)())
print()

-- Example 6: Integration with other modules
print("--- Example 6: Using path with other modules ---")
print([[
The path module is often used together with fs:

  local fs = require("fs")
  local path = require("path")
  
  -- Build a file path
  local filepath = path.join("data", "users.csv")
  
  -- Check if it exists
  if fs.existsSync(filepath) then
    -- Read the file
    local content = fs.readFileSync(filepath)
  end

This combination is very common in file-based
applications and scripts.
]])
print()

-- Example 7: Path module in project structure
print("--- Example 7: Project structure patterns ---")
print([[
Project using path module:

  my_app/
  ├── main.lua
  ├── config/
  │   └── settings.lua
  ├── src/
  │   ├── utils.lua
  │   └── engine.lua
  └── data/
      ├── input.txt
      └── output.txt

In code:
  local path = require("path")
  
  local data_dir = path.join(__dirname, "data")
  local input = path.join(data_dir, "input.txt")
  local output = path.join(data_dir, "output.txt")
]])
print()

print("═══════════════════════════════════════════════════════")
print("  Example completed successfully!")
print("═══════════════════════════════════════════════════════\n")
