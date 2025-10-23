-- Module with Dependencies
-- Demonstrates using multiple built-in modules together in a realistic scenario
--
-- Run with:
--   hype --module examples/module-dependencies.lua
-- or:
--   hype -m examples/module-dependencies.lua

print("═══════════════════════════════════════════════════════")
print("  Module Dependencies Example")
print("═══════════════════════════════════════════════════════\n")

-- Load multiple built-in modules
local fs = require("fs")
local path = require("path")
local events = require("events")

print("✓ Successfully loaded fs module")
print("✓ Successfully loaded path module")
print("✓ Successfully loaded events module\n")

-- Example 1: Combining fs and path modules
print("--- Example 1: File system operations with path utilities ---\n")

local function build_file_path(dir, name, ext)
    return path.join(dir, name .. "." .. ext)
end

local file_paths = {
    build_file_path("config", "app", "json"),
    build_file_path("src", "main", "lua"),
    build_file_path("data", "users", "csv")
}

print("Generated file paths:")
for _, file_path in ipairs(file_paths) do
    local dir = path.dirname(file_path)
    local base = path.basename(file_path)
    local ext = path.extname(file_path)
    
    print(string.format(
        "  Path: %s\n    Dir: %s, Base: %s, Ext: %s",
        file_path, dir, base, ext
    ))
end
print()

-- Example 2: Event emitter usage
print("--- Example 2: Using EventEmitter from events module ---\n")

-- Create an event emitter
local emitter = events.EventEmitter:new()

print("✓ Created new EventEmitter instance\n")

-- Register event handlers
local event_count = 0

emitter:on("file_processed", function(filename)
    event_count = event_count + 1
    print("  EVENT: File processed - " .. filename .. " [#" .. event_count .. "]")
end)

emitter:on("error", function(error_msg)
    print("  EVENT: Error occurred - " .. error_msg)
end)

emitter:on("complete", function()
    print("  EVENT: Processing complete!")
end)

print("Registered event handlers:")
print("  • file_processed")
print("  • error")
print("  • complete")
print()

-- Example 3: Combining modules in a file processor scenario
print("--- Example 3: File processor combining multiple modules ---\n")

local function process_files()
    local files_to_process = {
        "examples/module-basic-require.lua",
        "examples/module-path-operations.lua",
        "examples/module-custom-module.lua"
    }
    
    print("Processing files:")
    
    for i, filepath in ipairs(files_to_process) do
        -- Check if file exists using fs module
        if fs.existsSync(filepath) then
            -- Extract information using path module
            local dir = path.dirname(filepath)
            local base = path.basename(filepath)
            local ext = path.extname(filepath)
            
            -- Get file stats
            local ok, stat = pcall(function()
                return fs.statSync(filepath)
            end)
            
            print(string.format("  [%d] %s", i, base))
            print(string.format("      Location: %s", dir))
            print(string.format("      Extension: %s", ext))
            
            if ok and stat then
                print(string.format("      Size: %s bytes", stat.size or "unknown"))
            end
            
            -- Emit file processed event
            emitter:emit("file_processed", base)
        else
            -- Emit error event
            emitter:emit("error", "File not found: " .. filepath)
        end
    end
    
    emitter:emit("complete")
end

process_files()
print()

-- Example 4: Module require.cache introspection
print("--- Example 4: Inspecting require.cache ---\n")

print("Loaded modules (from require.cache):")
print("  • fs module - ID: " .. (fs.__id or "unknown"))
print("  • path module - ID: " .. (path.__id or "unknown"))
print("  • events module - ID: " .. (events.__id or "unknown"))
print()

-- Example 5: Error handling with dependencies
print("--- Example 5: Error handling patterns ---\n")

local function safe_read_file(filepath)
    if not fs.existsSync(filepath) then
        return nil, "File does not exist: " .. filepath
    end
    
    local ok, content = pcall(function()
        return fs.readFileSync(filepath)
    end)
    
    if not ok then
        return nil, "Failed to read file: " .. tostring(content)
    end
    
    return content, nil
end

print("Testing safe file read:")
local content, err = safe_read_file("examples/module-dependencies.lua")
if content then
    print("  ✓ Successfully read file")
    print("  Content length:", #content, "bytes")
else
    print("  ✗ Error:", err)
end
print()

-- Example 6: Building project paths with modules
print("--- Example 6: Project structure utilities ---\n")

local function create_project_paths(root)
    return {
        root = root,
        src = path.join(root, "src"),
        lib = path.join(root, "lib"),
        tests = path.join(root, "tests"),
        config = path.join(root, "config"),
        build = path.join(root, "build"),
        
        -- Helper to get main file
        main_file = function(self)
            return path.join(self.src, "main.lua")
        end,
        
        -- Helper to get test file
        test_file = function(self, name)
            return path.join(self.tests, name .. "_test.lua")
        end
    }
end

local project = create_project_paths("my_project")
print("Project structure for: my_project")
print("  src/ at:", project.src)
print("  lib/ at:", project.lib)
print("  tests/ at:", project.tests)
print("  config/ at:", project.config)
print("  Main file:", project:main_file())
print("  Test file:", project:test_file("utils"))
print()

-- Example 7: Module dependency chain visualization
print("--- Example 7: Module dependency information ---\n")

print([[
Dependency Chain (what this script loaded):

  module-dependencies.lua
    ├── fs (filesystem module)
    │   └── Internal file I/O functions
    ├── path (path utilities module)
    │   └── Path manipulation functions
    └── events (event emitter module)
        └── EventEmitter class and methods

Key points:
  ✓ Each module is loaded once (cached)
  ✓ Modules can be used together
  ✓ No circular dependency issues
  ✓ Clean module.exports interface
  ✓ All modules available globally via require()
]])
print()

print("═══════════════════════════════════════════════════════")
print("  Example completed successfully!")
print("  Total events emitted:", event_count)
print("═══════════════════════════════════════════════════════\n")
