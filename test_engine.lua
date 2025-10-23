-- Test script for the execution engine
print("Hello from hype-rs execution engine!")

-- Test arguments
if #args > 0 then
    print("Arguments received:")
    for i, arg in ipairs(args) do
        print("  " .. i .. ": " .. arg)
    end
else
    print("No arguments provided")
end

-- Test hype globals
print("hype version: " .. (hype.version or "unknown"))
print("Script path: " .. (SCRIPT_PATH or "unknown"))
print("Script name: " .. (SCRIPT_NAME or "unknown"))

-- Test some basic Lua operations
local result = 2 + 2
print("2 + 2 = " .. result)

local table_test = {a = 1, b = 2, c = 3}
print("Table test: " .. table_test.a + table_test.b + table_test.c)

print("Test completed successfully!")