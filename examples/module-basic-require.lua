-- Basic Module Loading
-- Demonstrates core require() functionality with the fs module
-- Shows module loading, caching, and introspection
--
-- Run with:
--   hype --module examples/module-basic-require.lua
-- or:
--   hype -m examples/module-basic-require.lua

print("═══════════════════════════════════════════════════════")
print("  Basic Module Loading Example")
print("═══════════════════════════════════════════════════════\n")

-- Example 1: Loading a built-in module
print("--- Example 1: Loading the fs module ---")
print("Executing: local fs = require('fs')")

local fs = require("fs")

print("✓ Successfully loaded fs module")
print("✓ Received module object from require()\n")

-- Example 2: Inspecting module structure
print("--- Example 2: Inspecting module structure ---")
print("Module type:", type(fs))

if type(fs) == "table" then
    print("✓ Module is a Lua table\n")
    
    print("Module contents:")
    local function print_table_contents(tbl, indent)
        indent = indent or ""
        for key, value in pairs(tbl) do
            if type(value) == "table" and value.__fn then
                print(indent .. "  • " .. key .. " (function)")
            elseif type(value) == "table" and value.__value then
                print(indent .. "  • " .. key .. " = " .. tostring(value.__value))
            elseif key:sub(1, 2) ~= "__" then
                print(indent .. "  • " .. key .. " (" .. type(value) .. ")")
            end
        end
    end
    
    print_table_contents(fs)
    print()
else
    print("✗ Unexpected module type\n")
end

-- Example 3: Checking module identity
print("--- Example 3: Module identity and metadata ---")
if fs.__id then
    print("Module ID:", fs.__id)
    print("✓ Module has __id metadata")
else
    print("(Module metadata not available)")
end
print()

-- Example 4: Module loading and caching
print("--- Example 4: Module caching ---")
print("Executing: local fs2 = require('fs')")

local fs2 = require("fs")

if fs == fs2 then
    print("✓ Both references point to the SAME module")
    print("✓ Module is cached and reused")
    print("✓ No duplicate loading")
else
    print("✗ Module caching issue detected")
end
print()

-- Example 5: Loading multiple modules
print("--- Example 5: Loading multiple built-in modules ---")
print("Available built-in modules:")
print("  • fs (file system operations)")
print("  • path (path manipulation)")
print("  • events (event emitter)")
print("  • util (utility functions)")
print("  • table (table operations)")
print()

print("Loading path module...")
local path = require("path")
print("✓ Successfully loaded path module\n")

print("Loading events module...")
local events = require("events")
print("✓ Successfully loaded events module\n")

-- Example 6: Module objects are independent
print("--- Example 6: Module independence ---")
print("fs and path are different modules:", fs ~= path)
print("fs and events are different modules:", fs ~= events)
print()

-- Example 7: Typical module usage pattern
print("--- Example 7: Typical module usage pattern ---")
print([[
Standard pattern for using modules:

  local module = require("module-name")
  
  -- Check if module was loaded
  if module then
    print("Module loaded successfully")
    
    -- Access module contents
    if module.someFunction then
      -- Call function (once implemented)
      -- module.someFunction(...)
    end
    
    -- Access module metadata
    if module.__id then
      print("Module ID:", module.__id)
    end
  end

Module names available:
  require("fs")
  require("path")
  require("events")
  require("util")
  require("table")
]])
print()

-- Example 8: Error handling for non-existent modules
print("--- Example 8: Error handling ---")
print("Attempting to load non-existent module...")

local ok, err = pcall(function()
    local nonexistent = require("nonexistent-module-xyz")
end)

if not ok then
    print("✓ Error caught as expected")
    print("Error message:", err:match("Unknown built-in module") and "Module not found" or err:sub(1, 50) .. "...")
else
    print("✗ Should have raised an error for missing module")
end
print()

print("═══════════════════════════════════════════════════════")
print("  Example completed successfully!")
print("═══════════════════════════════════════════════════════\n")
