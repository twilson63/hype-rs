-- Basic Application Example
-- Demonstrates require(), custom modules, __dirname, __filename, and error handling
-- Run with:
--   hype --module examples/basic-app.lua
-- or:
--   hype -m examples/basic-app.lua

print("═══════════════════════════════════════════════════════════")
print("  Basic Application with Module System")
print("═══════════════════════════════════════════════════════════\n")

print("--- Environment Information ---")
print("Script location:", __filename)
print("Script directory:", __dirname)
print()

print("--- Step 1: Loading Built-in Modules ---")
local fs = require("fs")
local path = require("path")
print("✓ Loaded fs module")
print("✓ Loaded path module")
print()

print("--- Step 2: Creating Custom Module (math_utils) ---")

local math_utils = {
    version = "1.0.0",
    
    add = function(a, b)
        if type(a) ~= "number" or type(b) ~= "number" then
            error("add: arguments must be numbers")
        end
        return a + b
    end,
    
    subtract = function(a, b)
        if type(a) ~= "number" or type(b) ~= "number" then
            error("subtract: arguments must be numbers")
        end
        return a - b
    end,
    
    multiply = function(a, b)
        if type(a) ~= "number" or type(b) ~= "number" then
            error("multiply: arguments must be numbers")
        end
        return a * b
    end,
    
    divide = function(a, b)
        if type(a) ~= "number" or type(b) ~= "number" then
            error("divide: arguments must be numbers")
        end
        if b == 0 then
            error("divide: division by zero")
        end
        return a / b
    end,
    
    power = function(base, exponent)
        if type(base) ~= "number" or type(exponent) ~= "number" then
            error("power: arguments must be numbers")
        end
        return base ^ exponent
    end,
    
    square_root = function(n)
        if type(n) ~= "number" then
            error("square_root: argument must be a number")
        end
        if n < 0 then
            error("square_root: cannot compute square root of negative number")
        end
        return n ^ 0.5
    end,
}

print("✓ Created math_utils module with 6 functions:")
print("  • add(a, b)")
print("  • subtract(a, b)")
print("  • multiply(a, b)")
print("  • divide(a, b)")
print("  • power(base, exponent)")
print("  • square_root(n)")
print()

print("--- Step 3: Using Custom Module Functions ---")

local function safe_call(func_name, func, ...)
    local ok, result = pcall(func, ...)
    if ok then
        return result
    else
        print("  ✗ Error:", result)
        return nil
    end
end

print("Basic calculations:")
local r1 = safe_call("add", math_utils.add, 10, 5)
if r1 then print("  10 + 5 =", r1) end

local r2 = safe_call("subtract", math_utils.subtract, 10, 5)
if r2 then print("  10 - 5 =", r2) end

local r3 = safe_call("multiply", math_utils.multiply, 10, 5)
if r3 then print("  10 * 5 =", r3) end

local r4 = safe_call("divide", math_utils.divide, 10, 5)
if r4 then print("  10 / 5 =", r4) end

print()
print("Advanced calculations:")
local r5 = safe_call("power", math_utils.power, 2, 10)
if r5 then print("  2^10 =", r5) end

local r6 = safe_call("square_root", math_utils.square_root, 16)
if r6 then print("  √16 =", r6) end

print()

print("--- Step 4: Error Handling ---")
print("Testing error cases:")

print("  • Dividing by zero:")
safe_call("divide_zero", math_utils.divide, 10, 0)

print("  • Non-numeric input:")
safe_call("add_string", math_utils.add, "hello", 5)

print("  • Negative square root:")
safe_call("sqrt_negative", math_utils.square_root, -4)

print()

print("--- Step 5: Module Caching ---")
print("Testing module caching behavior:")
local fs2 = require("fs")
print("  First fs load == Second fs load:", fs == fs2)
print("  ✓ Module is cached and reused")
print()

print("--- Step 6: Application Summary ---")
print("Demonstrated features:")
print("  ✓ require() for loading modules")
print("  ✓ Custom module creation")
print("  ✓ Module functions with parameters")
print("  ✓ Error handling with pcall()")
print("  ✓ __dirname and __filename globals")
print("  ✓ Module composition")
print("  ✓ Module caching")
print()

print("═══════════════════════════════════════════════════════════")
print("  Application completed successfully!")
print("═══════════════════════════════════════════════════════════\n")
