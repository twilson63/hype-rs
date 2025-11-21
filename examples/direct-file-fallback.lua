-- Example demonstrating direct file/directory fallback in module resolution
-- This shows how require() now works with files/directories outside hype_modules/

-- Direct file fallback: require a .lua file directly from the current directory
local utils = require("lib/utils")
print("Utils module loaded:", utils.version)
print("Helper function result:", utils.add(5, 3))

-- Another direct file example
local helpers = require("lib/helpers")
print("Helpers loaded:", helpers.greeting)
